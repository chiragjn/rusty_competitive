// https://leetcode.com/problems/random-point-in-non-overlapping-rectangles/

extern crate rand;
use rand::distributions::{Distribution, Uniform};
use rand::prelude::ThreadRng;

struct Solution {
    rects: Vec<Vec<i32>>,
    distribution: Uniform<i32>,
    rng: ThreadRng,
    mapped_rects: Vec<i32>,
}

impl Solution {
    fn new(rects: Vec<Vec<i32>>) -> Self {
        let mut mapped_rects = vec![0];
        let mut csum = 0;
        for rect in &rects {
            csum += (rect[2] - rect[0] + 1) * (rect[3] - rect[1] + 1);
            mapped_rects.push(csum);
        }
        return Self {
            rects: rects,
            distribution: Uniform::from(1..csum + 1),
            rng: rand::thread_rng(),
            mapped_rects: mapped_rects,
        };
    }

    fn pick(&mut self) -> Vec<i32> {
        let pick = self.distribution.sample(&mut self.rng);
        let (mut low, mut high, mut mid) = (0, self.mapped_rects.len() - 1, 0);
        let mut idx = 0;
        while low <= high {
            mid = low + ((high - low) / 2);
            if pick <= self.mapped_rects[mid] {
                idx = mid;
                high = mid - 1;
            } else {
                low = mid + 1;
            }
        }
        println!("{:?}, {:?}", (pick, idx), self.mapped_rects);
        let adjusted_pick = pick - self.mapped_rects[idx - 1] - 1;
        let picked_rect = &self.rects[idx - 1];
        let off_y = adjusted_pick / (picked_rect[2] - picked_rect[0] + 1);
        let off_x = adjusted_pick % (picked_rect[2] - picked_rect[0] + 1);
        return vec![picked_rect[0] + off_x, picked_rect[1] + off_y];
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test() {
        // let rects: Vec<Vec<i32>> = vec![vec![-2, -2, -1, -1], vec![1, 0, 3, 0]];
        let rects = vec![
            vec![-58953616, -40483558, -58953446, -40482555],
            vec![76369640, 94978791, 76371036, 94979394],
            vec![80970826, -37466957, 80971657, -37466388],
            vec![-79821573, -4177978, -79820536, -4177925],
        ];
        let mut solver = Solution::new(rects.clone());
        for _ in 0..100 {
            let picked = solver.pick();
            println!("{:?}", picked);
            assert!(rects.iter().any(|rect| picked[0] >= rect[0]
                && picked[0] <= rect[2]
                && picked[1] >= rect[1]
                && picked[1] <= rect[3]))
        }
    }
}
