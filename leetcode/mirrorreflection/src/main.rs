// https://leetcode.com/problems/mirror-reflection/

struct Solution {}

fn gcd(x: i32, y: i32) -> i32 {
    let mut a = x;
    let mut b = y;
    let mut temp;
    while a != 0 {
        temp = a;
        a = b % a;
        b = temp;
    }
    return b;
}

impl Solution {
    pub fn mirror_reflection(p: i32, q: i32) -> i32 {
        // Gave up and read the editorial at https://leetcode.com/problems/mirror-reflection/solution/
        let g = gcd(p, q);
        let pp = (p / g) % 2;
        let qq = (q / g) % 2;
        if pp == 1 && qq == 1 {
            return 1;
        } else if pp == 1 {
            return 0;
        }
        return 2;
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test() {
        assert_eq!(Solution::mirror_reflection(2, 1), 2);
        assert_eq!(Solution::mirror_reflection(2, 2), 1);
        assert_eq!(Solution::mirror_reflection(2, 0), 0);
    }
}
