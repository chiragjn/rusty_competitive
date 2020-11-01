// https://leetcode.com/problems/convert-binary-number-in-a-linked-list-to-integer/

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

struct Solution {}

impl Solution {
    pub fn get_decimal_value(head: Option<Box<ListNode>>) -> i32 {
        let mut current = &head;
        let mut answer: i32 = 0;
        while let Some(c) = current {
            answer <<= 1;
            answer |= c.val;
            current = &c.next;
        }
        return answer;
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::{list_from_vec, Solution};

    #[test]
    fn test() {
        assert_eq!(Solution::get_decimal_value(list_from_vec(vec![1, 0, 1])), 5);
        assert_eq!(Solution::get_decimal_value(list_from_vec(vec![0])), 0);
        assert_eq!(Solution::get_decimal_value(list_from_vec(vec![1])), 1);
        assert_eq!(
            Solution::get_decimal_value(list_from_vec(vec![
                1, 0, 0, 1, 0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0
            ])),
            18880
        );
        assert_eq!(Solution::get_decimal_value(list_from_vec(vec![0, 0])), 0);
    }
}
