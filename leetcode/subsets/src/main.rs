// https://leetcode.com/problems/subsets/

struct Solution {}

impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut answer: Vec<Vec<i32>> = vec![];
        let n = 1 << nums.len();
        for i in 0..n {
            answer.push(
                (0..31)
                    .filter(|j| (i & (1 << j)) != 0)
                    .map(|j| nums[j])
                    .collect(),
            );
        }
        return answer;
    }
}

pub fn main() {
    assert_eq!(Solution::subsets(vec![]), vec![vec![]]);
    assert_eq!(
        Solution::subsets(vec![1, 2, 3]),
        vec![
            vec![],
            vec![1],
            vec![2],
            vec![1, 2],
            vec![3],
            vec![1, 3],
            vec![2, 3],
            vec![1, 2, 3]
        ]
    );
}
