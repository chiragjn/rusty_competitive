struct Solution {}

impl Solution {
    pub fn add_digits(num: i32) -> i32 {
        if num > 0 {
            return 1 + ((num - 1) % 9);
        }
        return 0;
    }
}

fn main() {
    assert_eq!(Solution::add_digits(0), 0);
    assert_eq!(Solution::add_digits(38), 2);
    assert_eq!(Solution::add_digits(9876), 3);
    for i in 1..10 {
        assert_eq!(Solution::add_digits(i), i);
    }
}
