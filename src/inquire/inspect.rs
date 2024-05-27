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
                "[{} ({})] Could not reliably determine container information from inspection",
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
                "[{} ({})] Could not reliably determine container failing streak; default to 0",
                name, id
            );
            log_message(&msg0, ERROR).await;
            // Health information is not available, set failing_streak to 0
            0
        }
    };
    // Get last 'output' and 'exitcode' from state:health
    let mut failing_reason = "unknown".to_string();
    let mut exit_code: i64 = -1;
    if let Some(log) = container_inspect.state.as_ref().and_then(|s| s.health.as_ref().and_then(|h| h.log.clone())) {
        if let Some(last) = log.last() {
            failing_reason = last.output.clone().unwrap_or(failing_reason);
            exit_code = last.exit_code.unwrap_or(exit_code);
        } else {
            failing_reason = "log is empty".to_string();
        }
    } else {
        let msg0 = format!(
            "[{} ({})] Could not reliably determine container failing reason",
            name, id
        );
        log_message(&msg0, ERROR).await;
    }
    Result {
        failed: failing_streak != 0,
        failing_streak,
        failing_reason,
        exit_code,
    }
}
