// https://leetcode.com/problems/remove-linked-list-elements/

// Definition for singly-linked list.

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

struct Solution {}

impl Solution {
    fn _remove_elements(head: Option<Box<ListNode>>, val: &i32) -> Option<Box<ListNode>> {
        match head {
            Some(h) => {
                if h.val == *val {
                    return Solution::_remove_elements(h.next, val);
                } else {
                    let mut h_mut = h;
                    h_mut.next = Solution::_remove_elements(h_mut.next, val);
                    return Some(h_mut);
                }
            }
            None => {
                return head;
            }
        }
    }

    pub fn remove_elements(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
        return Solution::_remove_elements(head, &val);
    }
}

// TODO: implement an iterative version as an extra exercise
// TODO: Study other solutions on https://leetcode.com/submissions/detail/369303963/
// TODO: try implementing an proper iterator for ListNode as an extra exercise

fn run_from_vec(list_vec: Vec<i32>, val: i32) -> Vec<i32> {
    let mut head: Option<Box<ListNode>> = None;
    let mut prev: Option<Box<ListNode>>;
    for elem in list_vec.iter().rev() {
        prev = head;
        head = Some(Box::new(ListNode {
            val: *elem,
            next: prev,
        }));
    }
    head = Solution::remove_elements(head, val);
    let mut collected: Vec<i32> = Vec::new();
    while let Some(h) = head {
        collected.push(h.val);
        head = h.next;
    }
    return collected;
}

fn main() {
    assert_eq!(run_from_vec(vec![], 1), vec![]);
    assert_eq!(run_from_vec(vec![2, 3], 1), vec![2, 3]);
    assert_eq!(
        run_from_vec(vec![1, 1, 2, 3, 1, 1, 1, 4, 5, 1, 1], 1),
        vec![2, 3, 4, 5]
    );
}
