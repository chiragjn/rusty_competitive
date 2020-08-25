// https://leetcode.com/problems/minimum-cost-for-tickets/

use std::cmp::min;
use std::collections::HashSet;
use std::iter::FromIterator;

struct Solution {}

impl Solution {
    pub fn mincost_tickets(days: Vec<i32>, costs: Vec<i32>) -> i32 {
        let first_day = *days.iter().min().unwrap() as usize;
        let last_day = *days.iter().max().unwrap() as usize;
        let travel_days = HashSet::<i32>::from_iter(days);
        let mut cache: Vec<i32> = vec![0; 500];
        for day in (first_day..last_day + 1).rev() {
            cache[day] = cache[day + 1];
            if travel_days.contains(&(day as i32)) {
                cache[day] = min(
                    costs[0] + cache[day + 1],
                    min(costs[1] + cache[day + 7], costs[2] + cache[day + 30]),
                );
            }
        }
        return cache[first_day];
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test() {
        assert_eq!(
            Solution::mincost_tickets(vec![1, 4, 6, 7, 8, 20], vec![2, 7, 15]),
            11
        );
        assert_eq!(
            Solution::mincost_tickets(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 30, 31], vec![2, 7, 15]),
            17
        );
    }
}
