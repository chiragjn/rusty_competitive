// https://leetcode.com/problems/largest-number/

use std::cmp::Ordering;

struct Solution {}

fn compare(num1: &String, num2: &String) -> Ordering {
    let left: Vec<u8> = num1.bytes().chain(num2.bytes()).collect();
    let right: Vec<u8> = num2.bytes().chain(num1.bytes()).collect();
    return right.cmp(&left);
}

impl Solution {
    pub fn largest_number(nums: Vec<i32>) -> String {
        let mut numsc: Vec<String> = nums.iter().map(|&num| num.to_string()).collect();
        numsc.sort_by(compare);
        let answer: String = numsc.join("");
        if !answer.is_empty() && answer.chars().all(|c| c == '0') {
            return String::from("0");
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
        assert_eq!(
            Solution::largest_number(vec![35, 351, 3504, 33, 3, 30540, 30]),
            String::from("3535135043333054030")
        );
        assert_eq!(Solution::largest_number(vec![10, 2]), String::from("210"));
        assert_eq!(
            Solution::largest_number(vec![3, 30, 34, 5, 9]),
            String::from("9534330")
        );
        assert_eq!(
            Solution::largest_number(vec![1, 0, 1, 0, 6]),
            String::from("61100")
        );
        assert_eq!(
            Solution::largest_number(vec![0, 0, 0, 0, 0]),
            String::from("0")
        );
        assert_eq!(
            Solution::largest_number(vec![
                1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 11, 22, 33, 44, 55, 66, 77, 88, 99
            ]),
            String::from("9998887776665554443332221110")
        );
        assert_eq!(Solution::largest_number(vec![]), String::from(""));

        // Regress
        assert_eq!(
            Solution::largest_number(vec![824, 938, 1399, 5607, 6973, 5703, 9609, 4398, 8247]),
            String::from("9609938824824769735703560743981399")
        );
    }
}
