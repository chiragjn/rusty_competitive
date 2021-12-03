use std::{
    io::{self, BufRead, Stdin},
    slice::SliceIndex,
};

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

// TODO: Refactor. Not the neatest way to do things
#[derive(Debug)]
struct BinTrieNode {
    counts: [u32; 2],
    children: [Option<Box<BinTrieNode>>; 2],
}

impl BinTrieNode {
    fn new() -> Self {
        return BinTrieNode {
            counts: [0, 0],
            children: [None, None],
        };
    }

    fn insert(&mut self, string: &[char]) {
        if string.len() == 0 {
            return;
        }
        let i: usize = string[0]
            .to_digit(10)
            .expect("Failed to cast char to usize") as usize;
        self.counts[i] += 1;
        if i < self.children.len() {
            if string.len() > 1 {
                if self.children[i].is_none() {
                    self.children[i] = Some(Box::new(BinTrieNode::new()));
                }
                self.children[i].as_mut().unwrap().insert(&string[1..]);
            }
        } else {
            unreachable!("NO!");
        }
    }

    fn get(&self, most_common: bool, answer: &mut i64) {
        let i: usize = if (self.counts[0] + self.counts[1] == 1) || most_common {
            if self.counts[0] > self.counts[1] {
                0
            } else {
                1
            }
        } else {
            if self.counts[1] < self.counts[0] {
                1
            } else {
                0
            }
        };
        *answer = (*answer << 1) | (i as i64);
        if let Some(child) = &self.children[i] {
            child.get(most_common, answer)
        }
    }
}

fn solve(lines: Box<dyn Iterator<Item = String>>) -> i64 {
    let mut root = BinTrieNode::new();
    for line in lines {
        let chars: Vec<char> = line.chars().collect();
        root.insert(&chars[..]);
    }
    let mut o2 = 0;
    root.get(true, &mut o2);
    let mut co2 = 0;
    root.get(false, &mut co2);
    return o2 * co2;
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
        let test_input = r#"00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010"#;
        let it = test_input
            .split('\n')
            .into_iter()
            .map(|part| part.to_string());
        assert_eq!(solve(Box::new(it)), 230);
    }
}
