use crate::{
    execute::action::execute_tasks,
    inquire::{
        inspect::{self, inspect_container},
        list::containers_list,
    },
    report::{
        logging::log_message,
        record::{read_record, write_record, JsonRecord},
    },
    LoopVariablesList, ERROR, INFO, WARNING,
};
use bollard::Docker;
use std::time::Duration;

pub struct TaskVariablesList {
    pub hostname: String,
    pub docker: Docker,
    pub name: String,
    pub id: String,
    pub inspection: inspect::Result,
    pub stop_timeout: isize,
    pub apprise_url: String,
    pub webhook_key: String,
    pub webhook_url: String,
    pub post_action: String,
    pub restart_enable: bool,
}

pub async fn start_loop(
    var: LoopVariablesList,
    docker: Docker,
) -> Result<(), Box<dyn std::error::Error>> {
    // Get System Information
    let sys_info = docker.info().await;
    let hostname = sys_info.unwrap().name.unwrap_or("unknown".to_string());

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
            let hostname_clone = hostname.clone();
            let docker_clone = docker.clone();
            let apprise_url = var.apprise_url.clone();
            let webhook_key = var.webhook_key.clone();
            let webhook_url = var.webhook_url.clone();
            let post_action = var.post_action.clone();
            let log_all = var.log_all;
            let monitor_all = var.monitor_all;
            let log_ready = var.log_ready;
            let mut msg: String = "".to_string();
            let mut fail_reason: String = "".to_string();

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
                        msg = String::from("Could not reliably determine container name");
                        log_message(&msg, ERROR).await;
                        ""
                    }
                };
                let name = name_tmp.trim_matches('/').trim();

                // Get id of container
                let id = match container.id {
                    Some(id) => id.chars().take(12).collect(),
                    None => {
                        msg = String::from("Could not reliably determine container id");
                        log_message(&msg, ERROR).await;
                        "".to_string()
                    }
                };

                // Have all tests passed for unhealthy container to be remediated
                if name.is_empty() && id.is_empty() {
                    msg = format!(
                        "Could not reliably identify the container: name={}, id={}",
                        name, id
                    );
                    log_message(&msg, ERROR).await;
                } else if !autoheal_restart_enable && log_all {
                    msg = format!(
                        "[{}] Container ({}) is unhealthy, however restart is disabled on request",
                        name, id
                    );
                    log_message(&msg, WARNING).await;
                } else if autoheal_monitor_enable && (autoheal_restart_enable || log_all) {
                    // Determine failing streak of the unhealthy container
                    let inspection = inspect_container(docker_clone.clone(), name, &id).await;
                    fail_reason = inspection.failing_reason.clone();
                    if inspection.failed {
                        // Remediate
                        let task_variables = {
                            TaskVariablesList {
                                hostname: hostname_clone,
                                docker: docker_clone,
                                name: name.to_string(),
                                id: id.clone(),
                                inspection,
                                stop_timeout: autoheal_stop_timeout,
                                apprise_url,
                                webhook_key,
                                webhook_url,
                                post_action,
                                restart_enable: autoheal_restart_enable,
                            }
                        };
                        msg = execute_tasks(task_variables).await
                    }
                }

                if log_ready && !(msg.is_empty() && fail_reason.is_empty()) {
                    // Write to log.json
                    let data: JsonRecord = {
                        JsonRecord {
                            date: chrono::Local::now()
                                .format("%Y-%m-%d %H:%M:%S%z")
                                .to_string(),
                            name: name.to_string(),
                            id: id.clone(),
                            err: fail_reason,
                            action: msg,
                        }
                    };
                    write_record(data).await.ok();
                    // Read from log.json
                    match read_record().await {
                        Ok(records) => {
                            // Get unhealthy count for container
                            let action_count = records.into_iter().filter(|r| r.id == id).count();
                            // Report results
                            let mut noun = "time";
                            if action_count > 1 {
                                noun = "times"
                            }
                            msg = format!(
                                "[{}] Container ({}) has been unhealthy {} {}",
                                name, id, action_count, noun
                            );
                            log_message(&msg, INFO).await;
                        }
                        Err(_e) => (),
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
