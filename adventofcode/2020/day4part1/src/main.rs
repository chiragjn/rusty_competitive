use std::collections::HashSet;
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

fn is_valid(found_keys: &HashSet<String>, required_keys: &Vec<String>) -> bool {
    return required_keys.iter().all(|k| found_keys.contains(k));
}

fn solve(lines: Box<dyn Iterator<Item = String>>) -> i64 {
    let required_keys: Vec<String> = vec![
        String::from("byr"),
        String::from("iyr"),
        String::from("eyr"),
        String::from("hgt"),
        String::from("hcl"),
        String::from("ecl"),
        String::from("pid"),
    ];

    let mut found_keys: HashSet<String> = HashSet::new();
    let mut answer = 0;
    for line in lines {
        if line.is_empty() {
            if is_valid(&found_keys, &required_keys) {
                answer += 1;
            }
            found_keys.clear();
        } else {
            for part in line.split_whitespace() {
                let key = part
                    .split(':')
                    .next()
                    .expect(&format!("failed to split k:v part {}", part));
                found_keys.insert(key.to_string());
            }
        }
    }
    if is_valid(&found_keys, &required_keys) {
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
        let test_input = r#"ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in"#;
        let it = test_input
            .split('\n')
            .into_iter()
            .map(|part| part.to_string());
        assert_eq!(solve(Box::new(it)), 2);
    }
}
