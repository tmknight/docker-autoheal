use bollard::container::{ListContainersOptions, RestartContainerOptions};
use bollard::Docker;
use chrono::prelude::*;
use std::collections::HashMap;
use std::io::{stdout, Write};
use std::time::Duration;

async fn log_message(msg: &str) {
    let date = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    let mut lock = stdout().lock();
    writeln!(lock, "{} {}", date, msg).unwrap();
}

fn get_env(key: &str, default: &str) -> String {
    match std::env::var(key) {
        Ok(val) => return val,
        Err(e) => return default.to_string(),
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Autoheal variables
    let autoheal_connection_type = get_env("AUTOHEAL_CONNECTION_TYPE", "local");
    let autoheal_container_label = get_env("AUTOHEAL_CONTAINER_LABEL", "autoheal");
    let autoheal_default_stop_timeout = get_env("AUTOHEAL_DEFAULT_STOP_TIMEOUT", "10")
        .parse()
        .unwrap();
    let autoheal_interval = get_env("AUTOHEAL_INTERVAL", "5").parse().unwrap();
    let autoheal_start_period = get_env("AUTOHEAL_START_PERIOD", "0").parse().unwrap();

    // todo
    // Webhook variables
    // let webhook_url = "";
    // let webhook_json_key = "text";
    // let apprise_url = "";

    // Determine connection type & Connect to docker
    let mut docker_tmp: Option<Docker> = None;
    match autoheal_connection_type.as_str() {
        "socket" => {
            docker_tmp = Some(
                #[cfg(unix)]
                Docker::connect_with_socket_defaults()?,
            );
        }
        "http" => {
            docker_tmp = Some(Docker::connect_with_http_defaults()?);
        }
        // todo
        // "ssl" => {
        //     docker_tmp = Some(
        //         // #[cfg(feature = "ssl")]
        //         Docker::connect_with_ssl_defaults()?,
        //     );
        // }
        &_ => {
            docker_tmp = Some(Docker::connect_with_local_defaults()?);
        }
    }
    // Unwrap final connection paramaters
    let msg0 = format!("Monitoring Docker via {}", autoheal_connection_type);
    log_message(&msg0).await;
    let docker = docker_tmp.unwrap();

    // Delay start of loop if specified
    if autoheal_start_period > 0 {
        let msg0 = format!("Delaying evaluation {}s on request", autoheal_start_period);
        log_message(&msg0).await;
        std::thread::sleep(Duration::from_secs(autoheal_start_period));
    }

    // Establish loop interval
    let mut interval = tokio::time::interval(Duration::from_secs(autoheal_interval));

    loop {
        // Loop interval
        interval.tick().await;
        // Build container assessment criteria
        let mut filters = HashMap::new();
        filters.insert("health", vec!["unhealthy"]);
        if autoheal_container_label != "ALL" {
            filters.insert("label", vec![&autoheal_container_label]);
        }
        // Gather all containers that are unhealthy
        let container_options = Some(ListContainersOptions {
            all: true,
            filters,
            ..Default::default()
        });
        let containers = docker.list_containers(container_options).await?;

        for container in containers {
            // Execute concurrently
            let docker_clone = docker.clone();
            let join = tokio::task::spawn(async move {
                // Get name of container
                let name0 = &container.names.unwrap()[0];
                let name = name0.trim_matches('/').trim();
                // Get id of container
                let id: String = container.id.unwrap().chars().take(12).collect();
                // Determine if state is readable
                if let Some(state) = container.state {
                    // Determine if matches restart criteria
                    if !matches!(state.as_str(), "paused" | "restarting") {
                        // Build restart options
                        let restart_options = Some(RestartContainerOptions {
                            t: autoheal_default_stop_timeout,
                            ..Default::default()
                        });
                        // Report what is transpiring
                        let msg0 = format!("Container '{}' ({}) unhealthy", name, id);
                        // todo
                        // let msg1 = format!(
                        //     "Restarting '{}' with {}s timeout",
                        //     name, autoheal_default_stop_timeout
                        // );
                        let msg1 = format!("Restarting '{}' now", name);

                        log_message(&msg0).await;
                        log_message(&msg1).await;
                        // Restart unhealthy container
                        let rslt = docker_clone.restart_container(&id, restart_options).await;
                        match rslt {
                            Ok(()) => {
                                let msg0 = format!("Restart of '{}' was successful", name);
                                log_message(&msg0).await;
                            }
                            Err(e) => {
                                let msg0 = format!("Restart of '{}' failed: {}", name, e);
                                log_message(&msg0).await;
                            }
                        }
                    }
                } else {
                    let msg0 = format!("Could not determine state of {}", name);
                    log_message(&msg0).await;
                }
            });
            join.await?;
        }
    }
}
