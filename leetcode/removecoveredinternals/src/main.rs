// https://leetcode.com/problems/remove-covered-intervals/

struct Solution {}

impl Solution {
    pub fn remove_covered_intervals(mut intervals: Vec<Vec<i32>>) -> i32 {
        let mut answer = 0;
        if !intervals.is_empty() {
            intervals.sort_by_key(|v| (v[0], -v[1]));
            let (mut pstart, mut pend) = (intervals[0][0], intervals[0][1]);
            answer = 1;
            for i in 1..intervals.len() {
                if !(intervals[i][0] >= pstart && intervals[i][1] <= pend) {
                    pstart = intervals[i][0];
                    pend = intervals[i][1];
                    answer += 1;
                }
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
            Solution::remove_covered_intervals(vec![vec![1, 4], vec![3, 6], vec![2, 8]]),
            2
        );
        assert_eq!(
            Solution::remove_covered_intervals(vec![vec![1, 4], vec![2, 3]]),
            1
        );
        assert_eq!(
            Solution::remove_covered_intervals(vec![vec![0, 10], vec![5, 12]]),
            2
        );
        assert_eq!(
            Solution::remove_covered_intervals(vec![vec![3, 10], vec![4, 10], vec![5, 11]]),
            2
        );
        assert_eq!(
            Solution::remove_covered_intervals(vec![vec![1, 2], vec![1, 4], vec![3, 4]]),
            1
        );
    }
}
