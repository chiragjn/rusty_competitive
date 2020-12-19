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
        let rules = unresolved_rules.remove(&n).unwrap(); // safe only if there are no loops in rules
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

fn matches_rule_zero(
    line: &str,
    exact42_enabled: bool,
    pattern42_exact: &Regex,
    pattern42_prefix: &Regex,
    pattern31_suffix: &Regex,
) -> bool {
    // we are trying to match 42{x}42{y}31{x} where x and y are some integers
    if exact42_enabled && pattern42_exact.is_match(line) {
        return true;
    }
    if let (Some(lm), Some(rm)) = (pattern42_prefix.find(line), pattern31_suffix.find(line)) {
        return matches_rule_zero(
            &line[lm.end()..rm.start()],
            true,
            pattern42_exact,
            pattern42_prefix,
            pattern31_suffix,
        );
    }
    return false;
}

fn solve(mut lines: Box<dyn Iterator<Item = String>>) -> i64 {
    let mut answer = 0;
    let (mut resolved_rules, mut unresolved_rules) = parse_rules(&mut lines);
    unresolved_rules.insert(8, vec![vec![42], vec![42, 8]]);
    unresolved_rules.insert(11, vec![vec![42, 31], vec![42, 11, 31]]);
    assert_eq!(
        unresolved_rules[&0],
        vec![vec![8, 11]],
        "Rule 0 needs to be '8 11'"
    );
    let rule42 = resolve_rule(42, &mut resolved_rules, &mut unresolved_rules);
    let pattern42_exact = Regex::new(&format!("^(?:{})+$", rule42)).unwrap();
    let pattern42_prefix = Regex::new(&format!("^{}", rule42)).unwrap();
    let rule31 = resolve_rule(31, &mut resolved_rules, &mut unresolved_rules);
    let pattern31_suffix = Regex::new(&format!("{}$", rule31)).unwrap();
    for line in lines {
        if matches_rule_zero(
            &line,
            false,
            &pattern42_exact,
            &pattern42_prefix,
            &pattern31_suffix,
        ) {
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
        let test_input = r#"42: 9 14 | 10 1
9: 14 27 | 1 26
10: 23 14 | 28 1
1: "a"
11: 42 31
5: 1 14 | 15 1
19: 14 1 | 14 14
12: 24 14 | 19 1
16: 15 1 | 14 14
31: 14 17 | 1 13
6: 14 14 | 1 14
2: 1 24 | 14 4
0: 8 11
13: 14 3 | 1 12
15: 1 | 14
17: 14 2 | 1 7
23: 25 1 | 22 14
28: 16 1
4: 1 1
20: 14 14 | 1 15
3: 5 14 | 16 1
27: 1 6 | 14 18
14: "b"
21: 14 1 | 1 14
25: 1 1 | 1 14
22: 14 14
8: 42
26: 14 22 | 1 20
18: 15 15
7: 14 5 | 1 21
24: 14 1

abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa
bbabbbbaabaabba
babbbbaabbbbbabbbbbbaabaaabaaa
aaabbbbbbaaaabaababaabababbabaaabbababababaaa
bbbbbbbaaaabbbbaaabbabaaa
bbbababbbbaaaaaaaabbababaaababaabab
ababaaaaaabaaab
ababaaaaabbbaba
baabbaaaabbaaaababbaababb
abbbbabbbbaaaababbbbbbaaaababb
aaaaabbaabaaaaababaa
aaaabbaaaabbaaa
aaaabbaabbaaaaaaabbbabbbaaabbaabaaa
babaaabbbaaabaababbaabababaaab
aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba"#;
        let it = test_input
            .split('\n')
            .into_iter()
            .map(|part| part.to_string());
        assert_eq!(solve(Box::new(it)), 12);
    }
}
