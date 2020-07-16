// https://leetcode.com/problems/powx-n/

struct Solution {}

impl Solution {
    pub fn my_pow(x: f64, n: i32) -> f64 {
        // return x.powi(n);
        let mut answer: f64 = 1.0;
        let mut base = x;
        let mut expo: i64 = n as i64;
        if expo < 0 {
            base = 1.0 / base;
            expo = expo.abs();
        }
        while expo > 0 {
            if (expo & 1) == 1 {
                answer *= base;
            }
            base *= base;
            expo >>= 1;
        }
        return answer;
    }
}

// maybe impl for all float types with generics ?
fn f64_assert_delta(x: f64, y: f64, d: f64) {
    if x.is_finite() && y.is_finite() {
        let diff = (x - y).abs();
        if !(diff < d) {
            panic!(format!(
                "actual {x} is not close to expected {y}, diff is {diff}",
                x = x,
                y = y,
                diff = diff
            ));
        }
    } else {
        if !(x == y) {
            panic!(format!(
                "actual {x} is not equal to expected {y}",
                x = y,
                y = y
            ));
        }
    }
}

fn test(x: f64, n: i32, expected: Option<f64>) {
    match expected {
        Some(e) => {
            f64_assert_delta(Solution::my_pow(x, n), e, 1e-6);
        }
        None => {
            f64_assert_delta(Solution::my_pow(x, n), x.powi(n), 1e-6);
        }
    }
}

fn main() {
    test(2.00000, 10, Some(1024.00000));
    test(2.10000, 3, Some(9.26100));
    test(2.00000, -2, Some(0.25000));
    test(-2.00000, 3, Some(-8.0));
    test(0.0, 0, Some(1.0));
    test(0.0, 1, Some(0.0));
    test(100.0, 0, Some(1.0));
    test(100.0, i32::MAX, Some(f64::INFINITY));
    test(100.0, i32::MIN, Some(0.0));
    test(-100.0, i32::MAX, Some(f64::NEG_INFINITY));
    test(-100.0, i32::MIN, Some(0.0));
    test(99.9999999, i32::MAX, None);
}
