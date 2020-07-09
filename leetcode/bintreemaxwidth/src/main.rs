// https://leetcode.com/problems/maximum-width-of-binary-tree/
// Definition for a binary tree node.

use std::cell::RefCell;
use std::cmp::{max, min};
use std::collections::{HashMap, VecDeque};
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

struct Solution {}

impl Solution {
    pub fn width_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut answer = 0;
        if let Some(node) = root {
            let mut min_max: HashMap<u32, (u32, u32)> = HashMap::new();
            let mut q: VecDeque<(u32, u32, Rc<RefCell<TreeNode>>)> = VecDeque::new(); // (level, idx);
            q.push_back((0, 0, node));
            while !q.is_empty() {
                if let Some((lvl, idx, unode)) = q.pop_front() {
                    // println!("{:?}", (lvl, idx));
                    let v = min_max.entry(lvl).or_insert((u32::MAX, u32::MIN));
                    v.0 = min(v.0, idx);
                    v.1 = max(v.1, idx);
                    if unode.borrow().left.is_some() {
                        q.push_back((lvl + 1, idx * 2 + 1, unode.borrow().left.clone().unwrap()));
                    }
                    if unode.borrow().right.is_some() {
                        q.push_back((lvl + 1, idx * 2 + 2, unode.borrow().right.clone().unwrap()));
                    }
                }
            }
            for (min_idx, max_idx) in min_max.values() {
                answer = max(answer, (max_idx - min_idx) + 1);
            }
        }
        return answer as i32;
    }
}

macro_rules! tn {
    ($a: expr, $b: expr, $c: expr) => {
        Some(Rc::new(RefCell::new(TreeNode {
            val: $a,
            left: $b,
            right: $c,
        })))
    };
}

fn main() {
    let root = tn!(
        1,
        tn!(3, tn!(5, tn!(6, None, None), None), None),
        tn!(2, None, tn!(9, None, tn!(7, None, None)))
    );
    println!("{:?}", Solution::width_of_binary_tree(root));
}
