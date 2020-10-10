// https://leetcode.com/problems/minimum-number-of-arrows-to-burst-balloons/

struct Solution {}

impl Solution {
    pub fn find_min_arrow_shots(mut points: Vec<Vec<i32>>) -> i32 {
        if points.is_empty() {
            return 0;
        }
        points.sort_by_key(|p| p[1]);
        let mut current_end: i32 = points[0][1];
        let mut answer = 1;
        for i in 1..points.len() {
            if points[i][0] > current_end {
                current_end = points[i][1];
                answer += 1;
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
            Solution::find_min_arrow_shots(vec![vec![10, 16], vec![2, 8], vec![1, 6], vec![7, 12]]),
            2
        );
        assert_eq!(
            Solution::find_min_arrow_shots(vec![vec![1, 2], vec![3, 4], vec![5, 6], vec![7, 8]]),
            4
        );
        assert_eq!(
            Solution::find_min_arrow_shots(vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5]]),
            2
        );
        assert_eq!(Solution::find_min_arrow_shots(vec![vec![1, 2]]), 1);
        assert_eq!(
            Solution::find_min_arrow_shots(vec![vec![2, 3], vec![2, 3]]),
            1
        );
    }
}
