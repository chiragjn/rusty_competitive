// https://leetcode.com/problems/unique-morse-code-words/

use std::collections::HashSet;

struct Solution {}

impl Solution {
    pub fn unique_morse_representations(words: Vec<String>) -> i32 {
        let morse_values: Vec<&str> = vec![
            ".-", "-...", "-.-.", "-..", ".", "..-.", "--.", "....", "..", ".---", "-.-", ".-..",
            "--", "-.", "---", ".--.", "--.-", ".-.", "...", "-", "..-", "...-", ".--", "-..-",
            "-.--", "--..",
        ];
        let mut transformed: HashSet<String> = HashSet::new();
        for word in words {
            transformed.insert(
                word.chars()
                    .map(|x| morse_values[((x as u8) - ('a' as u8)) as usize].chars())
                    .flatten()
                    .collect::<String>(),
            );
        }
        return transformed.len() as i32;
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test() {
        assert_eq!(
            Solution::unique_morse_representations(vec![
                String::from("gin"),
                String::from("zen"),
                String::from("gig"),
                String::from("msg")
            ]),
            2
        );
    }
}
