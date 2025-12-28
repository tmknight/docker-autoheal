use super::record::{read_record, write_record, JsonRecord};
use crate::{INFO, LOG_FILE, LOG_PATH, WARNING, YEAR};
use chrono::Local;
use std::io::{stdout, Write};

// Return information about the binary
pub const NAME: &str = env!("CARGO_PKG_NAME");
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const AUTHORS: &str = env!("CARGO_PKG_AUTHORS");
pub const LICENSE: &str = env!("CARGO_PKG_LICENSE");
pub const DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");
pub const HOMEPAGE: &str = env!("CARGO_PKG_HOMEPAGE");

pub fn print_version() {
    println!("Name: {}", NAME);
    println!("Version: {}", VERSION);
    println!("Authors: {}", AUTHORS);
    println!("License: {}", LICENSE);
    println!("Description: {}", DESCRIPTION);
    println!("Homepage: {}", HOMEPAGE);
    println!();
    println!("Copyright (C) {}", YEAR);
    println!("This program comes with ABSOLUTELY NO WARRANTY.");
    println!(
        "This is free software, and you are welcome to redistribute it under certain conditions."
    );
    println!("For details, please refer to the GNU General Public License:");
    println!("https://www.gnu.org/licenses/gpl-3.0.html");
}

// Logging
pub async fn log_message(msg: &str, lvl: i8) {
    let date = Local::now().format("%Y-%m-%d %H:%M:%S%z").to_string();
    let level = match lvl {
        1 => "[WARNING]",
        2 => "[  ERROR]",
        _ => "[   INFO]",
    };
    let mut lock = stdout().lock();
    writeln!(lock, "{} {} {}", date, level, msg).ok();
}

// Write to log.json
pub async fn log_write(data: JsonRecord) {
    match write_record(data).await {
        Ok(()) => (),
        Err(e) => {
            let msg0 = format!("Unable to write to log ({}{}): {}", LOG_PATH, LOG_FILE, e);
            log_message(&msg0, WARNING).await
        }
    }
}

// Read from log.json
pub async fn log_read(name: &str, id: String) {
    match read_record().await {
        Ok(records) => {
            // Get unhealthy count for container
            let action_count = records.into_iter().filter(|r| r.id == id).count();
            // Report results
            let mut noun = "time";
            if action_count > 1 {
                noun = "times"
            }
            let msg0 = format!(
                "[{} ({})] Container has been unhealthy {} {}",
                name, id, action_count, noun
            );
            log_message(&msg0, INFO).await;
        }
        Err(e) => {
            let msg0 = format!("Unable to read from log ({}{}): {}", LOG_PATH, LOG_FILE, e);
            log_message(&msg0, WARNING).await
        }
    }
}
