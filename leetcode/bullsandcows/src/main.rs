// https://leetcode.com/problems/bulls-and-cows/

use std::cmp::min;
use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn get_hint(secret: String, guess: String) -> String {
        let mut a = 0;
        let mut b = 0;
        let mut sset: HashMap<char, u32> = HashMap::new();
        let mut gset: HashMap<char, u32> = HashMap::new();
        for (sc, gc) in secret.chars().zip(guess.chars()) {
            if sc == gc {
                a += 1;
            } else {
                *sset.entry(sc).or_default() += 1;
                *gset.entry(gc).or_default() += 1;
            }
        }

        for (k, v) in gset.iter() {
            b += min(&*sset.entry(*k).or_default(), v);
        }
        return format!("{}A{}B", a, b);
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test() {
        assert_eq!(
            Solution::get_hint(String::from("1807"), String::from("7810")),
            String::from("1A3B")
        );
        assert_eq!(
            Solution::get_hint(String::from("1123"), String::from("0111")),
            String::from("1A1B")
        );
        assert_eq!(
            Solution::get_hint(String::from("1122"), String::from("2211")),
            String::from("0A4B")
        );
    }
}
