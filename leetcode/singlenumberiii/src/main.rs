// https://leetcode.com/problems/single-number-iii/

struct Solution {}

fn rightmost_set_bit_pos(n: i32) -> Option<i32> {
    for i in 0..32 {
        if n & (1 << i) != 0 {
            return Some(i);
        }
    }
    return None;
}

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> Vec<i32> {
        // O(N) time, O(1) space solution;
        let mut answer = vec![0, 0];
        let a_xor_b: i32 = nums.iter().fold(0, |xor, val| xor ^ val);
        match rightmost_set_bit_pos(a_xor_b) {
            Some(pos) => {
                answer[0] = nums
                    .iter()
                    .filter(|&x| (x & 1 << pos) != 0)
                    .fold(0, |xor, val| xor ^ val);
                answer[1] = a_xor_b ^ answer[0];
                if answer[0] > answer[1] {
                    answer.swap(0, 1);
                }
            }
            None => {
                unreachable!("Invalid Input");
            }
        }
        return answer;
    }
}

fn main() {
    assert_eq!(Solution::single_number(vec![1, 2, 1, 3, 2, 5]), vec![3, 5]);
    assert_eq!(
        Solution::single_number(vec![300, -100, -200, 300]),
        vec![-200, -100]
    );
}
