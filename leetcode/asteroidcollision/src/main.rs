// https://leetcode.com/problems/asteroid-collision/

use std::cmp::Ordering;

struct Solution {}

impl Solution {
    pub fn asteroid_collision(asteroids: Vec<i32>) -> Vec<i32> {
        let mut buffer: Vec<i32> = vec![];
        for &x in asteroids.iter() {
            let mut destroyed = false;
            while !buffer.is_empty() && x < 0 && *buffer.last().unwrap() > 0 {
                let last = buffer.last().unwrap();
                match x.abs().cmp(&last.abs()) {
                    Ordering::Equal => {
                        destroyed = true;
                        buffer.pop();
                        break;
                    }
                    Ordering::Less => {
                        destroyed = true;
                        break;
                    }
                    _ => {
                        buffer.pop();
                    }
                }
            }
            if !destroyed {
                buffer.push(x);
            }
        }
        return buffer;
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test() {
        assert_eq!(Solution::asteroid_collision(vec![5, 10, -5]), vec![5, 10]);
        assert_eq!(Solution::asteroid_collision(vec![8, -8]), vec![]);
        assert_eq!(Solution::asteroid_collision(vec![10, 2, -5]), vec![10]);
        assert_eq!(
            Solution::asteroid_collision(vec![-2, -1, 1, 2]),
            vec![-2, -1, 1, 2]
        );
    }
}
