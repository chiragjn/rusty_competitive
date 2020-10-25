// https://leetcode.com/problems/stone-game-iv/

struct Solution {}

impl Solution {
    pub fn winner_square_game(n: i32) -> bool {
        let n = n as usize;
        let squares: Vec<usize> = (1..).map(|x| x * x).take_while(|&x| x <= n).collect();
        let mut cache = vec![false; n + 1];
        for i in 1..n + 1 {
            cache[i] = !squares
                .iter()
                .take_while(|&&sq| sq <= i)
                .map(|sq| cache[i - sq])
                .all(|x| x);
        }
        return cache[n];
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test() {
        assert_eq!(Solution::winner_square_game(1), true);
        assert_eq!(Solution::winner_square_game(2), false);
        assert_eq!(Solution::winner_square_game(4), true);
        assert_eq!(Solution::winner_square_game(7), false);
        assert_eq!(Solution::winner_square_game(17), false);
    }
}
