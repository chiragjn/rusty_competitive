use std::collections::{HashMap, HashSet};
use std::io::{self, BufRead, Stdin};

struct InputUtils {
    stream: Stdin,
}

impl Default for InputUtils {
    fn default() -> Self {
        return Self {
            stream: io::stdin(),
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

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Node<'a> {
    START,
    UPPER(&'a str),
    LOWER(&'a str),
    END,
}

fn to_node<'a>(s: &'a str) -> Node<'a> {
    if s == "start" {
        return Node::START;
    } else if s == "end" {
        return Node::END;
    } else if s.to_uppercase() == s {
        return Node::UPPER(s);
    } else {
        return Node::LOWER(s);
    }
}

// lifetimes madness
fn traverse<'a>(
    node: &'a Node<'a>,
    visited: &mut HashSet<&'a Node<'a>>,
    graph: &'a HashMap<Node<'a>, Vec<Node<'a>>>,
) -> u64 {
    if *node == Node::END {
        return 1;
    } else {
        if let Node::LOWER(_) | Node::START = node {
            visited.insert(node);
        }

        let mut paths = 0;
        for neighbor in graph[node].iter() {
            if !visited.contains(neighbor) {
                paths += traverse(neighbor, visited, graph);
            }
        }

        if let Node::LOWER(_) | Node::START = node {
            visited.remove(node);
        }

        return paths;
    }
}

fn solve(lines: Box<dyn Iterator<Item = String>>) -> u64 {
    let mut graph: HashMap<Node, Vec<Node>> = HashMap::new();
    let lines: Vec<String> = lines.collect();
    for line in lines.iter() {
        let mut parts = line.trim().split('-');
        let u = to_node(
            parts
                .next()
                .expect(&format!("Failed to split by line {} '-'", line)),
        );
        let v = to_node(
            parts
                .next()
                .expect(&format!("Failed to split by line {} '-'", line)),
        );
        graph.entry(u.clone()).or_default().push(v.clone());
        graph.entry(v).or_default().push(u);
    }
    let mut visited: HashSet<&Node> = HashSet::new();
    return traverse(&Node::START, &mut visited, &graph);
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
    fn test_small() {
        let test_input = r#"start-A
start-b
A-c
A-b
b-d
A-end
b-end"#;
        let it = test_input
            .split('\n')
            .into_iter()
            .map(|part| part.to_string());
        assert_eq!(solve(Box::new(it)), 10);
    }

    #[test]
    fn test_medium() {
        let test_input = r#"dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc"#;
        let it = test_input
            .split('\n')
            .into_iter()
            .map(|part| part.to_string());
        assert_eq!(solve(Box::new(it)), 19);
    }

    #[test]
    fn test_big() {
        let test_input = r#"fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW"#;
        let it = test_input
            .split('\n')
            .into_iter()
            .map(|part| part.to_string());
        assert_eq!(solve(Box::new(it)), 226);
    }
}
