use std::collections::VecDeque;
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

fn simulate(mut nums: VecDeque<u8>) -> VecDeque<u8> {
    let mut new_nums: VecDeque<u8> = VecDeque::new();
    let top = nums.pop_front().unwrap();
    let one = nums.pop_front().unwrap();
    let two = nums.pop_front().unwrap();
    let three = nums.pop_front().unwrap();
    let max = *nums.iter().max().unwrap();
    let mut dest = top - 1;
    loop {
        if dest == 0 {
            dest = max;
        }
        if dest != one && dest != two && dest != three {
            break;
        }
        dest -= 1;
    }
    for num in nums {
        new_nums.push_back(num);
        if num == dest {
            new_nums.push_back(one);
            new_nums.push_back(two);
            new_nums.push_back(three);
        }
    }
    new_nums.push_back(top);
    return new_nums;
}

fn solve(mut lines: Box<dyn Iterator<Item = String>>) -> String {
    let mut nums: VecDeque<u8> = lines
        .next()
        .expect("No input!")
        .chars()
        .map(|num_s| {
            num_s
                .to_digit(10)
                .expect(&format!("Failed to cast {} to u8", num_s)) as u8
        })
        .collect();
    for _ in 0..100 {
        nums = simulate(nums);
    }
    while *nums.front().unwrap() != 1 {
        let f = nums.pop_front().unwrap();
        nums.push_back(f);
    }
    nums.pop_front();
    return nums.iter().map(|&num| num.to_string()).collect();
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
        let test_input = r#"389125467"#;
        let it = test_input
            .split('\n')
            .into_iter()
            .map(|part| part.to_string());
        assert_eq!(solve(Box::new(it)), String::from("67384529"));
    }
}
