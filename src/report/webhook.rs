use crate::{log_message, INFO};
use reqwest::Client;

pub async fn notify_webhook(url: &str, payload: &str) {
    let client = Client::new();
    let resp = client
        .post(url)
        .header("Content-Type", "application/json")
        .body(payload.to_string())
        .send()
        .await;

    let msg0 = match resp {
        Ok(r) => format!("Response ({:?}): {}", r.url(), r.status()),
        Err(e) => format!("Response ({:?}): {}", e.url(), e),
    };
    // Log result
    log_message(&msg0, INFO).await;
}
