use std::{
    collections::HashMap,
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

fn decrement_remove(chars: &mut HashMap<char, u32>, c: char) {
    *chars
        .get_mut(&c)
        .unwrap_or_else(|| panic!("Map does not contain {c}")) -= 1;
    if *chars.get(&c).unwrap() == 0 {
        chars.remove(&c);
    }
}

fn solve(mut lines: Box<dyn Iterator<Item = String>>) -> u64 {
    let mut map: HashMap<char, u32> = HashMap::new();
    let chars: Vec<char> = lines.next().expect("Input is empty!").chars().collect();
    for (i, c) in chars.iter().enumerate() {
        *map.entry(*c).or_insert(0) += 1;
        if i >= 14 {
            decrement_remove(&mut map, chars[i - 14]);
        }
        if i >= 13 && map.len() == 14 {
            return (i + 1) as u64;
        }
    }
    0
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
        let test_input = r#"mjqjpqmgbljsphdztnvjfqwrcgsmlb"#;
        let it = test_input
            .split('\n')
            .into_iter()
            .map(|part| part.to_string());
        assert_eq!(solve(Box::new(it)), 19);
    }
}
