use crate::{log_message, ERROR};
use bollard::Docker;

pub struct Result {
    pub failed: bool,
    pub failing_streak: i64,
    pub failing_reason: String,
    pub exit_code: i64,
}

pub async fn inspect_container(docker: Docker, name: &str, id: &str) -> Result {
    // Attempt to inspect the container
    let container_inspect = match docker.inspect_container(id, None).await {
        Ok(response) => response,
        Err(_) => {
            // Log that we had an error
            let msg0 = format!(
                "[{}] Could not reliably determine container ({}) information from inspection",
                name, id
            );
            log_message(&msg0, ERROR).await;
            // Return container default if err so we can carry on
            Default::default()
        }
    };
    // Get failing streak from state:health
    let failing_streak = match container_inspect
        .state
        .as_ref()
        .and_then(|s| s.health.as_ref().and_then(|h| h.failing_streak))
    {
        Some(streak) => streak,
        None => {
            // Log that we had an error
            let msg0 = format!(
                "[{}] Could not reliably determine container ({}) failing streak; default to 0",
                name, id
            );
            log_message(&msg0, ERROR).await;
            // Health information is not available, set failing_streak to 0
            0
        }
    };
    // Get last 'output' and 'exitcode' from state:health
    let default_reason = String::from("unknown");
    let mut failing_reason = default_reason.clone();
    let mut exit_code: i64 = -1;
    match container_inspect
        .state
        .as_ref()
        .and_then(|s| s.health.as_ref().and_then(|h| h.log.clone()))
    {
        Some(log) => {
            let last = log.len() - 1;
            let reason = log[last].clone().output.unwrap_or(default_reason);
            failing_reason = reason.clone();
            exit_code = log[last].clone().exit_code.unwrap_or(-1);
        }
        None => {
            // Log that we had an error
            let msg0 = format!(
                "[{}] Could not reliably determine container ({}) failing reason",
                name, id
            );
            log_message(&msg0, ERROR).await;
        }
    };
    Result {
        failed: !matches!(failing_streak, 0),
        failing_streak,
        failing_reason,
        exit_code,
    }
}
