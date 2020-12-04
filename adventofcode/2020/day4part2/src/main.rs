use regex::Regex;
use std::collections::HashMap;
use std::io::{self, BufRead, Stdin};

struct InputUtils {
    stream: Stdin,
    buffer: String,
}

impl Default for InputUtils {
    fn default() -> Self {
        return Self {
            stream: io::stdin(),
            buffer: String::new(),
        };
    }
}

impl Iterator for InputUtils {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        match self.stream.lock().lines().next() {
            Some(line) => Some(line.unwrap().trim().to_string()),
            None => None,
        }
    }
}

struct PassportValidator {
    rules: HashMap<String, Regex>,
}

impl PassportValidator {
    fn new() -> Self {
        // forgive me for my regex sins
        let rules: HashMap<String, Regex> = vec![
            (
                String::from("byr"),
                Regex::new(r"^(19[2-9][0-9])|(200[0-2])$").unwrap(),
            ),
            (
                String::from("iyr"),
                Regex::new(r"^(201[0-9])|(2020)$").unwrap(),
            ),
            (
                String::from("eyr"),
                Regex::new(r"^(202[0-9])|(2030)$").unwrap(),
            ),
            (
                String::from("hgt"),
                Regex::new(r"^(((1[5-9][0-9])|(19[0-3]))cm)|(((59)|(6[0-9])|(7[0-6]))in)$")
                    .unwrap(),
            ),
            (String::from("hcl"), Regex::new(r"^#[0-9a-f]{6}$").unwrap()),
            (
                String::from("ecl"),
                Regex::new(r"^amb|blu|brn|gry|grn|hzl|oth$").unwrap(),
            ),
            (String::from("pid"), Regex::new(r"^[0-9]{9}$").unwrap()),
        ]
        .into_iter()
        .collect();
        return PassportValidator { rules: rules };
    }
    fn is_valid(&self, data: &HashMap<String, String>) -> bool {
        return self
            .rules
            .iter()
            .all(|(key, rule)| data.contains_key(key) && rule.is_match(&data[key]));
    }
}

fn solve(lines: Box<dyn Iterator<Item = String>>) -> i64 {
    let validator = PassportValidator::new();
    let mut parsed_map: HashMap<String, String> = HashMap::new();
    let mut answer = 0;
    for line in lines {
        if line.is_empty() {
            if validator.is_valid(&parsed_map) {
                answer += 1;
            }
            parsed_map.clear();
        } else {
            for part in line.split_whitespace() {
                let mut kv_parts = part.split(':');
                let key = kv_parts
                    .next()
                    .expect(&format!("failed to get key from k:v part {}", part));
                let value = kv_parts
                    .next()
                    .expect(&format!("failed to get value from k:v part {}", part));
                parsed_map.insert(key.to_string(), value.to_string());
            }
        }
    }
    if validator.is_valid(&parsed_map) {
        answer += 1;
    }
    return answer;
}

fn main() {
    let iu = InputUtils::default();
    let boxed_iter = Box::new(iu);
    println!("{}", solve(boxed_iter));
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test() {
        let test_input = r#"eyr:1972 cid:100
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

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719"#;
        let it = test_input
            .split('\n')
            .into_iter()
            .map(|part| part.to_string());
        assert_eq!(solve(Box::new(it)), 4);
    }
}
