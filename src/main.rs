use std::time::Duration;

// Docker-Autoheal modules
mod execute {
    pub mod connect;
    pub mod looper;
    pub mod postaction;
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

// Docker-Autoheal functions
use execute::{connect::connect_docker, looper::start_loop};
use inquire::{environment::get_var, options::get_opts};
use report::logging::log_message;

// Error level constants
const INFO: i8 = 0;
const WARNING: i8 = 1;
const ERROR: i8 = 2;

// Allowed connection types
const ALLOWED_CONNECTION_TYPES: [&str; 4] = ["local", "socket", "http", "ssl"];

struct LoopVariablesList {
    container_label: String,
    stop_timeout: isize,
    interval: u64,
    apprise_url: String,
    webhook_key: String,
    webhook_url: String,
    post_action: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Collect binary arguments
    let args = std::env::args().collect();
    let opt = get_opts(args);

    // Get Autoheal core variables
    // Determine if we have valid arguments, need to check env, or use defaults
    let var = get_var(opt).await;

    // Delay start of monitoring, if specified
    if var.start_delay > 0 {
        let msg0 = format!("Pausing startup {}s on request", var.start_delay);
        log_message(&msg0, INFO).await;
        tokio::time::sleep(Duration::from_secs(var.start_delay)).await;
        let msg1 = String::from("Resuming startup");
        log_message(&msg1, INFO).await;
    }

    // Connect to Docker per type
    let docker = connect_docker(
        var.connection_type,
        var.tcp_address,
        var.tcp_timeout,
        var.key_path,
        var.cert_path,
        var.ca_path,
    )
    .await;

    let loop_variables = {
        LoopVariablesList {
            container_label: var.container_label,
            stop_timeout: var.stop_timeout,
            interval: var.interval,
            apprise_url: var.apprise_url,
            webhook_key: var.webhook_key,
            webhook_url: var.webhook_url,
            post_action: var.post_action,
        }
    };

    // Begin work
    start_loop(loop_variables, docker).await
}
