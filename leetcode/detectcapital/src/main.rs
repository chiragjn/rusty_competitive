// https://leetcode.com/problems/detect-capital/

struct Solution {}

impl Solution {
    pub fn detect_capital_use(word: String) -> bool {
        let mut up_pos = word.len();
        let mut up_count: usize = 0;
        let mut low_count: usize = 0;
        for (i, ch) in word.chars().rev().enumerate() {
            if ch >= 'A' && ch <= 'Z' {
                up_count += 1;
                up_pos = word.len() - 1 - i;
            } else if ch >= 'a' && ch <= 'z' {
                low_count += 1;
            }
        }
        return up_count == word.len() || low_count == word.len() || (up_count == 1 && up_pos == 0);
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test() {
        assert_eq!(Solution::detect_capital_use(String::from("USA")), true);
        assert_eq!(Solution::detect_capital_use(String::from("usa")), true);
        assert_eq!(Solution::detect_capital_use(String::from("Usa")), true);
        assert_eq!(Solution::detect_capital_use(String::from("UsA")), false);
    }
}
