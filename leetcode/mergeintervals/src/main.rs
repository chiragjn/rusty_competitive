// https://leetcode.com/problems/merge-intervals/

struct Solution {}

impl Solution {
    pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut merged = vec![];
        if !intervals.is_empty() {
            let mut intervals = intervals;
            intervals.sort();
            merged.push(intervals[0].clone());
            for interval in intervals.iter().skip(1) {
                let prev = merged.last_mut().unwrap();
                if interval[0] >= prev[0] && interval[0] <= prev[1] {
                    prev[1] = prev[1].max(interval[1]);
                } else {
                    merged.push(interval.clone());
                }
            }
        }
        return merged;
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test() {
        assert_eq!(Solution::merge(vec![]), Vec::<Vec<i32>>::new());
        assert_eq!(
            Solution::merge(vec![vec![1, 3], vec![2, 6], vec![8, 10], vec![15, 18]]),
            vec![vec![1, 6], vec![8, 10], vec![15, 18]]
        );
        assert_eq!(
            Solution::merge(vec![vec![1, 4], vec![4, 5]]),
            vec![vec![1, 5]]
        );
        assert_eq!(
            Solution::merge(vec![vec![8, 10], vec![1, 3], vec![2, 6], vec![15, 18]]),
            vec![vec![1, 6], vec![8, 10], vec![15, 18]]
        );
    }
}
