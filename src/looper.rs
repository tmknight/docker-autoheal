use bollard::container::RestartContainerOptions;
use bollard::Docker;
use std::time::Duration;

use crate::action::logging::log_message;
use crate::check::inspect::inspect_container;
use crate::check::list::containers_list;

pub async fn start_loop(
    autoheal_interval: u64,
    autoheal_container_label: String,
    autoheal_stop_timeout: isize,
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
            // Execute concurrently
            let docker_clone = docker.clone();
            let handle = tokio::task::spawn(async move {
                // Get name of container
                let name_tmp = match &container.names {
                    Some(names) => &names[0],
                    None => {
                        let msg0 =
                            String::from("[ERROR]   Could not reliably determine container name");
                        log_message(&msg0).await;
                        ""
                    }
                };
                let name = name_tmp.trim_matches('/').trim();

                // Get id of container
                let id: String = match container.id {
                    Some(id) => id.chars().take(12).collect(),
                    None => {
                        let msg0 =
                            String::from("[ERROR]   Could not reliably determine container id");
                        log_message(&msg0).await;
                        "".to_string()
                    }
                };

                // Have all tests passed for unhealthy container to be remediated
                if name.is_empty() && id.is_empty() {
                    let msg0 = format!(
                        "[ERROR]   Could not reliably identify the container: name={}, id={}",
                        name, id
                    );
                    log_message(&msg0).await;
                } else {
                    // Determine failing streak of the unhealthy container
                    let inspection = inspect_container(docker_clone.clone(), name, &id).await;
                    if inspection.failed {
                        // Report unhealthy container
                        let msg0 = format!(
                            "[WARNING] [{}] Container ({}) is unhealthy with {} failures",
                            name, id, inspection.failing_streak
                        );
                        log_message(&msg0).await;

                        // Build restart options
                        let restart_options = Some(RestartContainerOptions {
                            t: autoheal_stop_timeout,
                        });

                        // Report container restart
                        let msg1 = format!(
                            "[WARNING] [{}] Restarting container ({}) with {}s timeout",
                            name, id, autoheal_stop_timeout
                        );
                        log_message(&msg1).await;

                        // Restart unhealthy container
                        match &docker_clone.restart_container(&id, restart_options).await {
                            Ok(()) => {
                                let msg0 = format!(
                                    "[INFO]    [{}] Restart of container ({}) was successful",
                                    name, id
                                );
                                log_message(&msg0).await;
                            }
                            Err(e) => {
                                let msg0 = format!(
                                    "[ERROR]   [{}] Restart of container ({}) failed: {}",
                                    name, id, e
                                );
                                log_message(&msg0).await;
                            }
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
