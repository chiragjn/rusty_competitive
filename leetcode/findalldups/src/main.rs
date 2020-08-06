// https://leetcode.com/problems/find-all-duplicates-in-an-array/

struct Solution {}

impl Solution {
    pub fn find_duplicates(mut nums: Vec<i32>) -> Vec<i32> {
        // no extra space allowed
        let n = nums.len() as i32;
        for i in 0..nums.len() {
            let j = (nums[i] % (n + 1)) as usize;
            nums[j - 1] += n + 1;
        }
        let mut answer = vec![];
        for i in 1..nums.len() + 1 {
            if nums[i - 1] / (n + 1) == 2 {
                answer.push(i as i32);
            }
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
        assert_eq!(
            Solution::find_duplicates(vec![4, 3, 2, 7, 8, 2, 3, 1]),
            vec![2, 3]
        );
    }
}
