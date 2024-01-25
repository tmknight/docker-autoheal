use getopts::Options;
use std::time::Duration;

mod execute {
    pub mod connect;
    pub mod looper;
}
mod inquire {
    pub mod environment;
    pub mod inspect;
    pub mod list;
}
mod report {
    pub mod logging;
}

use execute::{connect::connect_docker, looper::start_loop};
use inquire::environment::get_env;
use report::logging::{log_message, print_version};

pub const INFO: i8 = 0;
pub const WARNING: i8 = 1;
pub const ERROR: i8 = 2;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Collect binary arguments
    let args: Vec<String> = std::env::args().collect();
    let program = args[0].clone();

    // Establish usable arguments
    let mut opts = Options::new();
     opts.optopt(
        "c",
        "connection-type",
        "One of local, socket, or http",
        "<CONNECTION_TYPE>",
    );
    opts.optopt(
        "l",
        "container-label",
        "Container label to monitor (e.g. autoheal)",
        "<CONTAINER_LABEL>",
    );
    opts.optopt(
        "t",
        "stop-timeout",
        "Time in seconds to wait for action to complete",
        "<STOP_TIMEOUT>",
    );
    opts.optopt(
        "i",
        "interval",
        "Time in seconds to check health",
        "<INTERVAL>",
    );
    opts.optopt(
        "d",
        "start-delay",
        "Time in seconds to wait for first check",
        "<START_DELAY>",
    );
    opts.optopt(
        "n",
        "tcp-host",
        "The hostname or IP address of the Docker host (when -c http or ssl)",
        "<TCP_HOST>",
    );
    opts.optopt(
        "p",
        "tcp-port",
        "The tcp port number of the Docker host (when -c http or ssl)",
        "<TCP_PORT>",
    );
    opts.optflag("h", "help", "Print help");
    opts.optflag("v", "version", "Print version information");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(e) => {
            println!("{}", e);
            println!("{}", opts.usage(&program));
            std::process::exit(1);
        }
    };

    // Process matching arguments
    if matches.opt_present("v") {
        print_version();
        return Ok(());
    } else if matches.opt_present("h") {
        println!("{}", opts.usage(&program));
        return Ok(());
    }
    let connection_type = matches.opt_str("c").unwrap_or_default();
    let container_label = matches.opt_str("l").unwrap_or_default();
    let stop_timeout = matches.opt_str("t").unwrap_or_default();
    let interval = matches.opt_str("i").unwrap_or_default();
    let start_delay = matches.opt_str("d").unwrap_or_default();
    let tcp_host = matches.opt_str("n").unwrap_or_default();
    let tcp_port = matches.opt_str("p").unwrap_or_default();

    // Autoheal core variables
    // Determine if we have valid arguments, need to check env, or use defaults
    let autoheal_connection_type: String = match connection_type.is_empty() {
        true => get_env("AUTOHEAL_CONNECTION_TYPE", "local").to_string(),
        false => connection_type,
    };
    let autoheal_container_label: String = match container_label.is_empty() {
        true => get_env("AUTOHEAL_CONTAINER_LABEL", "autoheal").to_string(),
        false => container_label,
    };
    let autoheal_stop_timeout: isize = match stop_timeout.is_empty() {
        true => get_env("AUTOHEAL_STOP_TIMEOUT", "10").parse().unwrap(),
        false => stop_timeout.parse().unwrap(),
    };
    let autoheal_interval: u64 = match interval.is_empty() {
        true => get_env("AUTOHEAL_INTERVAL", "5").parse().unwrap(),
        false => interval.parse().unwrap(),
    };
    let autoheal_start_delay: u64 = match start_delay.is_empty() {
        true => get_env("AUTOHEAL_START_DELAY", "0").parse().unwrap(),
        false => start_delay.parse().unwrap(),
    };

    // Autoheal tcp variables
    let autoheal_tcp_host: String = match tcp_host.is_empty() {
        true => get_env("AUTOHEAL_TCP_HOST", "localhost"),
        false => tcp_host.parse().unwrap(),
    };
    let autoheal_tcp_port: u64 = match autoheal_connection_type.as_str() {
        "ssl" => match tcp_port.is_empty() {
            true => get_env("AUTOHEAL_TCP_PORT", "2376").parse().unwrap(),
            false => tcp_port.parse().unwrap(),
        },
        &_ => get_env("AUTOHEAL_TCP_PORT", "2375").parse().unwrap(),
    };
    let autoheal_tcp_address: String = format!("{}:{}", autoheal_tcp_host, autoheal_tcp_port);
    let autoheal_tcp_timeout: u64 = match stop_timeout.is_empty() {
        true => get_env("AUTOHEAL_TCP_TIMEOUT", "10").parse().unwrap(),
        false => stop_timeout.parse().unwrap(),
    };

    // Autoheal ssl variables
    let autoheal_pem_path = match key_path.is_empty() {
        true => get_env("AUTOHEAL_PEM_PATH", "/opt/docker-autoheal/tls"),
        false => key_path.parse().unwrap(),
    };
    let autoheal_key_path: String = format!("{}/key.pem", autoheal_pem_path);
    let autoheal_cert_path: String = format!("{}/cert.pem", autoheal_pem_path);
    let autoheal_ca_path: String = format!("{}/ca.pem", autoheal_pem_path);

    // Determine connection type & connect to docker per type
    let docker = connect_docker(
        autoheal_connection_type,
        autoheal_tcp_address,
        autoheal_tcp_timeout,
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
        docker,
    )
    .await
}
