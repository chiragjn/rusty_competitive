// https://leetcode.com/problems/add-binary/

use std::iter;
use std::mem::swap;

struct Solution {}

impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let (mut longer, mut shorter) = (&a, &b);
        if shorter.len() > longer.len() {
            swap(&mut longer, &mut shorter);
        }
        let mut carry = 0;
        let mut t;
        let mut crev = String::new();
        for (ai, bi) in longer
            .chars()
            .rev()
            .zip(shorter.chars().rev().chain(iter::repeat('0')))
        {
            t = carry + ai.to_digit(10).unwrap() + bi.to_digit(10).unwrap();
            crev.push(std::char::from_digit(t % 2, 10).unwrap());
            carry = t / 2;
        }

        if carry != 0 {
            crev.push('1');
        }

        return crev.chars().rev().collect();
    }
}

fn main() {
    assert_eq!(
        Solution::add_binary(String::from("11"), String::from("1")),
        "100"
    );
    assert_eq!(
        Solution::add_binary(String::from("0"), String::from("0")),
        "0"
    );
    assert_eq!(
        Solution::add_binary(String::from("0"), String::from("1")),
        "1"
    );
}
