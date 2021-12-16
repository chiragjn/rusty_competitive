use std::collections::{HashMap, HashSet};
use std::io::{self, BufRead, Stdin};

struct InputUtils {
    stream: Stdin,
}

impl Default for InputUtils {
    fn default() -> Self {
        Self {
            stream: io::stdin(),
        }
    }
}

impl Iterator for InputUtils {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        self.stream.lock().lines().next().map(|line| line.unwrap().trim().to_string())
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Node<'a> {
    START,
    UPPER(&'a str),
    LOWER(&'a str),
    END,
    PLACEHOLDER,
}

fn to_node<'a>(s: &'a str) -> Node<'a> {
    if s == "start" {
        Node::START
    } else if s == "end" {
        Node::END
    } else if s.to_uppercase() == s {
        Node::UPPER(s)
    } else {
        Node::LOWER(s)
    }
}

fn node_to_str<'a>(node: &'a Node) -> &'a str {
    match node {
        Node::START => {
            "start"
        }
        Node::END => {
            "end"
        }
        Node::LOWER(x) | Node::UPPER(x) => {
            x
        }
        Node::PLACEHOLDER => {
            unreachable!("Node::PLACEHOLDER not supported");
        }
    }
}

// lifetimes madness
struct PathsCounter<'a> {
    graph: &'a HashMap<Node<'a>, Vec<Node<'a>>>,
    visited: HashSet<&'a Node<'a>>,
    twice_allowed_for: &'a Node<'a>,
    visited_count: usize,
    buffer: Vec<&'a str>,
    paths: HashSet<String>,
}

impl<'a> PathsCounter<'a> {
    fn new(graph: &'a HashMap<Node<'a>, Vec<Node<'a>>>) -> Self {
        PathsCounter {
            graph,
            visited: HashSet::new(),
            twice_allowed_for: &Node::PLACEHOLDER,
            visited_count: 0,
            buffer: vec![],
            paths: HashSet::new(),
        }
    }

    fn _traverse(&mut self, node: &'a Node<'a>) -> u64 {
        if *node == Node::END {
            self.paths.insert(self.buffer.join("-"));
            1
        } else {
            self.buffer.push(node_to_str(node));
            if node == self.twice_allowed_for {
                self.visited_count += 1;
                if self.visited_count == 2 {
                    self.visited.insert(node);
                }
            } else if let Node::LOWER(_) | Node::START = node {
                self.visited.insert(node);
            }

            let mut paths = 0;
            for neighbor in self.graph[node].iter() {
                if !self.visited.contains(neighbor) {
                    paths += self._traverse(neighbor);
                }
            }

            if node == self.twice_allowed_for {
                self.visited_count -= 1;
                self.visited.remove(node);
            } else if let Node::LOWER(_) | Node::START = node {
                self.visited.remove(node);
            }

            self.buffer.pop();

            paths
        }
    }

    fn count(&mut self) -> u64 {
        for node in self.graph.keys() {
            if let Node::LOWER(_) = node {
                self.twice_allowed_for = node;
                self.visited_count = 0;
                self._traverse(&Node::START);
            }
        }
        self.paths.len() as u64
    }
}

fn solve(lines: Box<dyn Iterator<Item = String>>) -> u64 {
    let lines: Vec<String> = lines.collect();
    let mut graph: HashMap<Node, Vec<Node>> = HashMap::new();
    for line in lines.iter() {
        let mut parts = line.trim().split('-');
        let u = to_node(
            parts
                .next()
                .unwrap_or_else(|| panic!("Failed to split by line {} '-'", line)),
        );
        let v = to_node(
            parts
                .next()
                .unwrap_or_else(|| panic!("Failed to split by line {} '-'", line)),
        );
        graph.entry(u).or_default().push(v);
        graph.entry(v).or_default().push(u);
    }
    return PathsCounter::new(&graph).count();
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
        assert_eq!(solve(Box::new(it)), 36);
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
        assert_eq!(solve(Box::new(it)), 103);
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
        assert_eq!(solve(Box::new(it)), 3509);
    }
}
