// https://leetcode.com/problems/fizz-buzz/

struct Solution {}

impl Solution {
    pub fn fizz_buzz(n: i32) -> Vec<String> {
        return (1..n + 1)
            .map(|x| {
                if x % 15 == 0 {
                    String::from("FizzBuzz")
                } else if x % 5 == 0 {
                    String::from("Buzz")
                } else if x % 3 == 0 {
                    String::from("Fizz")
                } else {
                    x.to_string()
                }
            })
            .collect();
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::Solution;

    macro_rules! S {
        ($($s:expr),*) => {
            vec![$(String::from($s),)*]
        };
    }

    #[test]
    fn test() {
        assert_eq!(
            Solution::fizz_buzz(15),
            S![
                "1", "2", "Fizz", "4", "Buzz", "Fizz", "7", "8", "Fizz", "Buzz", "11", "Fizz",
                "13", "14", "FizzBuzz"
            ]
        );
    }
}
