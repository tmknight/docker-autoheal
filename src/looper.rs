use bollard::container::{ListContainersOptions, RestartContainerOptions};
use bollard::Docker;
use std::collections::HashMap;
use std::time::Duration;

use crate::logging::log_message;

pub async fn start_loop(
    autoheal_interval: u64,
    autoheal_container_label: String,
    autoheal_stop_timeout: isize,
    docker: Docker,
) -> Result<(), Box<dyn std::error::Error>> {
    // Establish loop interval
    let mut interval = tokio::time::interval(Duration::from_secs(autoheal_interval));
    loop {
        // Build container assessment criteria
        let mut filters = HashMap::new();
        filters.insert("health", vec!["unhealthy"]);
        filters.insert("status", vec!["running", "exited", "dead"]);
        if autoheal_container_label != "all" {
            filters.insert("label", vec![&autoheal_container_label]);
        }

        // Gather all containers that are unhealthy
        let container_options = Some(ListContainersOptions {
            all: true,
            filters,
            ..Default::default()
        });
        let mut handles = vec![];
        let containers = docker.list_containers(container_options).await?;
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

                // Determine failing streak of the unhealthy container
                // Attempt to inspect the container
                let container_inspect = match docker_clone.inspect_container(&id, None).await {
                    Ok(container) => container,
                    Err(_) => {
                        // Log that we had an error
                        let msg0 = String::from(
                            "[ERROR]   Could not reliably determine container information from inspection",
                        );
                        log_message(&msg0).await;
                        // Return container default if err so we can carry on
                        Default::default()
                    }
                };
                // Get failing streak from state:health
                let failing_streak = match container_inspect
                    .state
                    .as_ref()
                    .and_then(|s| s.health.as_ref().and_then(|h| h.failing_streak))
                {
                    Some(streak) => streak,
                    None => {
                        // Log that we had an error
                        let msg0 = String::from(
                            "[ERROR]   Could not reliably determine container failing streak; default to 0",
                        );
                        log_message(&msg0).await;
                        // If health information is not available, set failing_streak to 0
                        let x: i64 = 0;
                        x
                    }
                };
                // Failing streak not 0 should be considered for remediation
                let failing = match failing_streak {
                    0 => false,
                    _ => true,
                };

                // Have all tests passed for unhealthy container to be remediated
                if !(name.is_empty() && id.is_empty() && failing) {
                    // Report unhealthy container
                    let msg0 = format!("[WARNING] [{}] Container ({}) unhealthy", name, id);
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
                    let rslt = docker_clone.restart_container(&id, restart_options).await;
                    match rslt {
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
                } else {
                    let msg0 = format!(
                        "[ERROR]   Could not reliably identify the container and/or state: name={}, id={}, failing_streak={}",
                        name, id, failing_streak
                    );
                    log_message(&msg0).await;
                }
            });
            // Push handles for latter consumption
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
