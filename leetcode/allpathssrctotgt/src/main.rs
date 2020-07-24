// https://leetcode.com/problems/all-paths-from-source-to-target/

use std::mem::swap;

struct AllPathsFinder<'a> {
    dag: &'a Vec<Vec<i32>>,
    answer: Vec<Vec<i32>>,
    buffer: Vec<i32>,
}

impl<'a> AllPathsFinder<'a> {
    pub fn new(dag: &'a Vec<Vec<i32>>) -> Self {
        return AllPathsFinder {
            dag: dag,
            answer: vec![],
            buffer: vec![],
        };
    }

    fn _recurse(&mut self, u: i32, target: i32) {
        self.buffer.push(u);
        if u == target {
            self.answer.push(self.buffer.clone());
        } else {
            for v in &self.dag[u as usize] {
                self._recurse(*v, target);
            }
        }
        self.buffer.pop();
    }

    pub fn find(&mut self, source: i32, target: i32) -> Vec<Vec<i32>> {
        let mut all_paths = vec![];
        if self.dag.len() != 0 {
            self._recurse(source, target);
            swap(&mut all_paths, &mut self.answer);
        }
        return all_paths;
    }
}

struct Solution {}

impl Solution {
    pub fn all_paths_source_target(graph: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        return AllPathsFinder::new(&graph).find(0, graph.len() as i32 - 1);
    }
}

fn main() {
    assert_eq!(
        Solution::all_paths_source_target(vec![vec![1, 2], vec![3], vec![3], vec![]]),
        vec![vec![0, 1, 3], vec![0, 2, 3]]
    );
}
