// https://leetcode.com/problems/robot-bounded-in-circle/

#[derive(Copy, Clone, PartialEq, Eq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

struct Solution {}

impl Solution {
    pub fn is_robot_bounded(instructions: String) -> bool {
        let changes = [(0, 1), (1, 0), (0, -1), (-1, 0)];
        let ltransform = [
            Direction::Left,
            Direction::Up,
            Direction::Right,
            Direction::Down,
        ];
        let rtransform = [
            Direction::Right,
            Direction::Down,
            Direction::Left,
            Direction::Up,
        ];
        let mut x = 0;
        let mut y = 0;
        let mut dir = Direction::Up;
        for c in instructions.chars() {
            match c {
                'G' => {
                    let (offx, offy) = changes[dir as usize];
                    x += offx;
                    y += offy;
                }
                'L' => {
                    dir = ltransform[dir as usize];
                }
                'R' => {
                    dir = rtransform[dir as usize];
                }
                _ => {
                    unreachable!("Invalid Input Sequence!");
                }
            }
        }
        return (x == 0 && y == 0) || dir != Direction::Up;
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test() {
        assert_eq!(Solution::is_robot_bounded(String::from("GGLLGG")), true);
        assert_eq!(Solution::is_robot_bounded(String::from("GG")), false);
        assert_eq!(Solution::is_robot_bounded(String::from("GL")), true);
    }
}
