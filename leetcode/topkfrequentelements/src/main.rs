// https://leetcode.com/problems/top-k-frequent-elements/

use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;

struct Solution {}

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut counter: HashMap<i32, i32> = HashMap::new();
        for num in &nums {
            *counter.entry(*num).or_insert(0) += 1;
        }
        let mut counts_to_nums: HashMap<i32, Vec<i32>> = HashMap::new();
        for (num, count) in counter {
            counts_to_nums.entry(count).or_insert(Vec::new()).push(num);
        }
        let mut answer: Vec<i32> = Vec::new();
        let k_ = k as usize;
        for count in (1..(nums.len() as i32) + 1).rev() {
            if counts_to_nums.contains_key(&count) {
                for num in &counts_to_nums[&count] {
                    answer.push(*num);
                    if answer.len() == k_ {
                        return answer;
                    }
                }
            }
        }
        unreachable!("Invalid input data, can't make a set of size k");
    }
}

fn main() {
    assert_eq!(
        HashSet::<i32>::from_iter(Solution::top_k_frequent(vec![1, 1, 1, 2, 2, 3], 2)),
        HashSet::from_iter(vec![1, 2])
    );
    assert_eq!(
        HashSet::<i32>::from_iter(Solution::top_k_frequent(vec![1], 1)),
        HashSet::from_iter(vec![1])
    )
}
