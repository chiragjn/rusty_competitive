// https://leetcode.com/problems/minimum-height-trees/

use std::collections::{HashMap, VecDeque};

struct Solution {}

impl Solution {
    pub fn find_min_height_trees(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        if n == 1 {
            return vec![0];
        }
        let mut graph: HashMap<i32, Vec<i32>> = HashMap::new();
        let mut degrees: HashMap<i32, usize> = HashMap::new();
        let mut num_vertices_left = n as usize;
        let mut queue: VecDeque<i32> = VecDeque::new();
        let mut buffer: Vec<i32> = vec![];
        for edge in edges.iter() {
            graph.entry(edge[0]).or_default().push(edge[1]);
            graph.entry(edge[1]).or_default().push(edge[0]);
        }
        for (&u, vs) in graph.iter() {
            degrees.insert(u, vs.len());
            if degrees[&u] == 1 {
                queue.push_back(u);
            }
        }
        while num_vertices_left > 2 {
            num_vertices_left -= queue.len();
            while !queue.is_empty() {
                let u = queue.pop_front().unwrap();
                for v in graph[&u].iter() {
                    *degrees.get_mut(v).unwrap() -= 1;
                    if degrees[v] == 1 {
                        buffer.push(*v);
                    }
                }
            }
            queue.extend(buffer.drain(..));
        }
        let mut answer: Vec<i32> = queue.drain(..).collect();
        answer.sort();
        return answer;
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test() {
        assert_eq!(
            Solution::find_min_height_trees(4, vec![vec![1, 0], vec![1, 2], vec![1, 3]]),
            vec![1]
        );
        assert_eq!(
            Solution::find_min_height_trees(
                6,
                vec![vec![3, 0], vec![3, 1], vec![3, 2], vec![3, 4], vec![5, 4]]
            ),
            vec![3, 4]
        );
        assert_eq!(Solution::find_min_height_trees(1, vec![]), vec![0]);
        assert_eq!(
            Solution::find_min_height_trees(2, vec![vec![0, 1]]),
            vec![0, 1]
        );
    }
}
