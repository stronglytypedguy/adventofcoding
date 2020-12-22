use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::Client;
use std::env::{var};
mod one;
mod two;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let session_id = var("SESSION_ID").unwrap();
    let mut headers = HeaderMap::new();
    let session_cookie: HeaderValue = HeaderValue::from_str(&format!("session={}", session_id)).unwrap();
    let cookie_header = "Cookie";
    headers.insert(cookie_header, session_cookie);
    let client = Client::builder()
        .default_headers(headers)
        .build()?;
    let mut user_input = String::new();
    println!("Enter advent day:");
    std::io::stdin().read_line(&mut user_input).unwrap();
    println!("User inputted {}", user_input);
    let parsed_input = user_input.split("").collect::<Vec<&str>>()[1];
    println!("{:#?}", parsed_input);
    let i = parsed_input.parse::<i32>().unwrap();
    let challenge_input: String = client
        .get(&format!("https://adventofcode.com/2020/day/{}/input", &user_input))
        .send()
        .await?
        .text()
        .await?;
    let answer: String;
    if i == 1 {
        answer = one::solver(&challenge_input);
    } else if i == 2 {
        answer = two::solver(&challenge_input);
    } else {
        panic!("Have not implemented solution");
    }
    let params = [("level", &user_input), ("answer", &answer)];
    let submit = client
        .post(&format!("https://adventofcode.com/2020/day/{}/answer", &user_input))
        .form(&params)
        .send()
        .await?
        .text()
        .await?;
    println!("{:#?}", submit);
    Ok(())
}


