// https://leetcode.com/problems/find-right-interval/

struct Solution {}

impl Solution {
    pub fn find_right_interval(intervals: Vec<Vec<i32>>) -> Vec<i32> {
        let mut sintervals: Vec<(&Vec<i32>, i32)> = intervals
            .iter()
            .zip(0 as i32..intervals.len() as i32)
            .collect();
        sintervals.sort();
        let mut answer = vec![-1; intervals.len()];
        let (mut low, mut high, mut mid);
        for (i, interval) in sintervals.iter().enumerate() {
            let target = interval.0[1];
            low = i + 1;
            high = sintervals.len() - 1;
            while low <= high {
                mid = low + ((high - low) / 2);
                if sintervals[mid].0[0] >= target {
                    answer[interval.1 as usize] = sintervals[mid].1;
                    high = mid - 1;
                } else {
                    low = mid + 1;
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
        assert_eq!(Solution::find_right_interval(vec![vec![1, 2]]), vec![-1]);
        assert_eq!(
            Solution::find_right_interval(vec![vec![1, 4], vec![2, 3], vec![3, 4]]),
            vec![-1, 2, -1]
        );
        assert_eq!(
            Solution::find_right_interval(vec![vec![3, 4], vec![2, 3], vec![1, 2]]),
            vec![-1, 0, 1]
        );
    }
}
