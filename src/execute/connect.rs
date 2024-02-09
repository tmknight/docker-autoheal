use crate::{log_message, ERROR, INFO};
use bollard::{Docker, API_DEFAULT_VERSION};

pub async fn connect_docker(
    connection_type: String,
    tcp_address: String,
    tcp_timeout: u64,
    key_path: String,
    cert_path: String,
    ca_path: String,
) -> Docker {
    // Log final connection paramaters
    let msg0 = format!("Monitoring Docker via {}", connection_type);
    log_message(&msg0, INFO).await;

    // Connect to Docker as specified
    let docker = match connection_type.as_str() {
        "http" => {
            let msg1 = format!("Connecting to {}", tcp_address);
            log_message(&msg1, INFO).await;
            Docker::connect_with_http(&tcp_address, tcp_timeout, API_DEFAULT_VERSION)
        }
        "socket" => Docker::connect_with_socket_defaults(),
        "ssl" => {
            let msg1 = format!("Connecting to {}", tcp_address);
            log_message(&msg1, INFO).await;
            let msg2 = format!(
                "Certificate information: {}, {}, {}",
                key_path, cert_path, ca_path
            );
            log_message(&msg2, INFO).await;
            Docker::connect_with_ssl(
                &tcp_address,
                std::path::Path::new(&key_path),
                std::path::Path::new(&cert_path),
                std::path::Path::new(&ca_path),
                tcp_timeout,
                API_DEFAULT_VERSION,
            )
        }
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
