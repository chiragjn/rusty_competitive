// https://leetcode.com/problems/valid-palindrome/

struct Solution {}

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let chars: Vec<char> = s
            .chars()
            .map(|c| c.to_ascii_lowercase())
            .filter(|c| c.is_ascii_alphanumeric())
            .collect();
        return chars.iter().eq(chars.iter().clone().rev());
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test() {
        assert_eq!(
            Solution::is_palindrome(String::from("A man, a plan, a canal: Panama")),
            true
        );
        assert_eq!(Solution::is_palindrome(String::from("")), true);
        assert_eq!(Solution::is_palindrome(String::from("R")), true);
        assert_eq!(Solution::is_palindrome(String::from("race a car")), false);
    }
}
