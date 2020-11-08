// https://leetcode.com/problems/add-two-numbers-ii/

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

fn recurse_add(
    counter: usize,
    l1: &Option<Box<ListNode>>,
    l2: &Option<Box<ListNode>>,
    l1_len: usize,
    l2_len: usize,
) -> (Option<Box<ListNode>>, i32) {
    if counter == 0 {
        return (None, 0);
    }
    let mut this = ListNode::new(0);
    let mut l1_next = l1;
    let mut l2_next = l2;
    if counter <= l1_len {
        this.val += l1.as_ref().unwrap().val;
        l1_next = &l1_next.as_ref().unwrap().next;
    }
    if counter <= l2_len {
        this.val += l2.as_ref().unwrap().val;
        l2_next = &l2_next.as_ref().unwrap().next;
    }
    let (next, mut carry) = recurse_add(counter - 1, l1_next, l2_next, l1_len, l2_len);
    if counter > l1_len && counter > l2_len && carry == 0 {
        return (next, carry);
    }
    this.val += carry;
    this.next = next;
    carry = this.val / 10;
    this.val = this.val % 10;
    return (Some(Box::new(this)), carry);
}

fn list_size(l: &Option<Box<ListNode>>) -> usize {
    let mut curr = l;
    let mut size = 0;
    while curr.is_some() {
        size += 1;
        curr = &curr.as_ref().unwrap().next;
    }
    return size;
}

struct Solution {}

impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let l1_len = list_size(&l1);
        let l2_len = list_size(&l2);
        let (l, _) = recurse_add(std::cmp::max(l1_len, l2_len) + 1, &l1, &l2, l1_len, l2_len);
        return l;
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::{list_from_vec, vec_from_list, Solution};

    #[test]
    fn test() {
        assert_eq!(
            vec_from_list(Solution::add_two_numbers(
                list_from_vec(vec![]),
                list_from_vec(vec![])
            )),
            vec![]
        );
        assert_eq!(
            vec_from_list(Solution::add_two_numbers(
                list_from_vec(vec![1, 2, 3]),
                list_from_vec(vec![])
            )),
            vec![1, 2, 3]
        );
        assert_eq!(
            vec_from_list(Solution::add_two_numbers(
                list_from_vec(vec![]),
                list_from_vec(vec![3, 2, 1])
            )),
            vec![3, 2, 1]
        );
        assert_eq!(
            vec_from_list(Solution::add_two_numbers(
                list_from_vec(vec![7, 2, 4, 3]),
                list_from_vec(vec![5, 6, 4])
            )),
            vec![7, 8, 0, 7]
        );
        assert_eq!(
            vec_from_list(Solution::add_two_numbers(
                list_from_vec(vec![1]),
                list_from_vec(vec![9, 9, 9, 9])
            )),
            vec![1, 0, 0, 0, 0]
        );
    }
}
