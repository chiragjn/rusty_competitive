// https://leetcode.com/problems/insertion-sort-list/
// TODO: actually implement insertion sort. Maybe learn to write unsafe blocks for mutating linked
// lists without fighting the borrow checker

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

fn list_from_vec(values: Vec<i32>) -> Option<Box<ListNode>> {
    let mut next: Option<Box<ListNode>> = None;
    for &val in values.iter().rev() {
        let mut this = ListNode::new(val);
        this.next = next;
        next = Some(Box::new(this));
    }
    return next;
}

fn vec_from_list(head: Option<Box<ListNode>>) -> Vec<i32> {
    let mut current = &head;
    let mut collected: Vec<i32> = vec![];
    while let Some(c) = current {
        collected.push(c.val);
        current = &c.next;
    }
    return collected;
}

struct Solution {}

impl Solution {
    pub fn insertion_sort_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        // technically not insertion sort
        // linked lists in rust can cause headaches
        let mut nodes: Vec<Option<Box<ListNode>>> = vec![];
        let mut next = head;
        while let Some(mut n) = next {
            let temp = n.next.take();
            nodes.push(Some(n));
            next = temp;
        }
        nodes.sort_by_key(|node| node.as_ref().unwrap().val);
        let mut next: Option<Box<ListNode>> = None;
        while !nodes.is_empty() {
            let mut temp = nodes.pop().unwrap().unwrap();
            temp.next = next;
            next = Some(temp);
        }
        return next;
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::{list_from_vec, vec_from_list, Solution};

    #[test]
    fn test() {
        assert_eq!(
            vec_from_list(Solution::insertion_sort_list(list_from_vec(vec![
                4, 2, 1, 3
            ]))),
            vec![1, 2, 3, 4]
        );
        assert_eq!(
            vec_from_list(Solution::insertion_sort_list(list_from_vec(vec![
                -1, 5, 3, 4, 0
            ]))),
            vec![-1, 0, 3, 4, 5]
        );
    }
}
