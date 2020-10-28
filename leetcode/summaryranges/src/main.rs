// https://leetcode.com/problems/summary-ranges/

struct Solution {}

impl Solution {
    pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
        let mut answer: Vec<String> = vec![];
        if nums.is_empty() {
            return answer;
        }
        let mut beg = 0;
        while beg < nums.len() {
            let mut end = beg + 1;
            while end < nums.len() && (nums[end] - nums[end - 1]) == 1 {
                end += 1;
            }
            end -= 1;
            let mut range = nums[beg].to_string();
            if beg != end {
                range.push_str("->");
                range.push_str(nums[end].to_string().as_str());
            }
            answer.push(range);
            beg = end + 1;
        }
        return answer;
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test() {
        assert_eq!(
            Solution::summary_ranges(vec![0, 1, 2, 4, 5, 7]),
            vec![
                String::from("0->2"),
                String::from("4->5"),
                String::from("7")
            ]
        );
        assert_eq!(
            Solution::summary_ranges(vec![0, 2, 3, 4, 6, 8, 9]),
            vec![
                String::from("0"),
                String::from("2->4"),
                String::from("6"),
                String::from("8->9")
            ]
        );
        assert_eq!(Solution::summary_ranges(vec![]), Vec::<String>::new());
        assert_eq!(Solution::summary_ranges(vec![-1]), vec![String::from("-1")]);
        assert_eq!(Solution::summary_ranges(vec![0]), vec![String::from("0")]);
    }
}
