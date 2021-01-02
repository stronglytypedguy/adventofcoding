#[derive(Debug)]
#[derive(PartialEq)]
struct Rule {
    min: i32,
    max: i32,
    letter: String,
    password: String,
}

pub fn solver(input: &String) -> String {
    println!("Starting Two");
    let rules: Vec<&str> = input.split("\n").collect();
    let parsed_rules: Vec<Rule> = parse_rules(rules);
    let count: Vec<i32> = count_valid_passwords(parsed_rules);
    let answer: String = "Part 1: ".to_string() + &count[0].to_string() + " " +  "Part 2: " + &count[1].to_string();
    println!("Finished Two, Answer: {}", answer);
    return answer;
}

fn parse_rules(lines: Vec<&str>) -> Vec<Rule> {
    let mut res: Vec<Rule> = vec![];
    for line in lines {
        if line.len() > 5 {
            res.push(create_rule_from_line(line));
        }
    }
    return res
}

fn create_rule_from_line(line: &str) -> Rule {
    let first_split: Vec<&str> = line.split("-").collect();
    let min = first_split[0].parse::<i32>().unwrap();
    let second_split: Vec<&str> = first_split[1].split(" ").collect();
    let max = second_split[0].parse::<i32>().unwrap();
    let letter = second_split[1].split::<&str>("").collect::<Vec<&str>>()[1].to_string();
    let password = second_split[2].to_string();
    return Rule {
        min,
        max,
        letter,
        password
    };
}

fn count_valid_passwords(rules: Vec<Rule>) -> Vec<i32> {
    let mut count_one = 0;
    let mut count_two = 0;
    for rule in rules {
        if is_valid_password(&rule) {
            count_one += 1;
        }
        if is_valid_password2(&rule) {
            count_two += 1;
        }
    }
    return vec![count_one, count_two]
}

fn is_valid_password2(rule: &Rule) -> bool {
    let password_letters: Vec<&str> = rule.password.split("").collect();
    return (password_letters[(rule.min) as usize] == rule.letter) ^ (password_letters[(rule.max) as usize] == rule.letter) //Note: Don't have to subtract one become string array starts with ""
}

fn is_valid_password(rule: &Rule) -> bool {
    let mut count = 0;
    let password_letters: Vec<&str> = rule.password.split("").collect();
    for letter in password_letters {
        if letter == rule.letter {
            count += 1;
        }
    }
    return rule.min <= count && count <= rule.max
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_parse_rule() {
        let rule = Rule {
            min: 6,
            max: 10,
            letter: "s".to_string(),
            password: "snkscgszxsssscss".to_string()
        };
        assert_eq!(create_rule_from_line("6-10 s: snkscgszxsssscss"), rule);
    }

    #[test]
    fn test_parse_rules() {
        let rule = Rule {
            min: 6,
            max: 10,
            letter: "s".to_string(),
            password: "snkscgszxsssscss".to_string()
        };
        let rule2 = Rule {
            min: 6,
            max: 7,
            letter: "b".to_string(),
            password: "bbbbbxkb".to_string()
        };
        assert_eq!(parse_rules(vec!["6-10 s: snkscgszxsssscss", "6-7 b: bbbbbxkb"]), vec![rule, rule2]);
    }

    #[test]
    fn test_is_valid_password() {
        let too_many = Rule {
            min: 6,
            max: 7,
            letter: "s".to_string(),
            password: "snkscgszxsssscss".to_string()
        };
        let too_few = Rule {
            min: 6,
            max: 7,
            letter: "s".to_string(),
            password: "snkscgszx".to_string()
        };
        let just_right = Rule {
            min: 6,
            max: 7,
            letter: "b".to_string(),
            password: "bbbbbxkb".to_string()
        };
        assert_eq!(is_valid_password(too_many), false);
        assert_eq!(is_valid_password(too_few), false);
        assert_eq!(is_valid_password(just_right), true);
    }

    #[test]
    fn test_count_valid_passwords() {
        let too_many = Rule {
            min: 6,
            max: 7,
            letter: "s".to_string(),
            password: "snkscgszxsssscss".to_string()
        };
        let too_few = Rule {
            min: 6,
            max: 7,
            letter: "s".to_string(),
            password: "snkscgszx".to_string()
        };
        let just_right = Rule {
            min: 6,
            max: 7,
            letter: "b".to_string(),
            password: "bbbbbxkb".to_string()
        };
        assert_eq!(count_valid_passwords(vec![too_many, too_few, just_right]), 1);
    }

    #[test]
    fn test_is_valid_password2() {
        let good = Rule {
            min: 1,
            max: 3,
            letter: "a".to_string(),
            password: "abcde".to_string()
        };
        let bad_none = Rule {
            min: 1,
            max: 3,
            letter: "b".to_string(),
            password: "cdefg".to_string()
        };
        let bad_both = Rule {
            min: 2,
            max: 9,
            letter: "c".to_string(),
            password: "ccccccccc".to_string()
        };
        assert_eq!(is_valid_password2(bad_none), false);
        assert_eq!(is_valid_password2(bad_both), false);
        assert_eq!(is_valid_password2(good), true);
    }
}