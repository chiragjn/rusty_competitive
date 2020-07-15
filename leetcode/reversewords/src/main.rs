// https://leetcode.com/problems/reverse-words-in-a-string/

struct Solution {}

impl Solution {
    pub fn reverse_words(s: String) -> String {
        // Ugly O(1) space inplace solution, otherwise easy to do with split and reverse

        // String type is not equivalent to [char], so can't technically do it without
        // the Vec<char> conversion

        // First, reverse the entire string
        let mut chars: Vec<char> = s.chars().rev().collect();
        let n = chars.len();
        let mut write_ptr: usize;
        let mut rev_ptr: usize;
        let mut word_end_ptr: usize = 0;
        while word_end_ptr < n {
            // rewind write pointer to the correct place
            write_ptr = word_end_ptr;
            while write_ptr > 1 && !(chars[write_ptr - 1] == ' ' && chars[write_ptr - 2] != ' ') {
                write_ptr -= 1;
            }

            // find the end of next word
            while word_end_ptr < n
                && !(chars[word_end_ptr] != ' '
                    && (word_end_ptr == n - 1 || chars[word_end_ptr + 1] == ' '))
            {
                word_end_ptr += 1;
            }

            // reverse the word in place
            if word_end_ptr < n {
                rev_ptr = word_end_ptr;
                while write_ptr < rev_ptr {
                    chars.swap(write_ptr, rev_ptr);
                    write_ptr += 1;
                    rev_ptr -= 1;
                }
                word_end_ptr += 2;
            }
        }

        // Remove all the extra space at the end
        while !chars.is_empty() && chars[chars.len() - 1] == ' ' {
            chars.pop();
        }

        return chars.into_iter().collect();
    }
}

fn main() {
    assert_eq!(Solution::reverse_words(String::from("")), "");
    assert_eq!(Solution::reverse_words(String::from("       ")), "");
    assert_eq!(Solution::reverse_words(String::from("a")), "a");
    assert_eq!(
        Solution::reverse_words(String::from("hello world!")),
        "world! hello"
    );
    assert_eq!(Solution::reverse_words(String::from("hello")), "hello");
    assert_eq!(
        Solution::reverse_words(String::from("the sky is blue")),
        "blue is sky the"
    );
    assert_eq!(
        Solution::reverse_words(String::from("  hello world!  ")),
        "world! hello"
    );
    assert_eq!(
        Solution::reverse_words(String::from("a good   example")),
        "example good a"
    );
}
