// https://leetcode.com/problems/distribute-candies-to-people/

use std::cmp::min;

struct Solution {}

impl Solution {
    pub fn distribute_candies(candies: i32, num_people: i32) -> Vec<i32> {
        // what a ugly mess
        // beware of overflow panics!
        let n = num_people as i64;
        let n_sq = n * n;
        let mut c = candies.clone();
        let two_c = 2 * c as i64;
        let mut answer = vec![0; num_people as usize];

        // Find how many complete rounds we can make
        // can binary search here instead
        let mut k: i64 = 0;
        while (k * k * n_sq) + (k * n) <= two_c {
            k += 1;
        }
        k -= 1;

        // distribute according to k complete rounds
        for i in 1..n + 1 {
            answer[i as usize - 1] = ((((k * (k - 1)) / 2) * n) + (i * k)) as i32;
            c -= answer[i as usize - 1];
        }

        // distribute the rest
        let mut g = (k * n) as i32 + 1;
        let mut idx = 0;
        while c > 0 {
            answer[idx] += min(c, g);
            c -= g;
            g += 1;
            idx += 1;
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
        assert_eq!(Solution::distribute_candies(7, 4), vec![1, 2, 3, 1]);
        assert_eq!(Solution::distribute_candies(10, 3), vec![5, 2, 3]);
        assert_eq!(Solution::distribute_candies(60, 4), vec![15, 18, 15, 12]);
    }
}
