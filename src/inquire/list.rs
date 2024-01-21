use crate::report::logging::log_message;
use bollard::container::ListContainersOptions;
use bollard::models::ContainerSummary;
use bollard::Docker;
use std::collections::HashMap;

pub async fn containers_list(
    autoheal_container_label: &String,
    docker: Docker,
) -> Vec<ContainerSummary> {
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
    match docker.list_containers(container_options).await {
        Ok(list) => list,
        Err(e) => {
            let msg0 = String::from("Could not reliably determine containers to assess");
            log_message(&msg0, 2).await;
            panic!("{e}")
        }
    }
}
