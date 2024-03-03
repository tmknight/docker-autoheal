use crate::{LOG_FILE, LOG_PATH};
use serde::{Deserialize, Serialize};
use tokio::fs::File;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader, Error};

#[derive(Serialize, Deserialize)]
pub struct JsonRecords {
    pub date: String,
    pub name: String,
    pub id: String,
    pub err: String,
    pub action: String,
}

pub async fn read_record() -> Result<Vec<JsonRecords>, Error> {
    let log_file = LOG_PATH.to_owned() + LOG_FILE;
    // Open file
    let mut file = File::open(log_file).await?;
    let file_clone = file.try_clone().await?;
    let reader = BufReader::new(file_clone);
    // Initialize an empty vector to store the records
    let mut records = Vec::new();
    // Create a stream from the reader lines
    let mut lines = reader.lines();
    // Build array of results
    while let Ok(Some(line)) = lines.next_line().await {
        let record = serde_json::from_str(&line).unwrap();
        records.push(record);
    }
    file.flush().await?;
    Ok(records)
}

pub async fn write_record(data: JsonRecords) -> Result<(), Error> {
    let log_file = LOG_PATH.to_owned() + LOG_FILE;
    // Serialize the data to JSON
    let json_data = serde_json::to_string(&data)?;
    // Asynchronously open file for writing
    let mut file = File::options()
        .append(true)
        .create(true)
        .open(log_file)
        .await?;
    // Write the JSON data to file
    file.write_all(json_data.as_bytes()).await?;
    file.write_all(b"\n").await?;
    file.flush().await?;
    Ok(())
}