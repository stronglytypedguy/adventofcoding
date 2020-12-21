use reqwest::Client;
use std::collections::HashMap;

#[derive(Debug)]
struct Pair(i32, i32);

pub async fn solver(client: Client) -> Result<(), Box<dyn std::error::Error>> {
    let numbers: String = client
        .get("https://adventofcode.com/2020/day/1/input")
        .send()
        .await?
        .text()
        .await?;
    let numbers: Vec<&str> = numbers.split("\n").collect();
    let pair: Pair = find_pair(numbers);
    let answer: &str = &(pair.0 * pair.1).to_string();
    println!("{:#?}, {:#?}, {:#?}", pair.0, pair.1, answer);
    let params = [("level", "1"), ("answer", answer)];
    let submit = client
        .post("https://adventofcode.com/2020/day/1/answer")
        .form(&params)
        .send()
        .await?
        .text()
        .await?;
    println!("{:#?}", submit);
    Ok(())
}

fn find_pair(numbers: Vec<&str>) -> Pair {
    let mut pair_map = HashMap::new();
    for number_str in numbers {
        let number: i32 = number_str.parse::<i32>().unwrap();
        if pair_map.get(&(2020 - number)) != None {
            return Pair(number, 2020 - number);
        }
        pair_map.insert(
            number,
            true,
        );
    }
    panic!("Pair does not exist.");
}