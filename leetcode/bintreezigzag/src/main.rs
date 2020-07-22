// https://leetcode.com/problems/binary-tree-zigzag-level-order-traversal/

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

// Definition for a binary tree node.

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
    pub fn zigzag_level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut answer: Vec<Vec<i32>> = vec![];
        let mut q: VecDeque<(Option<Rc<RefCell<TreeNode>>>, usize)> = VecDeque::new();
        q.push_back((root, 0));
        while !q.is_empty() {
            if let Some((Some(node), lvl)) = q.pop_front() {
                while answer.len() < lvl + 1 {
                    answer.push(vec![]);
                }
                answer[lvl].push(node.borrow().val);
                q.push_back((node.borrow().left.clone(), lvl + 1));
                q.push_back((node.borrow().right.clone(), lvl + 1));
            }
        }

        for i in (1..answer.len()).step_by(2) {
            answer[i].reverse();
        }
        return answer;
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
        3,
        tn!(9, None, None),
        tn!(20, tn!(15, None, None), tn!(7, None, None))
    );
    assert_eq!(
        Solution::zigzag_level_order(root),
        vec![vec![3], vec![20, 9], vec![15, 7]]
    );
}
