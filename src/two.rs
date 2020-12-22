#[derive(Debug)]
struct Rule {
    min: i32,
    max: i32,
    letter: char,
    password: String,
}

pub fn solver(input: &String) -> String {
    let rules: Vec<&str> = input.split("\n").collect();
    let parsed_rules: Vec<Rule> = parse_rules(rules);
    let count: i32 = count_valid_passwords(parsed_rules);
    let answer: String = count.to_string();
    return answer;
}

fn parse_rules(numbers: Vec<&str>) -> Vec<Rule> {
    panic!("Not Implemented Yet")
}

fn count_valid_passwords(rules: Vec<Rule>) -> i32 {
    panic!("Not Implemented Yet")
}