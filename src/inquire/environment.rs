use super::options::OptionsList;
use crate::{log_message, ALLOWED_CONNECTION_TYPES, ERROR, WARNING};

pub struct VariablesList {
    pub connection_type: String,
    pub stop_timeout: isize,
    pub interval: u64,
    pub start_delay: u64,
    pub tcp_address: String,
    pub tcp_timeout: u64,
    pub key_path: String,
    pub cert_path: String,
    pub ca_path: String,
    pub apprise_url: String,
    pub webhook_key: String,
    pub webhook_url: String,
    pub post_action: String,
    pub log_all: bool,
    pub monitor_all: bool,
    pub history: bool,
}

// Get environment variable
fn get_env(key: &str, default: &str) -> String {
    match std::env::var(key) {
        Ok(val) => val.to_lowercase(),
        Err(_e) => default.to_string().to_lowercase(),
    }
}

// Determine if we have valid arguments, need to check env, or use defaults
pub async fn get_var(opt: OptionsList) -> VariablesList {
    let autoheal_connection_type: String = match opt.connection_type {
        None => {
            let env_connection_type = get_env("AUTOHEAL_CONNECTION_TYPE", "local");
            match ALLOWED_CONNECTION_TYPES.contains(&env_connection_type.as_str()) {
                true => env_connection_type,
                false => {
                    let msg0 = format!(
                        "Unexpected connection-type ({}): Expected one of {}",
                        env_connection_type,
                        ALLOWED_CONNECTION_TYPES.join(",")
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
    let autoheal_post_action: String = match opt.post_action {
        None => get_env("AUTOHEAL_POST_ACTION", ""),
        Some(o) => o,
    };
    let mut autoheal_log_all = get_env("AUTOHEAL_LOG_ALL", "false") == "true";
    if opt.log_all {
        autoheal_log_all = true;
    }
    let mut autoheal_monitor_all = get_env("AUTOHEAL_MONITOR_ALL", "false") == "true";
    if opt.monitor_all {
        autoheal_monitor_all = true
    }
    let mut autoheal_history = get_env("AUTOHEAL_HISTORY", "false") == "true";
    if opt.history {
        autoheal_history = true
    }

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
    let autoheal_key_path: String = format!("{}/key.pem", autoheal_pem_path);
    let autoheal_cert_path: String = format!("{}/cert.pem", autoheal_pem_path);
    let autoheal_ca_path: String = format!("{}/ca.pem", autoheal_pem_path);

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

    VariablesList {
        connection_type: autoheal_connection_type,
        stop_timeout: autoheal_stop_timeout,
        interval: autoheal_interval,
        start_delay: autoheal_start_delay,
        tcp_address: autoheal_tcp_address,
        tcp_timeout: autoheal_tcp_timeout,
        key_path: autoheal_key_path,
        cert_path: autoheal_cert_path,
        ca_path: autoheal_ca_path,
        apprise_url: autoheal_apprise_url,
        webhook_key: autoheal_webhook_key,
        webhook_url: autoheal_webhook_url,
        post_action: autoheal_post_action,
        log_all: autoheal_log_all,
        monitor_all: autoheal_monitor_all,
        history: autoheal_history,
    }
}
