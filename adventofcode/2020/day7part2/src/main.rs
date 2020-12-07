use regex::Regex;
use std::collections::{HashMap, HashSet};
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

fn _dfs_solver(
    current: &String,
    graph: &HashMap<String, HashSet<(i64, String)>>,
    visited: &mut HashMap<String, i64>,
) -> i64 {
    // relies on the constraint that there is no loop in the input data
    // TODO: Either check for a loop and panic or try sorting topologically first
    let mut answer: i64 = 0;
    if visited.contains_key(current) {
        return visited[current];
    }
    if let Some(entry) = graph.get(current) {
        for (count, inside) in entry.iter() {
            answer += count + count * _dfs_solver(inside, &graph, visited);
        }
    }
    visited.insert(current.clone(), answer);

    return answer;
}

fn solve(lines: Box<dyn Iterator<Item = String>>) -> i64 {
    let mut graph: HashMap<String, HashSet<(i64, String)>> = HashMap::new();
    let mut visited: HashMap<String, i64> = HashMap::new();
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
        let entry: &mut HashSet<(i64, String)> = graph.entry(color_u).or_default();
        for part in insides.split(", ") {
            let captured = insides_pattern
                .captures(&part)
                .expect(&format!("Failed to parse part {:?}", part));
            let count: i64 = captured["count"].parse().expect("Failed to cast to i64");
            let color_v = captured["color"].to_string();
            entry.insert((count, color_v));
        }
    }
    let start: String = String::from("shiny gold");
    return _dfs_solver(&start, &graph, &mut visited);
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
        assert_eq!(solve(Box::new(it)), 32);

        let test_input = r#"shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags."#;
        let it = test_input
            .split('\n')
            .into_iter()
            .map(|part| part.to_string());
        assert_eq!(solve(Box::new(it)), 126);
    }
}
