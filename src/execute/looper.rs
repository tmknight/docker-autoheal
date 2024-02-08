use crate::{
    inquire::inspect::inspect_container, inquire::list::containers_list,
    report::logging::log_message, report::webhook::notify_webhook, ERROR, INFO, WARNING,
};
use bollard::{container::RestartContainerOptions, Docker};
use std::time::Duration;

pub async fn start_loop(
    autoheal_interval: u64,
    autoheal_container_label: String,
    autoheal_stop_timeout: isize,
    autoheal_apprise_url: String,
    autoheal_webhook_key: String,
    autoheal_webhook_url: String,
    docker: Docker,
) -> Result<(), Box<dyn std::error::Error>> {
    // Establish loop interval
    let mut interval = tokio::time::interval(Duration::from_secs(autoheal_interval));
    loop {
        // Gather all unhealthy containers
        let containers = containers_list(&autoheal_container_label, docker.clone()).await;
        // Prepare for concurrent execution
        let mut handles = vec![];
        // Iterate through suspected unhealthy
        for container in containers {
            // Prepare reusable objects
            let docker_clone = docker.clone();
            let apprise_url = autoheal_apprise_url.clone();
            let webhook_key = autoheal_webhook_key.clone();
            let webhook_url = autoheal_webhook_url.clone();

            // Determine if stop override label
            let s = "autoheal.stop.timeout".to_string();
            let autoheal_stop_timeout: isize = match container.labels {
                Some(label) => match label.get(&s) {
                    Some(v) => v.parse().unwrap_or(autoheal_stop_timeout),
                    None => autoheal_stop_timeout,
                },
                None => autoheal_stop_timeout,
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
                let id: String = match container.id {
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
                    }
                }
            });
            // Push handles for later consumption
            handles.push(handle);
        }
        // Return joinhandle results as they arrive
        for join in handles {
            join.await?;
        }
        // Loop interval
        interval.tick().await;
    }
}
