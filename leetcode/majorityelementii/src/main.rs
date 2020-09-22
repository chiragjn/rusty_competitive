// https://leetcode.com/problems/majority-element-ii/
//
// Learn at
// https://en.wikipedia.org/wiki/Boyer%E2%80%93Moore_majority_vote_algorithm
// https://jayjingyuliu.wordpress.com/2018/01/29/algorithm-boyer-moore-majority-vote-algorithm-with-extension/

struct Solution {}

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> Vec<i32> {
        if nums.is_empty() {
            return vec![];
        }
        let mut major1 = nums[0];
        let mut major2 = nums[0];
        let mut count1 = 1;
        let mut count2 = 0;
        for &num in nums.iter() {
            if num == major1 {
                count1 += 1;
            } else if num == major2 {
                count2 += 1;
            } else if count1 == 0 {
                major1 = num;
                count1 = 1;
            } else if count2 == 0 {
                major2 = num;
                count2 = 1;
            } else {
                count1 -= 1;
                count2 -= 1;
            }
        }
        let mut answer = vec![];
        let tcount1 = nums.iter().filter(|&x| *x == major1).count();
        let tcount2 = nums.iter().filter(|&x| *x == major2).count();
        if tcount1 > (nums.len() / 3) {
            answer.push(major1);
        }
        if major2 != major1 && tcount2 > (nums.len() / 3) {
            answer.push(major2);
        }
        return answer;
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test() {
        assert_eq!(Solution::majority_element(vec![3, 2, 3]), vec![3]);
        assert_eq!(
            Solution::majority_element(vec![1, 1, 1, 3, 3, 2, 2, 2]),
            vec![1, 2]
        );
    }
}
