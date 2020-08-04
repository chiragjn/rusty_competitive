// https://leetcode.com/problems/power-of-four/

struct Solution {}

impl Solution {
    pub fn is_power_of_four(num: i32) -> bool {
        return num > 0 && (num & (num - 1)) == 0 && (num & 0x2AAAAAAA) == 0;
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test() {
        assert_eq!(Solution::is_power_of_four(-1), false);
        assert_eq!(Solution::is_power_of_four(0), false);
        assert_eq!(Solution::is_power_of_four(1), true);
        assert_eq!(Solution::is_power_of_four(2), false);
        assert_eq!(Solution::is_power_of_four(3), false);
        assert_eq!(Solution::is_power_of_four(4), true);
        assert_eq!(Solution::is_power_of_four(5), false);
        assert_eq!(Solution::is_power_of_four(16), true);
    }
}
