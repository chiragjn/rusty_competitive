struct Solution {}

impl Solution {
    pub fn calculate(s: String) -> i32 {
        let mut chars: Vec<char> = s.chars().filter(|&c| c != ' ').collect();
        let mut stack: Vec<i32> = vec![];
        let mut op: char = '+';
        let mut current_number: i32 = 0;
        for (i, &c) in chars.iter().enumerate() {
            if c.is_digit(10) {
                current_number *= 10;
                current_number += c.to_digit(10).unwrap() as i32;
            } else if !c.is_digit(10) || i == chars.len() - 1 {
                match op {
                    '+' => {
                        stack.push(current_number);
                    }
                    '-' => {
                        stack.push(-current_number);
                    }
                    '*' => {
                        let popped = stack.pop().unwrap();
                        stack.push(popped * current_number);
                    }
                    '/' => {
                        let popped = stack.pop().unwrap();
                        stack.push(popped / current_number);
                    }
                    _ => {
                        unreachable!();
                    }
                }
                current_number = 0;
                op = c;
            }
        }
        return stack.iter().sum();
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test() {}
}
