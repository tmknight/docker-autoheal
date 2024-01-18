use chrono::Local;
use std::io::{stdout, Write};

// Return binary information
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
    println!("This is free software; you are free to change and redistribute it.");
    println!("There is NO WARRANTY, to the extent permitted by law.");
}

// Logging
pub async fn log_message(msg: &str) {
    let date = Local::now().format("%Y-%m-%d %H:%M:%S%z").to_string();
    let mut lock = stdout().lock();
    writeln!(lock, "{} {}", date, msg).unwrap();
}

// todo
// Webhook
// pub async fn webhook (msg: &str) {
//     let date = Local::now().format("%Y-%m-%d %H:%M:%S%z").to_string();
//     msg;
// }
