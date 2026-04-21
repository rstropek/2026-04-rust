use std::{fmt::Display, str::Utf8Error};

use serde_json::{Value, from_str};
use thiserror::Error;
use tokio::{fs::File, io::AsyncReadExt};

async fn read_and_parse(file_name: &'static str) -> String {
    let mut f = File::open(file_name).await.unwrap();
    let mut buf = Vec::new();
    f.read_buf(&mut buf).await.unwrap();
    let s = String::from_utf8(buf).unwrap();
    let v: Value = from_str(&s).unwrap();
    let setting = v.get("bla").unwrap();
    setting.to_string()
}

#[derive(Debug, Error)]
enum ReadAndParseError {
    #[error("File error: {0}")]
    FileError(#[from] std::io::Error),
    #[error("UTF-8 error: {0}")]
    Utf8Error(#[from] std::string::FromUtf8Error),
    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),
    #[error("Missing key error")]
    MissingKeyError,
}

async fn read_and_parse_better(file_name: &'static str) -> Result<String, ReadAndParseError> {
    let mut f = File::open(file_name).await?;
    let mut buf = Vec::new();
    f.read_buf(&mut buf).await?;
    let s = String::from_utf8(buf)?;
    let v: Value = from_str(&s)?;
    let setting = v.get("bla").ok_or(ReadAndParseError::MissingKeyError)?;
    Ok(setting.to_string())
}

async fn read_and_parse_anyhow(file_name: &'static str) -> anyhow::Result<String> {
    let mut f = File::open(file_name).await?;
    let mut buf = Vec::new();
    f.read_buf(&mut buf).await?;
    let s = String::from_utf8(buf)?;
    let v: Value = from_str(&s)?;
    let setting = v.get("bla").ok_or(anyhow::anyhow!("missing key"))?;
    Ok(setting.to_string())
}

#[tokio::main]
async fn main() {
    let result = read_and_parse_better("conig.json").await;
    match result {
        Ok(setting) => println!("Setting: {}", setting),
        Err(e) => match e {
            ReadAndParseError::FileError(err) => println!("File error: {}", err),
            ReadAndParseError::Utf8Error(err) => println!("UTF-8 error: {}", err),
            e => println!("Other error: {}", e),
        },
    }
}
