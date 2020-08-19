// https://leetcode.com/problems/goat-latin/

use std::collections::VecDeque;
use std::iter::repeat;

struct Solution {}

impl Solution {
    pub fn to_goat_latin(s: String) -> String {
        let mut answer: String = String::new();
        let mut chars: VecDeque<char> = VecDeque::new();
        for (i, word) in s.split_whitespace().into_iter().enumerate() {
            chars.extend(word.chars());
            match chars[0] {
                'a' | 'e' | 'i' | 'o' | 'u' | 'A' | 'E' | 'I' | 'O' | 'U' => {}
                _ => {
                    let fc = chars.pop_front().unwrap();
                    chars.push_back(fc);
                }
            }
            chars.push_back('m');
            chars.extend(repeat('a').take(i + 2));
            answer.extend(chars.drain(..));
            answer.push(' ');
        }
        answer.pop();
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
            Solution::to_goat_latin(String::from("I speak Goat Latin")),
            String::from("Imaa peaksmaaa oatGmaaaa atinLmaaaaa")
        );
        assert_eq!(
            Solution::to_goat_latin(String::from("The quick brown fox jumped over the lazy dog")),
            String::from("heTmaa uickqmaaa rownbmaaaa oxfmaaaaa umpedjmaaaaaa overmaaaaaaa hetmaaaaaaaa azylmaaaaaaaaa ogdmaaaaaaaaaa")
        );
    }
}
