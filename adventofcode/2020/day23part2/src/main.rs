use std::collections::HashMap;
use std::io::{self, BufRead, Stdin};
use std::iter::once;

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

fn simulate(chain: &mut HashMap<usize, usize>, head: &mut usize, max_n: usize) {
    let link_start = chain[head];
    let mut link_end = *head;
    let mut neqs: Vec<usize> = Vec::with_capacity(3);
    for _ in 0..3 {
        link_end = chain[&link_end];
        neqs.push(link_end);
    }
    chain.insert(*head, chain[&link_end]);
    let mut dest = *head - 1;
    loop {
        if dest == 0 {
            dest = max_n;
        }
        if dest != neqs[0] && dest != neqs[1] && dest != neqs[2] {
            break;
        }
        dest -= 1;
    }
    chain.insert(link_end, chain[&dest]);
    chain.insert(dest, link_start);
    *head = chain[head];
}

fn solve(mut lines: Box<dyn Iterator<Item = String>>) -> u64 {
    // strong assumption that minimum is 1
    let nums: Vec<usize> = lines
        .next()
        .expect("No input!")
        .chars()
        .map(|num_s| {
            num_s
                .to_digit(10)
                .expect(&format!("Failed to cast {} to usize", num_s)) as usize
        })
        .collect();
    let given_max: usize = *nums.iter().max().unwrap();
    let max_n: usize = 1000000;
    let mut chain: HashMap<usize, usize> = HashMap::new();
    let it1 = nums.iter().copied().chain((given_max + 1)..=max_n);
    let it2 = it1.clone().chain(once(nums[0])).skip(1);
    for (from, to) in it1.zip(it2) {
        chain.insert(from, to);
    }
    let mut head = nums[0];
    // this is slow, go take a 2 minutes break
    // a faster way would be to store ranges and break them dynamically into sub ranges when
    // insertions happen, but that is much trickier to implement
    for _ in 0..10000000 {
        simulate(&mut chain, &mut head, max_n);
    }
    return chain[&1] as u64 * chain[&chain[&1]] as u64;
}

fn main() {
    let iu = InputUtils::default();
    let boxed_iter = Box::new(iu);
    println!("{}", solve(boxed_iter));
}

#[cfg(test)]
mod tests {
    use super::{once, simulate, HashMap};
    use crate::solve;

    #[test]
    fn test_simulate() {
        let mut chain: HashMap<usize, usize> = HashMap::new();
        let nums = vec![3, 8, 9, 1, 2, 5, 4, 6, 7];
        let it1 = nums.iter().copied();
        let it2 = it1.clone().chain(once(nums[0])).skip(1);
        for (from, to) in it1.zip(it2) {
            chain.insert(from, to);
        }
        let mut head = nums[0];
        for _ in 0..10 {
            simulate(&mut chain, &mut head, 9);
        }
        let mut new_chain = vec![];
        for _ in 0..nums.len() {
            new_chain.push(head);
            head = chain[&head];
        }
        assert_eq!(new_chain, vec![8, 3, 7, 4, 1, 9, 2, 6, 5])
    }

    #[test]
    fn test() {
        let test_input = r#"389125467"#;
        let it = test_input
            .split('\n')
            .into_iter()
            .map(|part| part.to_string());
        assert_eq!(solve(Box::new(it)), 149245887792);
    }
}
