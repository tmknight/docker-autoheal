use bollard::container::{ListContainersOptions, RestartContainerOptions};
use bollard::{Docker, API_DEFAULT_VERSION};
use chrono::prelude::*;
use std::collections::HashMap;
use std::io::{stdout, Write};
use std::time::Duration;

// Logging
async fn log_message(msg: &str) {
    let date = Local::now().format("%Y-%m-%d %H:%M:%S%z").to_string();
    let mut lock = stdout().lock();
    writeln!(lock, "{} {}", date, msg).unwrap();
}

// Return environment variable
fn get_env(key: &str, default: &str) -> String {
    match std::env::var(key) {
        Ok(val) => val.to_lowercase(),
        Err(_e) => default.to_string().to_lowercase(),
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Autoheal core variables
    let autoheal_connection_type: String = get_env("AUTOHEAL_CONNECTION_TYPE", "local");
    let autoheal_container_label: String = get_env("AUTOHEAL_CONTAINER_LABEL", "autoheal");
    let autoheal_stop_timeout: isize = get_env("AUTOHEAL_STOP_TIMEOUT", "10").parse().unwrap();
    let autoheal_interval: u64 = get_env("AUTOHEAL_INTERVAL", "5").parse().unwrap();
    let autoheal_start_delay: u64 = get_env("AUTOHEAL_START_DELAY", "0").parse().unwrap();
    // Autoheal tcp variables
    let autoheal_tcp_host: String = get_env("AUTOHEAL_TCP_HOST", "localhost");
    let autoheal_tcp_port: u64 = get_env("AUTOHEAL_TCP_PORT", "2375").parse().unwrap();
    let autoheal_tcp_address: String = format!("{}:{}", autoheal_tcp_host, autoheal_tcp_port);
    let autoheal_tcp_timeout: u64 = get_env("AUTOHEAL_TCP_TIMEOUT", "10").parse().unwrap();

    // todo
    // Autoheal ssl variables
    // let autoheal_key_path: String =
    //     get_env("AUTOHEAL_KEY_PATH", "/opt/docker-autoheal/tls/key.pem");
    // let autoheal_cert_path: String =
    //     get_env("AUTOHEAL_CERT_PATH", "/opt/docker-autoheal/tls/cert.pem");
    // let autoheal_ca_path: String = get_env("AUTOHEAL_CA_PATH", "/opt/docker-autoheal/tls/ca.pem");

    // todo
    // Webhook variables
    // let webhook_url = "";
    // let webhook_json_key = "text";
    // let apprise_url = "";

    // Determine connection type & connect to docker per type
    let mut docker_tmp: Option<Docker> = None;
    match autoheal_connection_type.as_str() {
        "socket" => {
            docker_tmp = Some(
                // #[cfg(unix)]
                Docker::connect_with_socket_defaults()?,
            );
        }
        "http" => {
            docker_tmp = Some(Docker::connect_with_http(
                &autoheal_tcp_address,
                autoheal_tcp_timeout,
                API_DEFAULT_VERSION,
            )?);
        }
        // todo
        // "ssl" => {
        //     docker_tmp = Some(
        //         #[cfg(feature = "ssl")]
        //         Docker::connect_with_ssl(
        //             autoheal_tcp_address,
        //             autoheal_tcp_timeout,
        //             Path::new(autoheal_key_path),
        //             Path::new(autoheal_cert_path),
        //             Path::new(autoheal_ca_path),
        //             API_DEFAULT_VERSION
        //         )?,
        //     );
        // }
        &_ => {
            docker_tmp = Some(Docker::connect_with_local_defaults()?);
        }
    }
    // Unwrap final connection paramaters
    let msg0 = format!("[INFO] Monitoring Docker via {}", autoheal_connection_type);
    log_message(&msg0).await;
    if autoheal_connection_type == "http" {
        let msg1 = format!(
            "[INFO] Connecting to {}:{}",
            autoheal_tcp_host, autoheal_tcp_port
        );
        log_message(&msg1).await;
    }
    let docker = docker_tmp.unwrap();

    // Delay start of loop if specified
    if autoheal_start_delay > 0 {
        let msg0 = format!(
            "[INFO] Delaying evaluation {}s on request",
            autoheal_start_delay
        );
        log_message(&msg0).await;
        std::thread::sleep(Duration::from_secs(autoheal_start_delay));
    }

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
        let containers = docker.list_containers(container_options).await?;
        for container in containers {
            // Execute concurrently
            let docker_clone = docker.clone();
            let join = tokio::task::spawn(async move {
                // Get name of container
                let name_tmp = match &container.names {
                    Some(names) => &names[0],
                    None => {
                        let msg0 = format!("[ERROR] Could not reliably determine container name");
                        log_message(&msg0).await;
                        ""
                    }
                };
                let name = name_tmp.trim_matches('/').trim();

                // Get id of container
                let id: String = match container.id {
                    Some(id) => id.chars().take(12).collect(),
                    None => {
                        let msg0 = format!("[ERROR] Could not reliably determine container id");
                        log_message(&msg0).await;
                        "".to_string()
                    }
                };

                if !(name.is_empty() && id.is_empty()) {
                    // Report unhealthy container
                    let msg0 = format!("[WARNING] [{}] Container ({}) unhealthy", name, id);
                    log_message(&msg0).await;

                    // Build restart options
                    let restart_options = Some(RestartContainerOptions {
                        t: autoheal_stop_timeout,
                        ..Default::default()
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
                    let msg0 = format!("[ERROR] Could not reliably identify the container");
                    log_message(&msg0).await;
                }
            });
            join.await?;
        }
        // Loop interval
        interval.tick().await;
    }
}
