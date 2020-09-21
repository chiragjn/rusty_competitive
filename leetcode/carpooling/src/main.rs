// https://leetcode.com/problems/car-pooling/

struct Solution {}

impl Solution {
    pub fn car_pooling(trips: Vec<Vec<i32>>, capacity: i32) -> bool {
        let mut events: Vec<(i32, bool, i32)> = vec![];
        for trip in trips.iter() {
            events.push((trip[1], true, trip[0]));
            events.push((trip[2], false, -trip[0]));
        }
        events.sort();
        let mut running_cap = 0;
        for (_, _, c) in events.iter() {
            running_cap += c;
            if running_cap < 0 || running_cap > capacity {
                return false;
            }
        }
        return true;
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test() {
        assert_eq!(
            Solution::car_pooling(vec![vec![2, 1, 5], vec![3, 3, 7]], 4),
            false
        );
        assert_eq!(
            Solution::car_pooling(vec![vec![2, 1, 5], vec![3, 3, 7]], 5),
            true
        );
        assert_eq!(
            Solution::car_pooling(vec![vec![2, 1, 5], vec![3, 5, 7]], 3),
            true
        );
        assert_eq!(
            Solution::car_pooling(vec![vec![3, 2, 7], vec![3, 7, 9], vec![8, 3, 9]], 11),
            true
        );
    }
}
