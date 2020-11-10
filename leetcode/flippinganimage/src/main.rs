// https://leetcode.com/problems/flipping-an-image/

struct Solution {}

impl Solution {
    pub fn flip_and_invert_image(a: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut answer = a.clone();
        for (i, row) in a.iter().enumerate() {
            for (j, &v) in row.iter().rev().enumerate() {
                answer[i][j] = match v {
                    0 => 1,
                    1 => 0,
                    _ => unreachable!(),
                };
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
            Solution::flip_and_invert_image(vec![vec![1, 1, 0], vec![1, 0, 1], vec![0, 0, 0]]),
            vec![vec![1, 0, 0], vec![0, 1, 0], vec![1, 1, 1]]
        );
        assert_eq!(
            Solution::flip_and_invert_image(vec![
                vec![1, 1, 0, 0],
                vec![1, 0, 0, 1],
                vec![0, 1, 1, 1],
                vec![1, 0, 1, 0]
            ]),
            vec![
                vec![1, 1, 0, 0],
                vec![0, 1, 1, 0],
                vec![0, 0, 0, 1],
                vec![1, 0, 1, 0]
            ]
        );
    }
}
