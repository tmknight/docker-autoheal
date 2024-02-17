use crate::{
    execute::postaction::execute_action,
    inquire::{inspect::inspect_container, list::containers_list},
    report::{logging::log_message, webhook::notify_webhook},
    LoopVariablesList, ERROR, INFO, WARNING,
};
use bollard::{container::RestartContainerOptions, Docker};
use std::time::Duration;

pub async fn start_loop(
    var: LoopVariablesList,
    docker: Docker,
) -> Result<(), Box<dyn std::error::Error>> {
    // Establish loop interval
    let mut interval = tokio::time::interval(Duration::from_secs(var.interval));
    loop {
        // Gather all unhealthy containers
        let containers = containers_list(&var.container_label, docker.clone()).await;
        // Prepare for concurrent execution
        let mut handles = vec![];
        // Iterate through suspected unhealthy
        for container in containers {
            // Prepare reusable objects
            let docker_clone = docker.clone();
            let apprise_url = var.apprise_url.clone();
            let webhook_key = var.webhook_key.clone();
            let webhook_url = var.webhook_url.clone();
            let post_action = var.post_action.clone();

            // Determine if stop override label
            let s = "autoheal.stop.timeout".to_string();
            let autoheal_stop_timeout = match container.labels {
                Some(ref label) => match label.get(&s) {
                    Some(v) => v.parse().unwrap_or(var.stop_timeout),
                    None => var.stop_timeout,
                },
                None => var.stop_timeout,
            };

            // Determine if excluded
            let s = "autoheal.restart.exclude".to_string();
            let autoheal_restart_exclude = match container.labels {
                Some(ref label) => match label.get(&s) {
                    Some(v) => v.parse().unwrap_or(false),
                    None => false,
                },
                None => false,
            };

            // Execute concurrently
            let handle = tokio::task::spawn(async move {
                // Get name of container
                let name_tmp = match &container.names {
                    Some(names) => &names[0],
                    None => {
                        let msg0 = String::from("Could not reliably determine container name");
                        log_message(&msg0, ERROR).await;
                        ""
                    }
                };
                let name = name_tmp.trim_matches('/').trim();

                // Get id of container
                let id = match container.id {
                    Some(id) => id.chars().take(12).collect(),
                    None => {
                        let msg0 = String::from("Could not reliably determine container id");
                        log_message(&msg0, ERROR).await;
                        "".to_string()
                    }
                };

                // Have all tests passed for unhealthy container to be remediated
                if name.is_empty() && id.is_empty() {
                    let msg0 = format!(
                        "Could not reliably identify the container: name={}, id={}",
                        name, id
                    );
                    log_message(&msg0, ERROR).await;
                } else if autoheal_restart_exclude {
                    let msg0 = format!(
                        "[{}] Container ({}) is unhealthy, however is labeled for restart exclusion",
                        name, id
                    );
                    log_message(&msg0, WARNING).await;
                } else {
                    // Determine failing streak of the unhealthy container
                    let inspection = inspect_container(docker_clone.clone(), name, &id).await;
                    if inspection.failed {
                        // Report unhealthy container
                        let msg0 = format!(
                            "[{}] Container ({}) is unhealthy with {} failures",
                            name, id, inspection.failing_streak
                        );
                        log_message(&msg0, WARNING).await;

                        // Build restart options
                        let restart_options = Some(RestartContainerOptions {
                            t: autoheal_stop_timeout,
                        });

                        // Report container restart
                        let msg1 = format!(
                            "[{}] Restarting container ({}) with {}s timeout",
                            name, id, autoheal_stop_timeout
                        );
                        log_message(&msg1, WARNING).await;

                        // Restart unhealthy container
                        let msg = match &docker_clone.restart_container(&id, restart_options).await
                        {
                            Ok(()) => {
                                // Log result
                                let msg0 = format!(
                                    "[{}] Restart of container ({}) was successful",
                                    name, id
                                );
                                log_message(&msg0, INFO).await;
                                msg0
                            }
                            Err(e) => {
                                // Log result
                                let msg0 = format!(
                                    "[{}] Restart of container ({}) failed: {}",
                                    name, id, e
                                );
                                log_message(&msg0, ERROR).await;
                                msg0
                            }
                        };

                        // Send webhook
                        if !(webhook_url.is_empty() && webhook_key.is_empty()) {
                            let payload = format!("{{\"{}\":\"{}\"}}", &webhook_key, &msg);
                            notify_webhook(&webhook_url, &payload).await;
                        }
                        // Send apprise
                        if !apprise_url.is_empty() {
                            let payload =
                                format!("{{\"title\":\"Docker-Autoheal\",\"body\":\"{}\"}}", &msg);
                            notify_webhook(&apprise_url, &payload).await;
                        }
                        // Execute post-action
                        if !post_action.is_empty() {
                            execute_action(
                                post_action,
                                name,
                                id,
                                autoheal_stop_timeout.to_string(),
                            )
                            .await;
                        }
                    }
                }
            });
            // Push handles for later consumption
            handles.push(handle);
        }
        // Return JoinHandle results as they arrive
        for join in handles {
            join.await?;
        }
        // Loop interval
        interval.tick().await;
    }
}
