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
                        "[{} ({})] Container post-action ({}) was successful",
                        name, id, post_action
                    ),
                    Err(e) => format!(
                        "[{} ({})] Container post-action ({}) failed to complete: {}",
                        name, id, post_action, e
                    ),
                }
            }
            Err(e) => format!(
                "[{} ({})] Container post-action ({}) failed to start: {}",
                name, id, post_action, e
            ),
        };
        log_message(&msg0, INFO).await;
    } else {
        let msg0 = format!(
            "[{} ({})] Container post-action ({}) not found",
            name, id, post_action
        );
        log_message(&msg0, ERROR).await;
    }
}
