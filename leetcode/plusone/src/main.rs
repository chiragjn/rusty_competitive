// https://leetcode.com/problems/plus-one/

struct Solution {}

impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let mut carry: i32 = 0;
        let mut add: i32 = 1;
        let mut answer = digits.clone();
        for i in answer.iter_mut().rev() {
            *i = *i + carry + add;
            carry = *i / 10;
            *i = *i % 10;
            add = 0;
        }
        if carry != 0 {
            answer.insert(0, carry);
        }
        return answer;
    }
}

pub fn main() {
    println!("{:?}", Solution::plus_one(vec![9, 9, 9, 9]));
}
