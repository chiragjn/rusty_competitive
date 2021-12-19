use core::fmt;
use std::{
    io::{self, BufRead, Stdin},
    ops,
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

struct ExplodeResult {
    done: bool,
    a: Option<u64>,
    b: Option<u64>,
}

#[derive(Debug, Clone)]
enum NodeType {
    Internal,
    Terminal(u64),
}

#[derive(Debug, Clone)]
struct Node {
    value: NodeType,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    fn is_terminal_pair_parent(&self) -> Option<(u64, u64)> {
        if let (Some(&NodeType::Terminal(a)), Some(&NodeType::Terminal(b))) = (
            self.left.as_ref().map(|l| &l.value),
            self.right.as_ref().map(|r| &r.value),
        ) {
            return Some((a, b));
        }
        None
    }

    fn _update_left_most(&mut self, x: u64) {
        match self.value {
            NodeType::Terminal(v) => {
                self.value = NodeType::Terminal(v + x);
            }
            NodeType::Internal => {
                if let Some(left) = self.left.as_mut() {
                    left._update_left_most(x);
                } else if let Some(right) = self.right.as_mut() {
                    right._update_left_most(x);
                }
            }
        }
    }

    fn _update_right_most(&mut self, x: u64) {
        match self.value {
            NodeType::Terminal(v) => {
                self.value = NodeType::Terminal(v + x);
            }
            NodeType::Internal => {
                if let Some(right) = self.right.as_mut() {
                    right._update_right_most(x);
                } else if let Some(left) = self.left.as_mut() {
                    left._update_right_most(x);
                }
            }
        }
    }

    fn _split(&mut self) -> bool {
        if let NodeType::Terminal(v) = self.value {
            if v >= 10 {
                self.value = NodeType::Internal;
                self.left = Some(Box::new(Node {
                    value: NodeType::Terminal(v / 2),
                    left: None,
                    right: None,
                }));
                self.right = Some(Box::new(Node {
                    value: NodeType::Terminal((v as f64 / 2.0).ceil().round() as u64),
                    left: None,
                    right: None,
                }));
                return true;
            }
        }
        false
    }

    fn _try_split(&mut self) -> bool {
        let mut done = false;
        match self.value {
            NodeType::Terminal(_) => {
                done = self._split();
            }
            NodeType::Internal => {
                if let Some(left) = self.left.as_mut() {
                    done = left._try_split();
                }
                if !done {
                    if let Some(right) = self.right.as_mut() {
                        done = right._try_split();
                    }
                }
            }
        }
        done
    }

    fn _try_explode(&mut self, level: usize) -> ExplodeResult {
        let mut explode_result = ExplodeResult {
            done: false,
            a: None,
            b: None,
        };
        if let (true, Some((a, b))) = (level >= 4, self.is_terminal_pair_parent()) {
            self.value = NodeType::Terminal(0);
            self.left = None;
            self.right = None;
            explode_result.done = true;
            explode_result.a = Some(a);
            explode_result.b = Some(b);
        } else {
            if let Some(left) = self.left.as_mut() {
                explode_result = left._try_explode(level + 1);
                if let (Some(bv), Some(right)) = (explode_result.b, self.right.as_mut()) {
                    right._update_left_most(bv);
                    explode_result.b = None;
                }
            }
            if !explode_result.done {
                if let Some(right) = self.right.as_mut() {
                    explode_result = right._try_explode(level + 1);
                    if let (Some(av), Some(left)) = (explode_result.a, self.left.as_mut()) {
                        left._update_right_most(av);
                        explode_result.a = None;
                    }
                }
            }
        }
        explode_result
    }

    fn reduce(&mut self) {
        while self._try_explode(0).done || self._try_split() {}
    }

    fn sum(&self) -> u64 {
        match self.value {
            NodeType::Terminal(v) => v,
            NodeType::Internal => {
                3 * self.left.as_ref().map(|c| c.sum()).unwrap_or(0)
                    + 2 * self.right.as_ref().map(|c| c.sum()).unwrap_or(0)
            }
        }
    }
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.value {
            NodeType::Terminal(v) => {
                write!(f, "{}", v)?;
            }
            NodeType::Internal => {
                write!(f, "[")?;
                if let Some(left) = self.left.as_ref() {
                    write!(f, "{}", left)?;
                }
                write!(f, ",")?;
                if let Some(right) = self.right.as_ref() {
                    write!(f, "{}", right)?;
                }
                write!(f, "]")?;
            }
        }
        Ok(())
    }
}

impl TryFrom<&str> for Node {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, String> {
        // there's got to be a better way than writing your own parser?
        let mut nodes: Vec<Node> = vec![];
        let mut last_complete_node: Option<Node> = None;
        for (i, c) in value.chars().enumerate() {
            match c {
                '[' => {
                    nodes.push(Self {
                        value: NodeType::Internal,
                        left: None,
                        right: None,
                    });
                }
                ',' => {
                    nodes
                        .last_mut()
                        .ok_or("Invalid input sequence found ',' without a left part of the pair")?
                        .left = last_complete_node.map(Box::new);
                    last_complete_node = None;
                }
                ']' => {
                    let mut parent = nodes
                        .pop()
                        .ok_or("Invalid input sequence found ']' without a '[' before it")?;
                    parent.right = last_complete_node.map(Box::new);
                    last_complete_node = Some(parent);
                }
                d if d.is_numeric() => {
                    last_complete_node = Some(Self {
                        value: NodeType::Terminal(d.to_digit(10).unwrap() as u64),
                        left: None,
                        right: None,
                    });
                }
                _ => {
                    return Err(format!(
                        "Invalid character found in input string {:?} at pos {}",
                        value, i
                    ));
                }
            }
        }
        if last_complete_node.is_none() || !nodes.is_empty() {
            return Err(
                "Invalid input sequence, extra trailing characters or incomplete sequence"
                    .to_string(),
            );
        }
        if let Some(ln) = last_complete_node.as_mut() {
            ln.reduce();
        }
        Ok(last_complete_node.unwrap())
    }
}

impl ops::Add for Node {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let mut added = Node {
            value: NodeType::Internal,
            left: Some(Box::new(self)),
            right: Some(Box::new(other)),
        };
        added.reduce();
        added
    }
}

fn solve(lines: Box<dyn Iterator<Item = String>>) -> u64 {
    let nodes: Vec<Node> = lines
        .map(|line| Node::try_from(line.trim()).unwrap())
        .collect();
    let mut pair_sums: Vec<u64> = vec![];
    for i in 0..nodes.len() {
        for j in 0..nodes.len() {
            if i != j {
                pair_sums.push((nodes[i].clone() + nodes[j].clone()).sum());
            }
        }
    }
    pair_sums.into_iter().max().unwrap_or(0)
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
        let test_input = r#"[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]"#;
        let it = test_input
            .split('\n')
            .into_iter()
            .map(|part| part.to_string());
        assert_eq!(solve(Box::new(it)), 3993);
    }
}
