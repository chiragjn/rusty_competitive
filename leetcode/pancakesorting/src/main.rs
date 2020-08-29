// https://leetcode.com/problems/pancake-sorting/

struct Solution {}

impl Solution {
    pub fn pancake_sort(a: Vec<i32>) -> Vec<i32> {
        // 1 <= A.length <= 100
        // All integers in A are unique (i.e. A is a permutation of the integers from 1 to A.length).
        // Any valid answer that sorts the array within 10 * A.length flips will be judged as correct.
        let mut a = a.clone();
        let mut flips: Vec<i32> = vec![];
        let mut cpos;
        for i in (2..a.len() as i32 + 1).rev() {
            cpos = a.iter().position(|&x| x == i).unwrap();
            if cpos == (i - 1) as usize {
                continue;
            }
            if cpos != 0 {
                flips.push(cpos as i32 + 1);
                a[..cpos + 1].reverse();
            }
            flips.push(i);
            a[..i as usize].reverse();
        }
        return flips;
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test() {
        let mut a = vec![3, 2, 4, 1];
        let flips = Solution::pancake_sort(a.clone());
        assert!(flips.len() <= a.len() * 10);
        assert_eq!(flips, vec![3, 4, 2, 3, 2]);
        for flip in flips {
            a[..flip as usize].reverse();
        }
        assert_eq!(a, vec![1, 2, 3, 4]);

        let mut a = vec![1, 2, 3];
        let flips = Solution::pancake_sort(a.clone());
        assert_eq!(flips.len(), 0);
        for flip in flips {
            a[..flip as usize].reverse();
        }
        assert_eq!(a, vec![1, 2, 3]);
    }
}
