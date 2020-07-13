// https://leetcode.com/problems/same-tree/

use std::cell::RefCell;
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
    pub fn is_same_tree(
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        match (p, q) {
            (Some(first), Some(second)) => {
                return first.borrow().val == second.borrow().val
                    && Solution::is_same_tree(
                        first.borrow().left.clone(),
                        second.borrow().left.clone(),
                    )
                    && Solution::is_same_tree(
                        first.borrow().right.clone(),
                        second.borrow().right.clone(),
                    )
            }
            (None, None) => {
                return true;
            }
            _ => {
                return false;
            }
        }
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
    let p = tn!(
        1,
        tn!(3, tn!(5, tn!(6, None, None), None), None),
        tn!(2, None, tn!(9, None, tn!(7, None, None)))
    );
    let q = tn!(
        1,
        tn!(3, tn!(5, tn!(6, None, None), None), None),
        tn!(2, None, tn!(9, None, tn!(7, None, None)))
    );
    println!("{:?}", Solution::is_same_tree(p, q));
    let a = tn!(1, tn!(2, None, None), None);
    let b = tn!(1, None, tn!(2, None, None));
    println!("{:?}", Solution::is_same_tree(a, b));
}
