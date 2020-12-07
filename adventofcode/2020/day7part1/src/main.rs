use regex::Regex;
use std::collections::{HashMap, HashSet, VecDeque};
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

fn solve(lines: Box<dyn Iterator<Item = String>>) -> i64 {
    let mut graph: HashMap<String, HashSet<String>> = HashMap::new();
    let mut visited: HashSet<String> = HashSet::new();
    let line_pattern =
        Regex::new(r"^(?P<color_u>[a-z\s]+) bags contain (?P<insides>[a-z\s\d,]+)\.$").unwrap();
    let insides_pattern = Regex::new(r"^(?P<count>\d+) (?P<color>[a-z\s]+) bags?$").unwrap();
    for line in lines {
        let captured = line_pattern
            .captures(&line)
            .expect(&format!("Failed to parse line {:?}", line));
        let color_u: String = captured["color_u"].to_string();
        let insides = &captured["insides"];
        if insides == "no other bags" {
            continue;
        }
        for part in insides.split(", ") {
            let captured = insides_pattern
                .captures(&part)
                .expect(&format!("Failed to parse part {:?}", part));
            let color_v = captured["color"].to_string();
            let entry: &mut HashSet<String> = graph.entry(color_v).or_default();
            entry.insert(color_u.clone());
        }
    }
    let mut q: VecDeque<String> = VecDeque::new();
    let start: String = String::from("shiny gold");
    visited.insert(start.clone());
    q.push_back(start);
    while let Some(u) = q.pop_front() {
        for v in graph.entry(u).or_default().iter() {
            if !visited.contains(v) {
                visited.insert(v.clone());
                q.push_back(v.clone());
            }
        }
    }
    return visited.len() as i64 - 1;
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
        let test_input = r#"light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags."#;
        let it = test_input
            .split('\n')
            .into_iter()
            .map(|part| part.to_string());
        assert_eq!(solve(Box::new(it)), 4);
    }
}
