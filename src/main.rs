use std::time::Duration;

// Docker-Autoheal modules
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

// Docker-Autoheal functions
use execute::{connect::connect_docker, looper::start_loop};
use inquire::{environment::get_var, options::get_opts};
use report::logging::log_message;

// Error level constants
pub const INFO: i8 = 0;
pub const WARNING: i8 = 1;
pub const ERROR: i8 = 2;

// Allowed connection types
pub const ALLOWED_CONNECTION_TYPES: [&str; 4] = ["local", "socket", "http", "ssl"];

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Collect binary arguments
    let args = std::env::args().collect();
    let opt = get_opts(args);

    // Get Autoheal core variables
    // Determine if we have valid arguments, need to check env, or use defaults
    let var = get_var(opt).await;

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

    // Delay start of loop, if specified
    if var.start_delay > 0 {
        let msg0 = format!("Delaying evaluation {}s on request", var.start_delay);
        log_message(&msg0, INFO).await;
        tokio::time::sleep(Duration::from_secs(var.start_delay)).await;
    }

    // Begin work
    start_loop(
        var.interval,
        var.container_label,
        var.stop_timeout,
        var.apprise_url,
        var.webhook_key,
        var.webhook_url,
        docker,
    )
    .await
}
