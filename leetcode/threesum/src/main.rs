// https://leetcode.com/problems/3sum/
use std::collections::{BTreeSet, HashMap};

struct Solution {}

impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut triplets: BTreeSet<Vec<i32>> = BTreeSet::new();
        let mut counts: HashMap<i32, i32> = HashMap::new();
        for elem in nums.iter() {
            *counts.entry(*elem).or_insert(0) += 1;
        }
        for i in 0..nums.len() {
            for j in (i + 1)..nums.len() {
                let rem = 0 - (nums[i] + nums[j]);
                let rem_count = 1 + (nums[i] == rem) as i32 + (nums[j] == rem) as i32;
                if counts.contains_key(&rem) && counts[&rem] >= rem_count {
                    let mut item = vec![nums[i], nums[j], rem];
                    item.sort();
                    triplets.insert(item);
                }
            }
        }
        return triplets.into_iter().collect();
    }
}

fn main() {
    println!("{:?}", Solution::three_sum(vec![-1, 0, 1, 2, -1, -4]));
}
