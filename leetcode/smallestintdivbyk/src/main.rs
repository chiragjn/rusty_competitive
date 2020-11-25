// https://leetcode.com/problems/smallest-integer-divisible-by-k/

struct Solution {}

impl Solution {
    pub fn smallest_repunit_div_by_k(k: i32) -> i32 {
        let last_digit = k % 10;
        let answer: i32 = match last_digit {
            0 | 2 | 4 | 5 | 6 | 8 => -1,
            _ => {
                let mut current = 0;
                for count in 1..k + 1 {
                    current = (((current * 10) % k) + 1) % k;
                    if current == 0 {
                        return count;
                    }
                }
                -1
            }
        };
        return answer;
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test() {
        assert_eq!(Solution::smallest_repunit_div_by_k(1), 1);
        assert_eq!(Solution::smallest_repunit_div_by_k(2), -1);
        assert_eq!(Solution::smallest_repunit_div_by_k(3), 3);
        assert_eq!(Solution::smallest_repunit_div_by_k(9), 9);
    }
}
