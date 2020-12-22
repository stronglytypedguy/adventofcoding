pub fn solver(input: &String) -> String {
    let lines: Vec<&str> = input.split("\n").collect();
    let mut count = 0;
    let mut passport_check = vec![0, 0, 0, 0, 0, 0, 0];
    for line in lines {
        let split: Vec<&str> = line.split(" ").collect();
        let mut made_change = false;
        for field in split {
            if field != "" {
                made_change = true;
                let passport_field: Vec<&str> = field.split(":").collect();
                let key = retrieve_passport_field_mapping(passport_field[0], passport_field[1]);
                if key < passport_check.len() {
                    passport_check[key] = 1;
                }
            }
        }
        if !made_change {
            if !passport_check.contains(&0) {
                count += 1;
            }
            passport_check = vec![0, 0, 0, 0, 0, 0, 0]
        }
    }
    let answer: String = count.to_string();
    return answer;
}

fn retrieve_passport_field_mapping(input: &str, value: &str) -> usize {
    match input {
        "byr" => if check_byr(value) { return 0 } else { return 7 },
        "iyr" => if check_iyr(value) { return 1 } else { return 7 },
        "eyr" => if check_eyr(value) { return 2 } else { return 7 },
        "hgt" => if check_hgt(value) { return 3 } else { return 7 },
        "hcl" => if check_hcl(value) { return 4 } else { return 7 },
        "ecl" => if check_ecl(value) { return 5 } else { return 7 },
        "pid" => if check_pid(value) { return 6 } else { return 7 },
        _ => return 7,
    }
}

fn check_byr(val: &str) -> bool {
    let byr = val.parse::<i32>().unwrap();
    return byr >= 1920 && byr <= 2002;
}
fn check_iyr(val: &str) -> bool {
    let iyr = val.parse::<i32>().unwrap();
    return iyr >= 2010 && iyr <= 2020;
}
fn check_eyr(val: &str) -> bool {
    let eyr = val.parse::<i32>().unwrap();
    return eyr >= 2020 && eyr <= 2030;
}

fn check_hgt(val: &str) -> bool {
    // Not working
    if val.contains("cm") {
        let split: Vec<&str> = val.split("cm").collect();
        let id = split[0].parse::<i32>().unwrap();
        return split[1] == "" && id >= 150 && id <= 193
    } else if val.contains("in") {
        let split: Vec<&str> = val.split("in").collect();
        let id = split[0].parse::<i32>().unwrap();
        return split[1] == "" && id >= 59 && id <= 76
    }
    return false;
}
fn check_hcl(val: &str) -> bool {
    let mut characters = val.chars();
    for (i, c) in val.chars().enumerate() {
        if i == 0 {
            match c {
                '#' => continue,
                _ => return false,
            }
        } else {
            match c {
                'a' | 'b' | 'c' | 'd' | 'e' | 'f' => continue,
                '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => continue,
                _ => return false,
            }
        }

    }
    return val.len() == 7; 
}
fn check_ecl(val: &str) -> bool {
    match val {
        "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => return true,
        _ => return false,
    }
}
fn check_pid(val: &str) -> bool {
    for c in val.chars() {
        match c {
            '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => continue,
            _ => return false,
        }
    }
    return val.len() == 9;
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_four_solver() {
        let passports = "eyr:1972 cid:100
        hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926
        
        iyr:2019
        hcl:#602927 eyr:1967 hgt:170cm
        ecl:grn pid:012533040 byr:1946
        
        hcl:dab227 iyr:2012
        ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277
        
        hgt:59cm ecl:zzz
        eyr:2038 hcl:74454a iyr:2023
        pid:3556412378 byr:2007
        
        pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
        hcl:#623a2f

        eyr:2029 ecl:blu cid:129 byr:1989
        iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

        hcl:#888785
        hgt:164cm byr:2001 iyr:2015 cid:88
        pid:545766238 ecl:hzl
        eyr:2022

        iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719
        ".to_string();
        assert_eq!(solver(&passports), "4")
    }
}