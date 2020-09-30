// https://leetcode.com/problems/word-break/

use std::collections::HashMap;

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

fn _recurse(
    node: &TrieNode,
    i: usize,
    chars: &Vec<char>,
    root: &TrieNode,
    cache: &mut Vec<Option<bool>>,
) -> bool {
    if i == chars.len() && node.value == root.value {
        return true;
    }
    if cache[i].is_some() {
        return cache[i].unwrap();
    }
    let mut possible = false;
    let mut current: &TrieNode = node;
    for j in i..chars.len() {
        if current.children.contains_key(&chars[j]) {
            current = &current.children[&chars[j]];
            if current.is_terminal {
                possible |= _recurse(root, j + 1, chars, root, cache);
            }
        } else {
            break;
        }
    }
    cache[i] = Some(possible);
    return cache[i].unwrap();
}

struct Solution {}

impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        let mut root = TrieNode::new('!');
        for word in word_dict.iter() {
            root.insert(word);
        }
        let chars: Vec<char> = s.chars().collect();
        let mut cache = vec![None; chars.len()];
        return _recurse(&root, 0, &chars, &root, &mut cache);
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::Solution;

    macro_rules! S {
        ($s: expr) => {
            String::from($s);
        };
    }

    #[test]
    fn test() {
        assert_eq!(
            Solution::word_break(S!("leetcode"), vec![S!("leet"), S!("code")]),
            true
        );
        assert_eq!(
            Solution::word_break(S!("applepenapple"), vec![S!("apple"), S!("pen")]),
            true
        );
        assert_eq!(
            Solution::word_break(
                S!("catsandog"),
                vec![S!("cats"), S!("dog"), S!("sand"), S!("and"), S!("cat")]
            ),
            false
        );
    }
}
