use regex::Regex;
use std::collections::HashMap;
use std::io::{self, BufRead, Read, Stdin};

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

fn sum_of_invalids(field2ranges: &HashMap<String, [(i64, i64); 2]>, values: &Vec<i64>) -> i64 {
    let mut sum = 0;
    'outer: for &value in values {
        for &ranges in field2ranges.values() {
            for &range in ranges.iter() {
                if value >= range.0 && value <= range.1 {
                    continue 'outer;
                }
            }
        }
        sum += value;
    }
    return sum;
}

fn solve(mut lines: Box<dyn Iterator<Item = String>>) -> i64 {
    let mut answer = 0;
    let mut field2ranges: HashMap<String, [(i64, i64); 2]> = HashMap::new();
    let pattern = Regex::new(
        r"(?P<field>[a-z\s]+): (?P<low1>\d+)-(?P<high1>\d+) or (?P<low2>\d+)-(?P<high2>\d+)",
    )
    .unwrap();
    loop {
        if let Some(sline) = lines.next() {
            if sline == "" {
                break;
            }
            let captured = pattern
                .captures(&sline)
                .expect(&format!("Failed to parse line {:?}", sline));
            let low1: i64 = captured["low1"].parse().expect("Failed to cast to int");
            let high1: i64 = captured["high1"].parse().expect("Failed to cast to int");
            let low2: i64 = captured["low2"].parse().expect("Failed to cast to int");
            let high2: i64 = captured["high2"].parse().expect("Failed to cast to int");
            field2ranges.insert(
                captured["field"].to_string(),
                [(low1, high1), (low2, high2)],
            );
        } else {
            unreachable!("Input terminated after first empty line!")
        }
    }
    lines.next();
    let _: Vec<i64> = lines
        .next()
        .expect("Failed to get values for 'your ticket'")
        .split(",")
        .map(|part| {
            part.parse()
                .expect(&format!("failed to parse {} to i64", part))
        })
        .collect();
    lines.next();
    lines.next();
    for line in lines {
        let values: Vec<i64> = line
            .split(",")
            .map(|part| {
                part.parse()
                    .expect(&format!("failed to parse {} to i64", part))
            })
            .collect();
        answer += sum_of_invalids(&field2ranges, &values);
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
        let test_input = r#"class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12"#;
        let it = test_input
            .split('\n')
            .into_iter()
            .map(|part| part.to_string());
        assert_eq!(solve(Box::new(it)), 71);
    }
}
