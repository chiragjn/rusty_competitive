// https://leetcode.com/submissions/detail/380398955/
// 1 <= combinationLength <= characters.length <= 15

struct CombinationIterator {
    n: usize,
    r: usize,
    pool: Vec<char>,
    state: Vec<usize>,
    _has_next: bool,
}

impl CombinationIterator {
    fn new(characters: String, combination_length: i32) -> Self {
        let chars: Vec<char> = characters.chars().collect();
        return Self {
            n: chars.len(),
            r: combination_length as usize,
            pool: chars,
            state: (0..combination_length as usize).collect(),
            _has_next: true,
        };
    }

    fn next(&mut self) -> String {
        let item: String = self.state.iter().map(|&idx| self.pool[idx]).collect();
        self._has_next = false;
        for i in (0..self.r).rev() {
            if self.state[i] != i + self.n - self.r {
                self._has_next = true;
                self.state[i] += 1;
                for j in i + 1..self.r {
                    self.state[j] = self.state[j - 1] + 1;
                }
                break;
            }
        }
        return item;
    }

    fn has_next(&self) -> bool {
        return self._has_next;
    }
}

/**
 * Your CombinationIterator object will be instantiated and called as such:
 * let obj = CombinationIterator::new(characters, combinationLength);
 * let ret_1: String = obj.next();
 * let ret_2: bool = obj.has_next();
 */
fn main() {}

#[cfg(test)]
mod tests {
    use super::CombinationIterator;

    #[test]
    fn test() {
        let mut combinations: Vec<String> = vec![];
        let mut it = CombinationIterator::new(String::from("abcd"), 2);
        while it.has_next() {
            combinations.push(it.next());
        }
        assert_eq!(
            combinations,
            vec![
                String::from("ab"),
                String::from("ac"),
                String::from("ad"),
                String::from("bc"),
                String::from("bd"),
                String::from("cd")
            ]
        );
    }
}
