// Some terrible design going on over here

struct BinTrieNode {
    left: Option<Box<BinTrieNode>>,
    right: Option<Box<BinTrieNode>>,
}

fn borrowed<'a, T>(val: &'a Option<Box<T>>) -> Option<&'a T> {
    match val {
        Some(ref v) => {
            return Some(v);
        }
        None => {
            return None;
        }
    }
}

impl BinTrieNode {
    fn new() -> Self {
        return Self {
            left: None,
            right: None,
        };
    }

    fn insert(&mut self, val: u32, bits: usize) {
        if bits == 0 {
            return;
        }
        if val & (1 << (bits - 1)) == 0 {
            if self.left.is_none() {
                self.left = Some(Box::new(BinTrieNode::new()));
            }
            let mut temp = self.left.take().unwrap();
            temp.insert(val, bits - 1);
            self.left = Some(temp);
        } else {
            if self.right.is_none() {
                self.right = Some(Box::new(BinTrieNode::new()));
            }
            let mut temp = self.right.take().unwrap();
            temp.insert(val, bits - 1);
            self.right = Some(temp);
        }
    }

    fn max_xor(&self, val: u32, bits: usize) -> u32 {
        let mut current: Option<&BinTrieNode> = Some(self);
        let mut answer: u32 = 0;
        println!("\nGiven {}", val);
        for i in (0..bits).rev() {
            match current {
                Some(c) => {
                    answer <<= 1;
                    if val & (1 << i) == 0 {
                        if c.right.is_some() {
                            current = borrowed(&c.right);
                            print!("1");
                            answer |= 1;
                        } else {
                            print!("0");
                            current = borrowed(&c.left);
                        }
                    } else {
                        if c.left.is_some() {
                            print!("0");
                            current = borrowed(&c.left);
                        } else {
                            print!("1");
                            current = borrowed(&c.right);
                            answer |= 1;
                        }
                    }
                }
                None => {
                    panic!("Max depth exceeded - `bits` > depth of Trie");
                }
            }
        }
        return answer;
    }
}

struct Solution {}

impl Solution {
    pub fn find_maximum_xor(nums: Vec<i32>) -> i32 {
        let mut root = BinTrieNode::new();
        nums.iter()
            .map(|&x| root.insert(x as u32, 32))
            .for_each(drop);
        return nums
            .iter()
            .map(|&x| root.max_xor(x as u32, 32))
            .max()
            .unwrap_or(0) as i32;
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test() {
        assert_eq!(Solution::find_maximum_xor(vec![3, 10, 5, 25, 2, 8]), 28);
        assert_eq!(Solution::find_maximum_xor(vec![3]), 0);
        assert_eq!(Solution::find_maximum_xor(vec![10, 0, 5]), 15);
    }
}
