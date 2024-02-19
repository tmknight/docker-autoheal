use crate::{
    execute::action::execute_tasks,
    inquire::{
        inspect::{self, inspect_container},
        list::containers_list,
    },
    report::logging::log_message,
    LoopVariablesList, ERROR, WARNING,
};
use bollard::Docker;
use std::time::Duration;

pub struct TaskVariablesList {
    pub docker: Docker,
    pub name: String,
    pub id: String,
    pub inspection: inspect::Result,
    pub autoheal_stop_timeout: isize,
    pub apprise_url: String,
    pub webhook_key: String,
    pub webhook_url: String,
    pub post_action: String,
    pub autoheal_restart_enable: bool,
    pub log_all: bool,
}

pub async fn start_loop(
    var: LoopVariablesList,
    docker: Docker,
) -> Result<(), Box<dyn std::error::Error>> {
    // Establish loop interval
    let mut interval = tokio::time::interval(Duration::from_secs(var.interval));
    loop {
        // Gather all unhealthy containers
        let containers = containers_list(docker.clone()).await;
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
            let log_all = var.log_all;
            let monitor_all = var.monitor_all;

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
            let s = "autoheal.monitor.enable".to_string();
            let autoheal_monitor_enable = match container.labels {
                Some(ref label) => match label.get(&s) {
                    Some(v) => v.parse().unwrap_or(monitor_all),
                    None => monitor_all,
                },
                None => monitor_all,
            };
            let s = "autoheal.restart.enable".to_string();
            let autoheal_restart_enable = match container.labels {
                Some(ref label) => match label.get(&s) {
                    Some(v) => v.parse().unwrap_or(true),
                    None => true,
                },
                None => true,
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
                } else if !autoheal_restart_enable {
                    if log_all {
                        let msg0 = format!(
                        "[{}] Container ({}) is unhealthy, however restart is disabled on request",
                        name, id
                    );
                        log_message(&msg0, WARNING).await;
                    };
                } else if autoheal_monitor_enable {
                    // Determine failing streak of the unhealthy container
                    let inspection = inspect_container(docker_clone.clone(), name, &id).await;
                    if inspection.failed {
                        // Remediate
                        let task_variables = {
                            TaskVariablesList {
                                docker: docker_clone,
                                name: name.to_string(),
                                id,
                                inspection,
                                autoheal_stop_timeout,
                                apprise_url,
                                webhook_key,
                                webhook_url,
                                post_action,
                                autoheal_restart_enable,
                                log_all,
                            }
                        };
                        execute_tasks(task_variables).await
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
