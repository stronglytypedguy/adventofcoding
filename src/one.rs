use std::collections::HashMap;

#[derive(Debug)]
struct Pair(i32, i32);

pub fn solver(input: &String) -> String {
    let numbers: Vec<&str> = input.split("\n").collect();
    let pair: Pair = find_pair(numbers);
    let answer: String = (pair.0 * pair.1).to_string();
    println!("{:#?}, {:#?}, {:#?}", pair.0, pair.1, answer);
    return answer;
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