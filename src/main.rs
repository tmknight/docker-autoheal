use std::time::Duration;

mod execute {
    pub mod connect;
    pub mod looper;
}
mod inquire {
    pub mod environment;
    pub mod inspect;
    pub mod list;
    pub mod options;
}
mod report {
    pub mod logging;
    pub mod webhook;
}

use execute::{connect::connect_docker, looper::start_loop};
use inquire::{environment::get_env, options::get_opts};
use report::logging::log_message;

pub const INFO: i8 = 0;
pub const WARNING: i8 = 1;
pub const ERROR: i8 = 2;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Collect binary arguments
    let args: Vec<String> = std::env::args().collect();
    let opt = get_opts(args);

    // Autoheal core variables
    // Determine if we have valid arguments, need to check env, or use defaults
    let autoheal_connection_type: String = match opt.connection_type {
        None => {
            let allowed_connection_types: Vec<&str> = vec!["local", "socket", "http", "ssl"];
            let env_connection_type = get_env("AUTOHEAL_CONNECTION_TYPE", "local");
            match allowed_connection_types.contains(&env_connection_type.as_str()) {
                true => env_connection_type,
                false => {
                    let msg0 = format!(
                        "Unexpected connection-type ({}): {}",
                        allowed_connection_types.join(","),
                        env_connection_type
                    );
                    log_message(&msg0, ERROR).await;
                    let msg1 = String::from("Attempting connection via default (local)");
                    log_message(&msg1, WARNING).await;
                    "local".to_string()
                }
            }
        }
        Some(o) => o,
    };
    let autoheal_container_label: String = match opt.container_label {
        None => get_env("AUTOHEAL_CONTAINER_LABEL", "autoheal"),
        Some(o) => o,
    };
    let autoheal_stop_timeout: isize = match opt.stop_timeout {
        None => get_env("AUTOHEAL_STOP_TIMEOUT", "10").parse().unwrap(),
        Some(o) => match o.parse() {
            Ok(a) => a,
            Err(e) => {
                let msg0 = format!("Unexpected value; using default: {}", e);
                log_message(&msg0, WARNING).await;
                10
            }
        },
    };
    let autoheal_interval: u64 = match opt.interval {
        None => get_env("AUTOHEAL_INTERVAL", "5").parse().unwrap(),
        Some(o) => match o.parse() {
            Ok(a) => a,
            Err(e) => {
                let msg0 = format!("Unexpected value; using default: {}", e);
                log_message(&msg0, WARNING).await;
                5
            }
        },
    };
    let autoheal_start_delay: u64 = match opt.start_delay {
        None => get_env("AUTOHEAL_START_DELAY", "0").parse().unwrap(),
        Some(o) => match o.parse() {
            Ok(a) => a,
            Err(e) => {
                let msg0 = format!("Unexpected value; using default: {}", e);
                log_message(&msg0, WARNING).await;
                0
            }
        },
    };

    // Autoheal tcp variables
    let autoheal_tcp_host: String = match opt.tcp_host {
        None => get_env("AUTOHEAL_TCP_HOST", "localhost"),
        Some(o) => o,
    };
    let autoheal_tcp_port: u64 = match autoheal_connection_type.as_str() {
        "ssl" => match opt.tcp_port {
            None => get_env("AUTOHEAL_TCP_PORT", "2376").parse().unwrap(),
            Some(o) => match o.parse() {
                Ok(a) => a,
                Err(e) => {
                    let msg0 = format!("Unexpected value; using default: {}", e);
                    log_message(&msg0, WARNING).await;
                    2376
                }
            },
        },
        &_ => match opt.tcp_port {
            None => get_env("AUTOHEAL_TCP_PORT", "2375").parse().unwrap(),
            Some(o) => match o.parse() {
                Ok(a) => a,
                Err(e) => {
                    let msg0 = format!("Unexpected value; using default: {}", e);
                    log_message(&msg0, WARNING).await;
                    2375
                }
            },
        },
    };
    let autoheal_tcp_address: String = format!("{}:{}", autoheal_tcp_host, autoheal_tcp_port);
    let autoheal_tcp_timeout: u64 = match opt.tcp_timeout {
        None => get_env("AUTOHEAL_TCP_TIMEOUT", "10").parse().unwrap(),
        Some(o) => match o.parse() {
            Ok(a) => a,
            Err(e) => {
                let msg0 = format!("Unexpected value; using default: {}", e);
                log_message(&msg0, WARNING).await;
                10
            }
        },
    };

    // Autoheal ssl variables
    let autoheal_pem_path: String = match opt.key_path {
        None => get_env("AUTOHEAL_PEM_PATH", "/opt/docker-autoheal/tls"),
        Some(o) => o,
    };
    let autoheal_key_path = format!("{}/key.pem", autoheal_pem_path);
    let autoheal_cert_path = format!("{}/cert.pem", autoheal_pem_path);
    let autoheal_ca_path = format!("{}/ca.pem", autoheal_pem_path);

    // Webhook variables
    let autoheal_apprise_url: String = match opt.apprise_url {
        None => get_env("AUTOHEAL_APPRISE_URL", ""),
        Some(o) => o,
    };
    let autoheal_webhook_key: String = match opt.webhook_key {
        None => get_env("AUTOHEAL_WEBHOOK_KEY", ""),
        Some(o) => o,
    };
    let autoheal_webhook_url: String = match opt.webhook_url {
        None => get_env("AUTOHEAL_WEBHOOK_URL", ""),
        Some(o) => o,
    };

    // Determine connection type & connect to Docker per type
    let docker = connect_docker(
        autoheal_connection_type,
        autoheal_tcp_address,
        autoheal_tcp_timeout,
        autoheal_key_path,
        autoheal_cert_path,
        autoheal_ca_path,
    )
    .await;

    // Delay start of loop if specified
    if autoheal_start_delay > 0 {
        let msg0 = format!("Delaying evaluation {}s on request", autoheal_start_delay);
        log_message(&msg0, INFO).await;
        std::thread::sleep(Duration::from_secs(autoheal_start_delay));
    }

    // Begin work
    start_loop(
        autoheal_interval,
        autoheal_container_label,
        autoheal_stop_timeout,
        autoheal_apprise_url,
        autoheal_webhook_key,
        autoheal_webhook_url,
        docker,
    )
    .await
}
