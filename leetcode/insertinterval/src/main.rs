// https://leetcode.com/problems/insert-interval/

struct Solution {}

impl Solution {
    pub fn insert(mut intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        let mut lptr: usize = 0;
        let mut rptr: isize = intervals.len() as isize - 1;
        while lptr < intervals.len() && intervals[lptr][1] < new_interval[0] {
            lptr += 1;
        }
        if lptr == intervals.len() {
            intervals.push(new_interval);
            return intervals;
        }
        while rptr > -1 && intervals[rptr as usize][0] > new_interval[1] {
            rptr -= 1;
        }
        if rptr == -1 {
            intervals.insert(0, new_interval);
            return intervals;
        }
        let rptr = rptr as usize;
        let mut merged_intervals: Vec<Vec<i32>> = vec![];
        merged_intervals.extend(intervals[..lptr].iter().cloned());
        merged_intervals.push(vec![
            *[new_interval[0], intervals[lptr][0]].iter().min().unwrap(),
            *[new_interval[1], intervals[rptr][1]].iter().max().unwrap(),
        ]);
        merged_intervals.extend(intervals[(rptr + 1)..].iter().cloned());
        return merged_intervals;
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test() {
        assert_eq!(Solution::insert(vec![], vec![2, 8]), vec![vec![2, 8]]);
        assert_eq!(
            Solution::insert(vec![vec![100, 101], vec![103, 104]], vec![2, 8]),
            vec![vec![2, 8], vec![100, 101], vec![103, 104]]
        );
        assert_eq!(
            Solution::insert(vec![vec![100, 101], vec![103, 104]], vec![106, 107]),
            vec![vec![100, 101], vec![103, 104], vec![106, 107]]
        );
        assert_eq!(
            Solution::insert(vec![vec![1, 3], vec![6, 9]], vec![2, 8]),
            vec![vec![1, 9]]
        );
        assert_eq!(
            Solution::insert(vec![vec![1, 3], vec![6, 9]], vec![2, 5]),
            vec![vec![1, 5], vec![6, 9]]
        );
        assert_eq!(
            Solution::insert(
                vec![
                    vec![1, 2],
                    vec![3, 5],
                    vec![6, 7],
                    vec![8, 10],
                    vec![12, 16]
                ],
                vec![4, 8]
            ),
            vec![vec![1, 2], vec![3, 10], vec![12, 16]]
        );
    }
}
