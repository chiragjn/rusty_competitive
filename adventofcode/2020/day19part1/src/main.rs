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

type ResolvedRules = HashMap<usize, String>;
type UnResolvedRules = HashMap<usize, Vec<Vec<usize>>>;

fn parse_rules(lines: &mut Box<dyn Iterator<Item = String>>) -> (ResolvedRules, UnResolvedRules) {
    let terminal_pattern = Regex::new(r#"(?P<number>\d+): "(?P<char>[a-z])"#).unwrap();
    let non_terminal_pattern = Regex::new(r"(?P<number>\d+): (?P<references>[\d\s\|]+)").unwrap();
    let mut resolved_rules: ResolvedRules = HashMap::new();
    let mut unresolved_rules: UnResolvedRules = HashMap::new();
    while let Some(line) = lines.next() {
        if line.is_empty() {
            break;
        }
        if let Some(captured) = non_terminal_pattern.captures(&line) {
            let mut rules = vec![];
            for rule_str in captured["references"].split(" | ") {
                rules.push(
                    rule_str
                        .split_whitespace()
                        .map(|num_s| num_s.parse().unwrap())
                        .collect(),
                );
            }
            unresolved_rules.insert(captured["number"].parse().unwrap(), rules);
        } else {
            let captured = terminal_pattern.captures(&line).expect(&format!(
                "Failed to capture line {} with terminal pattern",
                line
            ));
            resolved_rules.insert(
                captured["number"].parse().unwrap(),
                captured["char"].to_string(),
            );
        }
    }
    return (resolved_rules, unresolved_rules);
}

fn resolve_rule<'a>(
    n: usize,
    resolved_rules: &'a mut ResolvedRules,
    unresolved_rules: &'a mut UnResolvedRules,
) -> &'a String {
    if !resolved_rules.contains_key(&n) {
        let mut resolved: String = String::from("(?:");
        let rules = unresolved_rules.remove(&n).unwrap(); // safe because there are no loops in rules
        for rule in rules.iter() {
            if rule.is_empty() {
                continue;
            }
            resolved.push_str("(?:");
            for &m in rule {
                resolved.extend(
                    resolve_rule(m, resolved_rules, unresolved_rules)
                        .clone()
                        .chars(),
                );
            }
            resolved.push_str(")|");
        }
        resolved.pop();
        resolved.push_str(")");
        resolved_rules.insert(n, resolved);
    }
    return &resolved_rules[&n];
}

fn solve(mut lines: Box<dyn Iterator<Item = String>>) -> i64 {
    let mut answer = 0;
    let (mut resolved_rules, mut unresolved_rules) = parse_rules(&mut lines);
    let rule_zero = resolve_rule(0, &mut resolved_rules, &mut unresolved_rules);
    let zero_pattern = Regex::new(&format!("^{}$", rule_zero)).unwrap();
    for line in lines {
        if zero_pattern.is_match(&line) {
            answer += 1;
        }
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
        let test_input = r#"0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: "a"
5: "b"

ababbb
bababa
abbbab
aaabbb
aaaabbb"#;
        let it = test_input
            .split('\n')
            .into_iter()
            .map(|part| part.to_string());
        assert_eq!(solve(Box::new(it)), 2);
    }
}
