// https://leetcode.com/problems/number-of-recent-calls/

use std::collections::VecDeque;

struct RecentCounter {
    buffer: VecDeque<i32>,
}

impl RecentCounter {
    fn new() -> Self {
        return Self {
            buffer: VecDeque::new(),
        };
    }

    fn ping(&mut self, t: i32) -> i32 {
        self.buffer.push_back(t);
        while !self.buffer.is_empty() && *self.buffer.front().unwrap() < (t - 3000) {
            self.buffer.pop_front();
        }
        return self.buffer.len() as i32;
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::RecentCounter;

    fn make_pings(times: Vec<i32>) -> Vec<i32> {
        let mut counter = RecentCounter::new();
        return times.iter().map(|&x| counter.ping(x)).collect();
    }

    #[test]
    fn test() {
        assert_eq!(
            make_pings(vec![1, 100, 3001, 3002, 30000]),
            vec![1, 2, 3, 3, 1]
        );
    }
}
