// https://leetcode.com/problems/course-schedule-ii/

struct Solution {}

#[derive(Clone, Debug)]
enum NodeState {
    Unvisited,
    Started,
    Finished,
}

struct TopologicalSorter {
    node_states: Vec<NodeState>,
    sorted: Vec<i32>,
}

impl TopologicalSorter {
    fn new(n: usize) -> Self {
        return Self {
            node_states: vec![NodeState::Unvisited; n],
            sorted: vec![],
        };
    }

    fn _sort(&mut self, u: i32, graph: &Vec<Vec<i32>>) -> bool {
        let uu = u as usize;
        match self.node_states[uu] {
            NodeState::Finished => {
                return true;
            }
            NodeState::Started => {
                return false;
            }
            NodeState::Unvisited => {
                self.node_states[uu] = NodeState::Started;
                let mut r = true;
                for v in &graph[uu] {
                    r &= self._sort(*v, graph);
                }
                self.node_states[uu] = NodeState::Finished;
                self.sorted.push(u);
                return r;
            }
        }
    }

    fn sort(&mut self, graph: &Vec<Vec<i32>>) -> &Vec<i32> {
        let mut acyclic = true;
        for u in 0..graph.len() {
            acyclic &= self._sort(u as i32, graph);
            if !acyclic {
                self.sorted = vec![];
                break;
            }
        }
        return &self.sorted;
    }
}

impl Solution {
    pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
        let sz = num_courses as usize;
        let mut graph: Vec<Vec<i32>> = vec![vec![]; sz];
        let mut sorter = TopologicalSorter::new(sz);
        for pair in prerequisites {
            graph[pair[0] as usize].push(pair[1]);
        }
        return sorter.sort(&graph).clone();
    }
}

fn main() {
    println!("{:?}", Solution::find_order(0, vec![]));
    println!(
        "{:?}",
        Solution::find_order(2, vec![vec![1, 0], vec![0, 1]])
    );
    println!("{:?}", Solution::find_order(2, vec![vec![1, 0]]));
    println!(
        "{:?}",
        Solution::find_order(4, vec![vec![1, 0], vec![2, 0], vec![3, 1], vec![3, 2]])
    );
}
