// https://leetcode.com/problems/sort-array-by-parity/

struct Solution {}

impl Solution {
    pub fn sort_array_by_parity(mut a: Vec<i32>) -> Vec<i32> {
        // let mut b = a.clone();
        // b.sort_by_key(|x| x & 1);
        // return b;
        let (mut left, mut right) = (0, a.len() - 1);
        while left < right {
            if a[left] & 1 == 1 {
                a.swap(left, right);
                right -= 1;
            } else {
                left += 1;
            }
        }
        return a;
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test() {
        assert_eq!(
            Solution::sort_array_by_parity(vec![3, 1, 2, 4]),
            vec![4, 2, 1, 3]
        );
    }
}
