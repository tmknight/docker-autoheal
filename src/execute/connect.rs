use crate::{report::logging::log_message, ERROR, INFO};
use bollard::{Docker, API_DEFAULT_VERSION};

pub async fn connect_docker(
    autoheal_connection_type: String,
    autoheal_tcp_address: String,
    autoheal_tcp_timeout: u64,
) -> Docker {
    // Log final connection paramaters
    let msg0 = format!("Monitoring Docker via {}", autoheal_connection_type);
    log_message(&msg0, INFO).await;
    match autoheal_connection_type.as_str() {
        "http" => {
            let msg1 = format!("Connecting to {}", autoheal_tcp_address);
            log_message(&msg1, INFO).await;
        }
        &_ => {}
    }
    // Connect to Docker as specified
    let docker = match autoheal_connection_type.as_str() {
        "http" => Docker::connect_with_http(
            &autoheal_tcp_address,
            autoheal_tcp_timeout,
            API_DEFAULT_VERSION,
        ),
        "socket" => Docker::connect_with_socket_defaults(),
        &_ => Docker::connect_with_local_defaults(),
    };
    match docker {
        Ok(docker) => docker,
        Err(e) => {
            let msg0 = String::from("Could not reliably connect to Docker host");
            log_message(&msg0, ERROR).await;
            panic!("{e}")
        }
    }
}
