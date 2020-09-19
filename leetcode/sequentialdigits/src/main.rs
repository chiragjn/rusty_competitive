// https://leetcode.com/problems/sequential-digits/

struct Solution {}

impl Solution {
    pub fn sequential_digits(low: i32, high: i32) -> Vec<i32> {
        let digits = "123456789";
        let mut answer: Vec<i32> = vec![];
        'outer: for i in 1..10 {
            for j in 0..(9 - i + 1) {
                let num = digits[j..j + i].parse::<i32>().unwrap();
                if num > high {
                    break 'outer;
                }
                if num >= low {
                    answer.push(num);
                }
            }
        }
        return answer;
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test() {
        assert_eq!(Solution::sequential_digits(100, 300), vec![123, 234]);
        assert_eq!(
            Solution::sequential_digits(1000, 13000),
            vec![1234, 2345, 3456, 4567, 5678, 6789, 12345]
        );
    }
}
