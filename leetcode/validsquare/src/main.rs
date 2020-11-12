// https://leetcode.com/problems/valid-square/

struct Solution {}

fn dist_squared(first: &Vec<i32>, second: &Vec<i32>) -> i64 {
    let x1 = first[0] as i64;
    let y1 = first[1] as i64;
    let x2 = second[0] as i64;
    let y2 = second[1] as i64;
    return (x1 - x2) * (x1 - x2) + (y1 - y2) * (y1 - y2);
}

impl Solution {
    pub fn valid_square(p1: Vec<i32>, p2: Vec<i32>, p3: Vec<i32>, p4: Vec<i32>) -> bool {
        let d12 = dist_squared(&p1, &p2);
        let d13 = dist_squared(&p1, &p3);
        let d14 = dist_squared(&p1, &p4);
        let d23 = dist_squared(&p2, &p3);
        let d24 = dist_squared(&p2, &p4);
        let d34 = dist_squared(&p3, &p4);

        if d12 == 0 || d13 == 0 || d14 == 0 || d23 == 0 || d24 == 0 || d34 == 0 {
            return false;
        }
        return (d12 == d14 && d14 == d34 && d34 == d23 && 2 * d12 == d13 && 2 * d12 == d24)
            || (d12 == d13 && d13 == d34 && d34 == d24 && 2 * d12 == d14 && 2 * d12 == d23)
            || (d13 == d14 && d14 == d24 && d24 == d23 && 2 * d13 == d12 && 2 * d13 == d34);
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test() {
        assert_eq!(
            Solution::valid_square(vec![0, 0], vec![1, 1], vec![1, 0], vec![0, 1]),
            true
        );
        assert_eq!(
            Solution::valid_square(vec![0, 0], vec![0, 0], vec![0, 0], vec![0, 0]),
            false
        );
        // regression
        assert_eq!(
            Solution::valid_square(vec![0, 0], vec![0, 1], vec![0, 2], vec![1, 2]),
            false
        );
        assert_eq!(
            Solution::valid_square(vec![0, 0], vec![0, 1], vec![1, 1], vec![1, 2]),
            false
        );
    }
}
