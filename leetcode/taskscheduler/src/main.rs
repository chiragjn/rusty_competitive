// https://leetcode.com/problems/task-scheduler/
// TODO: Solve this without simulation approach, just count how many idle cycles are needed

use std::collections::{BinaryHeap, HashMap, VecDeque};

struct Solution {}

impl Solution {
    pub fn least_interval(tasks: Vec<char>, n: i32) -> i32 {
        let mut tasks_map: HashMap<char, i32> = HashMap::new();
        let mut heap: BinaryHeap<(i32, i32, char)> = BinaryHeap::new(); // (remaining, last picked, name)
        let mut q: VecDeque<(i32, i32, char)> = VecDeque::new();
        for task in tasks {
            *tasks_map.entry(task).or_insert(0) += 1;
        }
        for (task, count) in tasks_map.iter() {
            heap.push((*count, 0, *task));
        }
        let mut timer = 0;
        while !(heap.is_empty() && q.is_empty()) {
            if let Some((_, last_picked, _)) = q.front() {
                if last_picked - timer > n {
                    heap.push(q.pop_front().unwrap());
                }
            }
            if let Some((remaining, _, task)) = heap.pop() {
                if remaining > 1 {
                    q.push_back((remaining - 1, timer, task));
                }
            }
            timer -= 1;
        }
        return -timer;
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test() {
        assert_eq!(
            Solution::least_interval(vec!['A', 'A', 'A', 'B', 'B', 'B'], 2),
            8
        );
        assert_eq!(
            Solution::least_interval(vec!['A', 'A', 'A', 'B', 'B', 'B'], 0),
            6
        );
        assert_eq!(
            Solution::least_interval(
                vec!['A', 'A', 'A', 'A', 'A', 'A', 'B', 'C', 'D', 'E', 'F', 'G'],
                2
            ),
            16
        );
    }
}
