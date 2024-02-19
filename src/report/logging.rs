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
    println!("Copyright (C) 2024");
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
