// https://leetcode.com/problems/search-a-2d-matrix/

struct Solution {}

impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let not_really_empty: Vec<Vec<i32>> = vec![vec![]]; // some bullshit testcase
        if matrix.is_empty() || matrix == not_really_empty {
            return false;
        }
        let mut mid: isize;
        let mut low: isize = 0;
        let mut high = (matrix.len() - 1) as isize;
        let mut row: Option<usize> = None;
        while low <= high {
            mid = low + ((high - low) / 2);
            if matrix[mid as usize][0] > target {
                high = mid - 1;
            } else {
                row = Some(mid as usize);
                low = mid + 1;
            }
        }
        if let Some(i) = row {
            low = 0;
            high = (matrix[i].len() - 1) as isize;
            while low <= high {
                mid = low + ((high - low) / 2);
                if matrix[i][mid as usize] == target {
                    return true;
                } else if matrix[i][mid as usize] > target {
                    high = mid - 1;
                } else {
                    low = mid + 1;
                }
            }
        }
        return false;
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test() {
        assert_eq!(
            Solution::search_matrix(
                vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 50]],
                13
            ),
            false
        );
        assert_eq!(
            Solution::search_matrix(
                vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 50]],
                16
            ),
            true
        );
        assert_eq!(
            Solution::search_matrix(
                vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 50]],
                1
            ),
            true
        );
        assert_eq!(
            Solution::search_matrix(
                vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 50]],
                0
            ),
            false
        );
        assert_eq!(
            Solution::search_matrix(
                vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 50]],
                51
            ),
            false
        );
    }
}
