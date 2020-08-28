// https://leetcode.com/problems/implement-rand10-using-rand7/
// I have no idea why this does not pass, seems uniformly distributed to me and the editorial
// solution is also similar :(

extern crate rand;
use rand::distributions::{Distribution, Uniform};

fn rand7() -> i32 {
    let mut rng = rand::thread_rng();
    let dist = Uniform::new(1, 8);
    return dist.sample(&mut rng);
}

struct Solution {}

impl Solution {
    pub fn rand10() -> i32 {
        let mut pick = 1000;
        let offset = rand7();
        while pick > 40 {
            pick = 7 * (rand7() - 1) + offset;
            if pick <= 40 {
                break;
            }
        }
        return ((pick - 1) % 10) + 1;
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::Solution;
    use std::collections::HashMap;

    #[test]
    fn test() {
        let picks: Vec<i32> = (0..10000).map(|_| Solution::rand10()).collect();
        assert!(picks.iter().all(|&x| x >= 1 && x <= 10));
        let mut counter: HashMap<i32, u32> = HashMap::new();
        for pick in picks {
            *counter.entry(pick).or_insert(0) += 1;
        }
        println!("{:?}", counter);
    }
}
