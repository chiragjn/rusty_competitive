// https://leetcode.com/problems/construct-binary-tree-from-inorder-and-postorder-traversal/

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
    fn _build_tree(inorder: &[i32], postorder: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(value) = postorder.last() {
            let inpos = inorder.iter().position(|&x| x == *value).unwrap();
            let inleft = &inorder[..inpos];
            let inright = &inorder[inpos + 1..];
            let postleft = &postorder[..inleft.len()];
            let postright = &postorder[inleft.len()..postorder.len() - 1];
            return Some(Rc::new(RefCell::new(TreeNode {
                val: *value,
                left: Solution::_build_tree(inleft, postleft),
                right: Solution::_build_tree(inright, postright),
            })));
        }
        return None;
    }
    pub fn build_tree(inorder: Vec<i32>, postorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        return Solution::_build_tree(&inorder[..], &postorder[..]);
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
    assert_eq!(Solution::build_tree(vec![], vec![]), None);
    assert_eq!(
        Solution::build_tree(vec![9, 3, 15, 20, 7], vec![9, 15, 7, 20, 3]),
        tn!(
            3,
            tn!(9, None, None),
            tn!(20, tn!(15, None, None), tn!(7, None, None))
        )
    );
}
