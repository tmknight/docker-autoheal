use std::time::Duration;

// Docher-Autoheal modules
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

// Docher-Autoheal functions
use execute::{connect::connect_docker, looper::start_loop};
use inquire::{environment::get_var, options::get_opts};
use report::logging::log_message;

// Error level constants
pub const INFO: i8 = 0;
pub const WARNING: i8 = 1;
pub const ERROR: i8 = 2;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Collect binary arguments
    let args: Vec<String> = std::env::args().collect();
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
        std::thread::sleep(Duration::from_secs(var.start_delay));
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
