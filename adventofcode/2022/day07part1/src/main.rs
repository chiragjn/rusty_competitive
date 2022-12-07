use std::{
    collections::{hash_map::Entry, HashMap},
    io::{self, BufRead, Stdin},
};

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
        self.stream
            .lock()
            .lines()
            .next()
            .map(|line| line.unwrap().trim().to_string())
    }
}

enum NodeType {
    File(u64),
    Dir(Node),
}

struct Node {
    parent: usize,
    children: HashMap<String, usize>,
}

struct FileTree {
    nodes: Vec<NodeType>,
    cwd_pointer: usize,
}

impl FileTree {
    fn default() -> Self {
        Self {
            nodes: vec![NodeType::Dir(Node {
                parent: 0,
                children: HashMap::new(),
            })],
            cwd_pointer: 0,
        }
    }

    fn add_file(&mut self, name: &str, size: u64) {
        let next_node_idx = self.nodes.len();
        match &mut self.nodes[self.cwd_pointer] {
            NodeType::Dir(node) => {
                let key = name.to_string();
                if let Entry::Vacant(e) = node.children.entry(key) {
                    e.insert(next_node_idx);
                    self.nodes.push(NodeType::File(size));
                }
            }
            _ => unreachable!("Expected cwd_pointer to always point to a Dir"),
        }
    }

    fn add_dir(&mut self, name: &str) {
        let next_node_idx = self.nodes.len();
        match &mut self.nodes[self.cwd_pointer] {
            NodeType::Dir(node) => {
                let key = name.to_string();
                if let Entry::Vacant(e) = node.children.entry(key) {
                    e.insert(next_node_idx);
                    self.nodes.push(NodeType::Dir(Node {
                        parent: self.cwd_pointer,
                        children: HashMap::new(),
                    }));
                }
            }
            _ => unreachable!("Expected cwd_pointer to always point to a Dir"),
        }
    }

    fn change_dir(&mut self, to: &str) {
        match to {
            "/" => self.cwd_pointer = 0,
            path => {
                self.cwd_pointer = match &mut self.nodes[self.cwd_pointer] {
                    NodeType::Dir(node) => match path {
                        ".." => node.parent,
                        _ => *node
                            .children
                            .get(&to.to_string())
                            .unwrap_or_else(|| panic!("No directory {to} registered")),
                    },
                    _ => unreachable!("Expected cwd_pointer to always point to a Dir"),
                }
            }
        }
    }

    fn computed_dir_sizes(&self) -> Vec<u64> {
        let mut sizes: Vec<u64> = vec![0; self.nodes.len()];
        for (i, item) in self.nodes.iter().enumerate().rev() {
            match item {
                NodeType::Dir(node) => {
                    for &c in node.children.values() {
                        sizes[i] += sizes[c];
                    }
                }
                NodeType::File(filesize) => {
                    sizes[i] = *filesize;
                }
            }
        }
        sizes
            .iter()
            .zip(self.nodes.iter())
            .filter(|(_, nt)| matches!(nt, NodeType::Dir(_)))
            .map(|(&size, _)| size)
            .collect()
    }
}

fn solve(lines: Box<dyn Iterator<Item = String>>) -> u64 {
    let mut file_tree: FileTree = FileTree::default();
    for line in lines {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if let Some(&"$") = parts.first() {
            match parts[1] {
                "cd" => file_tree.change_dir(parts[2]),
                "ls" => {}
                _ => unreachable!("Unrecognized command encountered"),
            }
        } else {
            match parts[0] {
                "dir" => file_tree.add_dir(parts[1]),
                filesize => file_tree.add_file(
                    parts[1],
                    filesize
                        .parse::<u64>()
                        .expect("Failed to convert size string to u64"),
                ),
            }
        }
    }
    file_tree
        .computed_dir_sizes()
        .iter()
        .filter(|&&size| size <= 100000)
        .sum()
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
        let test_input = r#"$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k"#;
        let it = test_input
            .split('\n')
            .into_iter()
            .map(|part| part.to_string());
        assert_eq!(solve(Box::new(it)), 95437);
    }
}
