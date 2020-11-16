// https://leetcode.com/problems/longest-mountain-in-array/

struct Solution {}

impl Solution {
    pub fn longest_mountain(a: Vec<i32>) -> i32 {
        let mut left: usize = 0;
        let mut right: usize;
        let mut best: usize = 0;
        let n = a.len();
        while left < n {
            right = left + 1;
            while right < n && a[right] > a[right - 1] {
                right += 1;
            }
            let peak = right - 1;
            if peak < n && peak != left {
                while right < n && a[right] < a[right - 1] {
                    right += 1;
                }
            }
            right -= 1;
            if left != peak && peak != right {
                best = best.max(right - left + 1);
                left = right;
            } else {
                left = right + 1;
            }
        }

        return best as i32;
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test() {
        assert_eq!(Solution::longest_mountain(vec![2, 1, 4, 7, 3, 2, 5]), 5);
        assert_eq!(
            Solution::longest_mountain(vec![
                2, 1, 4, 7, 3, 2, 5, 7, 8, 9, 10, 9, 8, 7, 6, 5, 1, 2, 3
            ]),
            12
        );
        assert_eq!(Solution::longest_mountain(vec![2, 2, 2]), 0);
        assert_eq!(Solution::longest_mountain(vec![2, 1, 4, 7, 3]), 4);
        assert_eq!(Solution::longest_mountain(vec![1, 4, 7, 3]), 4);
        assert_eq!(Solution::longest_mountain(vec![2, 1, 4, 7]), 0);
    }
}
