// https://leetcode.com/problems/largest-time-for-given-digits/

struct Solution {}

impl Solution {
    pub fn largest_time_from_digits(a: Vec<i32>) -> String {
        // Thanks I hate it;
        let mut ac = a.clone();
        ac.sort();
        for times in (0..1440).rev() {
            let t = vec![
                (times / 60) / 10,
                (times / 60) % 10,
                (times % 60) / 10,
                (times % 60) % 10,
            ];
            let mut ti = t.clone();
            ti.sort();
            if ac == ti {
                return format!("{}{}:{}{}", t[0], t[1], t[2], t[3]);
            }
        }
        return String::new();
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test() {
        assert_eq!(
            Solution::largest_time_from_digits(vec![1, 2, 3, 4]),
            String::from("23:41")
        );
        assert_eq!(
            Solution::largest_time_from_digits(vec![5, 5, 5, 5]),
            String::from("")
        );
        assert_eq!(
            Solution::largest_time_from_digits(vec![2, 0, 6, 6]),
            String::from("06:26")
        );
    }
}
