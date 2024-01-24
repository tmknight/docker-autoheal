use reqwest::Client;
use crate::{log_message, INFO};

pub async fn notify_webhook(url: &str, payload: &str) {
    let client = Client::new();
    let rslt = client
        .post(url)
        .header("Content-Type", "application/json")
        .body(payload.to_string())
        .send()
        .await;

    let msg0 = match rslt {
        Ok(r) => format!("Response: {}", r.status()),
        Err(e) => format!("Response: {}", e),
    };
    // Log result
    log_message(&msg0, INFO).await;
}
