// https://leetcode.com/problems/vertical-order-traversal-of-a-binary-tree/

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

    /// Convenience method to construct Tree from leetcode input format
    fn from_vec(serialized: Vec<Option<i32>>) -> Option<Rc<RefCell<Self>>> {
        if serialized.len() == 0 || serialized[0].is_none() {
            return None;
        }
        let root = Rc::new(RefCell::new(Self::new(serialized[0].unwrap())));
        let mut q: VecDeque<Rc<RefCell<Self>>> = VecDeque::new();
        q.push_back(root.clone());
        let mut idx: usize = 1;
        while !q.is_empty() {
            let node = q.pop_front().unwrap();
            if idx < serialized.len() {
                if let Some(v) = serialized[idx] {
                    let left = Rc::new(RefCell::new(Self::new(v)));
                    node.borrow_mut().left = Some(left.clone());
                    q.push_back(left);
                }
                idx += 1;

                if idx < serialized.len() {
                    if let Some(v) = serialized[idx] {
                        let right = Rc::new(RefCell::new(Self::new(v)));
                        node.borrow_mut().right = Some(right.clone());
                        q.push_back(right);
                    }
                    idx += 1;
                }
            }
        }

        return Some(root);
    }
}

struct Solution {}

impl Solution {
    fn _traverse(
        node: &Option<Rc<RefCell<TreeNode>>>,
        x: i32,
        y: i32,
        buffer: &mut Vec<(i32, i32, i32)>,
    ) {
        if let Some(u) = node {
            buffer.push((x, y, u.borrow().val));
            Solution::_traverse(&u.borrow().left, x - 1, y + 1, buffer);
            Solution::_traverse(&u.borrow().right, x + 1, y + 1, buffer);
        }
    }
    pub fn vertical_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut buffer = vec![];
        Solution::_traverse(&root, 0, 0, &mut buffer);
        buffer.sort();
        let mut prevx = std::i32::MIN;
        let mut answer = vec![];
        for (x, _, v) in buffer {
            if x != prevx {
                answer.push(vec![]);
            }
            prevx = x;
            answer.last_mut().unwrap().push(v);
        }
        return answer;
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::{Solution, TreeNode};

    #[test]
    fn test() {
        assert_eq!(
            Solution::vertical_traversal(TreeNode::from_vec(vec![
                Some(3),
                Some(9),
                Some(20),
                None,
                None,
                Some(15),
                Some(7)
            ])),
            vec![vec![9], vec![3, 15], vec![20], vec![7]]
        );
        assert_eq!(
            Solution::vertical_traversal(TreeNode::from_vec(vec![
                Some(1),
                Some(2),
                Some(3),
                Some(4),
                Some(5),
                Some(6),
                Some(7)
            ])),
            vec![vec![4], vec![2], vec![1, 5, 6], vec![3], vec![7]]
        );
    }
}
