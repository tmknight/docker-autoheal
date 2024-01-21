use crate::report::logging::log_message;
use bollard::Docker;

const ZERO64: i64 = 0;

pub struct Result {
    pub failed: bool,
    pub failing_streak: i64,
}

pub async fn inspect_container(docker: Docker, name: &str, id: &str) -> Result {
    // Attempt to inspect the container
    let container_inspect = match docker.inspect_container(id, None).await {
        Ok(repsonse) => repsonse,
        Err(_) => {
            // Log that we had an error
            let msg0 = format!(
                "[{}] Could not reliably determine container ({}) information from inspection",
                name, id
            );
            log_message(&msg0, 2).await;
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
            log_message(&msg0, 2).await;
            // Health information is not available, set failing_streak to 0
            ZERO64
        }
    };
    Result {
        failed: !matches!(failing_streak, ZERO64),
        failing_streak,
    }
}
