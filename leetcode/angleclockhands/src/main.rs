// https://leetcode.com/problems/angle-between-hands-of-a-clock/

struct Solution {}

impl Solution {
    pub fn angle_clock(hour: i32, minutes: i32) -> f64 {
        let hour_offset: f64 = ((hour % 12) as f64 + (minutes as f64 / 60.0)) * 30.0;
        let minutes_offset: f64 = minutes as f64 * 6.0;
        let answer = hour_offset.max(minutes_offset) - hour_offset.min(minutes_offset);
        return answer.min(360.0 - answer);
    }
}

macro_rules! assert_delta {
    ($x:expr, $y:expr, $d:expr) => {
        if !($x - $y < $d || $y - $x < $d) {
            panic!();
        }
    };
}

fn main() {
    assert_delta!(Solution::angle_clock(12, 30), 165.0, 1e-5);
    assert_delta!(Solution::angle_clock(3, 30), 75.0, 1e-5);
    assert_delta!(Solution::angle_clock(3, 15), 7.5, 1e-5);
    assert_delta!(Solution::angle_clock(4, 50), 155.0, 1e-5);
    assert_delta!(Solution::angle_clock(12, 00), 0.0, 1e-5);
}
