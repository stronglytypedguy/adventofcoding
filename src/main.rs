use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::Client;
use std::env::{var};
mod one;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let session_id = var("SESSION_ID").unwrap();
    let mut headers = HeaderMap::new();
    let session_cookie: HeaderValue = HeaderValue::from_str(&format!("session={}", session_id)).unwrap();
    println!("{:#?}", session_cookie);
    let cookie_header = "Cookie";
    headers.insert(cookie_header, session_cookie);
    let client = Client::builder()
        .default_headers(headers)
        .build()?;
    let res = one::solver(client).await?;
    println!("{:#?}", res);
    Ok(())
}


