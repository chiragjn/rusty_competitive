// https://leetcode.com/problems/rotate-list/

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
    pub fn rotate_right(mut head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        if head.is_none() || k == 0 {
            return head;
        }
        let mut current = head.as_ref();
        let mut size = 0;
        while let Some(c) = current {
            current = c.next.as_ref();
            size += 1;
        }
        let k = size - (k % size);
        if k == size {
            return head;
        }
        let mut current = head.as_mut();
        for _ in 1..k {
            current = current.unwrap().next.as_mut();
        }
        let mut new_head = current.unwrap().next.take();
        let mut current = new_head.as_mut().unwrap();
        while !current.next.is_none() {
            current = current.next.as_mut().unwrap();
        }
        current.next = head;
        return new_head;
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::{ListNode, Solution};

    fn collect_as_vec(head: Option<Box<ListNode>>) -> Vec<i32> {
        let mut values = vec![];
        let mut current = head.as_ref();
        while let Some(c) = current {
            values.push(c.val);
            current = c.next.as_ref();
        }
        return values;
    }

    fn list_from_vec(values: Vec<i32>) -> Option<Box<ListNode>> {
        let mut next: Option<Box<ListNode>> = None;
        for &v in values.iter().rev() {
            next = Some(Box::new(ListNode { val: v, next: next }));
        }
        return next;
    }

    #[test]
    fn test() {
        assert_eq!(
            collect_as_vec(Solution::rotate_right(
                list_from_vec(vec![1, 2, 3, 4, 5]),
                2
            )),
            vec![4, 5, 1, 2, 3]
        );
        assert_eq!(
            collect_as_vec(Solution::rotate_right(list_from_vec(vec![0, 1, 2]), 4)),
            vec![2, 0, 1]
        );
        assert_eq!(
            collect_as_vec(Solution::rotate_right(list_from_vec(vec![0]), 4)),
            vec![0]
        );
        assert_eq!(
            collect_as_vec(Solution::rotate_right(list_from_vec(vec![]), 2)),
            vec![]
        );
        assert_eq!(
            collect_as_vec(Solution::rotate_right(list_from_vec(vec![1, 2]), 3)),
            vec![2, 1]
        );
    }
}
