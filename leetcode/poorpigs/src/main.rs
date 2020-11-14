// https://leetcode.com/problems/poor-pigs/

struct Solution {}

impl Solution {
    pub fn poor_pigs(buckets: i32, minutes_to_die: i32, minutes_to_test: i32) -> i32 {
        // prone to overflows
        let rounds = minutes_to_test / minutes_to_die;
        let mut answer: u32 = 0;
        while (rounds + 1).pow(answer) < buckets {
            answer += 1;
        }
        return answer as i32;
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test() {
        assert_eq!(Solution::poor_pigs(3, 15, 60), 1);
        assert_eq!(Solution::poor_pigs(3, 15, 15), 2);
    }
}
