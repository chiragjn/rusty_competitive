// https://leetcode.com/problems/complement-of-base-10-integer/

struct Solution {}

impl Solution {
    pub fn bitwise_complement(n: i32) -> i32 {
        if n == 0 {
            return 1;
        }
        let mut n = n;
        let mut base = 1;
        let mut answer = 0;
        while n > 0 {
            if (n & 1) == 0 {
                answer += base;
            }
            n >>= 1;
            base <<= 1;
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
        assert_eq!(Solution::bitwise_complement(5), 2);
        assert_eq!(Solution::bitwise_complement(10), 5);
        assert_eq!(Solution::bitwise_complement(7), 0);
    }
}
