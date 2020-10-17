use std::collections::HashSet;

struct Solution {}

impl Solution {
    pub fn find_repeated_dna_sequences(s: String) -> Vec<String> {
        let width: usize = 10;
        let chars: Vec<char> = s.chars().collect();
        let mut seen: HashSet<i64> = HashSet::new();
        let mut answer: Vec<String> = vec![];
        let mut hash: i64 = 0;
        for i in 0..(width - 1) {}
        for i in (width - 1)..s.len() {}
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
            Solution::find_repeated_dna_sequences(String::from("AAAAACCCCCAAAAACCCCCCAAAAAGGGTTT")),
            vec![String::from("AAAAACCCCC"), String::from("CCCCCAAAAA")]
        );
    }
}
