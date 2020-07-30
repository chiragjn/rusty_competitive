// https://leetcode.com/problems/word-break-ii/
// This solution might be an overkill, learn from other submissions

use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;
use std::mem::swap;

struct TrieNode {
    value: char,
    is_terminal: bool,
    children: HashMap<char, TrieNode>,
}

impl TrieNode {
    fn new(value: char) -> Self {
        return TrieNode {
            value: value,
            is_terminal: false,
            children: HashMap::new(),
        };
    }

    fn insert(&mut self, word: &String) {
        let mut current = self;
        for c in word.chars() {
            current = current.children.entry(c).or_insert(TrieNode::new(c));
        }
        current.is_terminal = true
    }
}

struct Solver<'a> {
    trie_root: &'a TrieNode,
    sentences: Vec<String>,
    buffer: String,
}

impl<'a> Solver<'a> {
    fn new(trie_root: &'a TrieNode) -> Self {
        return Solver {
            trie_root: trie_root,
            sentences: vec![],
            buffer: String::new(),
        };
    }

    fn _recurse(&mut self, rem: &str, node: &TrieNode) {
        if rem.len() == 0 {
            if node.value == self.trie_root.value {
                self.sentences
                    .push(self.buffer.chars().take(self.buffer.len() - 1).collect());
            }
            return;
        }
        let this_char = rem.chars().next().unwrap();
        if node.children.contains_key(&this_char) {
            let next_node = node.children.get(&this_char).unwrap();
            if next_node.is_terminal {
                self.buffer.push(this_char);
                self.buffer.push(' ');
                self._recurse(&rem[1..], self.trie_root);
                self.buffer.pop();
                self.buffer.pop();
            }
            self.buffer.push(this_char);
            self._recurse(&rem[1..], &next_node);
            self.buffer.pop();
        }
    }

    fn find_all_sentences(&mut self, s: &String) -> Vec<String> {
        self._recurse(&s[..], self.trie_root);
        let mut answer = vec![];
        swap(&mut answer, &mut self.sentences);
        return answer;
    }
}

struct Solution {}

impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> Vec<String> {
        // check to prevent catastrophic backtracking!
        let dict_charset =
            HashSet::<char>::from_iter(word_dict.iter().map(|word| word.chars()).flatten());
        for c in s.chars() {
            if !dict_charset.contains(&c) {
                return vec![];
            }
        }
        let mut root = TrieNode::new('!');
        for word in word_dict.iter() {
            root.insert(word);
        }
        let mut solver = Solver::new(&root);
        return solver.find_all_sentences(&s);
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::Solution;
    use std::collections::HashSet;
    use std::iter::FromIterator;

    macro_rules! S {
        ($s: expr) => {
            String::from($s);
        };
    }

    fn _unordered_compare(first: Vec<String>, second: Vec<String>) {
        assert_eq!(
            HashSet::<String>::from_iter(first),
            HashSet::from_iter(second)
        );
    }

    #[test]
    fn test() {
        _unordered_compare(
            Solution::word_break(
                S!("catsanddog"),
                vec![S!("cat"), S!("cats"), S!("and"), S!("sand"), S!("dog")],
            ),
            vec![S!("cats and dog"), S!("cat sand dog")],
        );

        _unordered_compare(
            Solution::word_break(
                S!("pineapplepenapple"),
                vec![
                    S!("apple"),
                    S!("pen"),
                    S!("applepen"),
                    S!("pine"),
                    S!("pineapple"),
                ],
            ),
            vec![
                S!("pine apple pen apple"),
                S!("pineapple pen apple"),
                S!("pine applepen apple"),
            ],
        );

        _unordered_compare(
            Solution::word_break(
                S!("catsandog"),
                vec![S!("cats"), S!("dog"), S!("sand"), S!("and"), S!("cat")],
            ),
            vec![],
        );
    }
}
