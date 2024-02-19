use crate::{report::logging::log_message, ERROR, INFO};
use std::fs;
use std::process::Command;

pub async fn execute_command(post_action: String, name: &str, id: String, timeout: String) {
    // Check if the script exists
    if fs::metadata(post_action.clone()).is_ok() {
        // Execute using Command
        let mut command = Command::new(post_action.clone());

        // Arguments to the command
        command.args([name, &id, &timeout]);

        // Execute the command and handle the result
        let msg0 = match command.spawn() {
            Ok(mut child) => {
                // Wait for the child process to finish
                match child.wait() {
                    Ok(_s) => format!(
                        "[{}] Post-action ({}) for container ({}) was successful",
                        name, post_action, id
                    ),
                    Err(e) => format!(
                        "[{}] Post-action ({}) for container ({}) failed to complete: {}",
                        name, post_action, id, e
                    ),
                }
            }
            Err(e) => format!(
                "[{}] Post-action ({}) for container ({}) failed to start: {}",
                name, post_action, id, e
            ),
        };
        log_message(&msg0, INFO).await;
    } else {
        let msg0 = format!(
            "[{}] Post-action ({}) for container ({}) not found",
            name, post_action, id
        );
        log_message(&msg0, ERROR).await;
    }
}
