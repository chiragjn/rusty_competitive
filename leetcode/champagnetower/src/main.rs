// https://leetcode.com/problems/champagne-tower/

struct Solution {}

impl Solution {
    pub fn champagne_tower(poured: i32, query_row: i32, query_glass: i32) -> f64 {
        let mut state: Vec<Vec<f64>> = vec![vec![0.0; 105]; 105];
        let query_row = query_row as usize;
        let query_glass = query_glass as usize;
        state[0][0] = poured as f64;
        for row in 0..query_row + 1 {
            for col in 0..row + 1 {
                if state[row][col] > 1.0 {
                    let distribute = (state[row][col] - 1.0) / 2.0;
                    state[row + 1][col] += distribute;
                    state[row + 1][col + 1] += distribute;
                    state[row][col] = 1.0;
                }
            }
        }
        return 1.0f64.min(state[query_row][query_glass]);
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test() {
        assert_eq!(Solution::champagne_tower(1, 1, 1), 0.0);
        assert_eq!(Solution::champagne_tower(2, 1, 1), 0.5);
        assert_eq!(Solution::champagne_tower(100000009, 33, 17), 1.0);
    }
}
