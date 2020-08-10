// https://leetcode.com/problems/excel-sheet-column-number/

struct Solution {}

impl Solution {
    pub fn title_to_number(s: String) -> i32 {
        let mut answer: i32 = 0;
        let mut mutlitpier: i32 = 1;
        for c in s.chars().rev() {
            answer += (c as i32 - 64) * mutlitpier;
            mutlitpier *= 26;
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
        assert_eq!(Solution::title_to_number(String::from("A")), 1);
        assert_eq!(Solution::title_to_number(String::from("AB")), 28);
        assert_eq!(Solution::title_to_number(String::from("ZY")), 701);
        assert_eq!(Solution::title_to_number(String::from("AA")), 27);
    }
}
