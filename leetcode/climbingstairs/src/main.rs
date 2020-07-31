// https://leetcode.com/problems/climbing-stairs/

struct Solution {}

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        let nu = n as usize;
        let mut cache: Vec<i32> = vec![0; nu + 1];
        cache[nu] = 1;
        for i in (0..nu + 1).rev() {
            if i + 1 < nu + 1 {
                cache[i] += cache[i + 1];
            }
            if i + 2 < nu + 1 {
                cache[i] += cache[i + 2];
            }
        }
        return cache[0];
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test() {
        assert_eq!(Solution::climb_stairs(0), 1);
        assert_eq!(Solution::climb_stairs(1), 1);
        assert_eq!(Solution::climb_stairs(2), 2);
        assert_eq!(Solution::climb_stairs(3), 3);
        assert_eq!(Solution::climb_stairs(4), 5);
    }
}
