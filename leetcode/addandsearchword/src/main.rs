// https://leetcode.com/problems/add-and-search-word-data-structure-design/

use std::collections::HashMap;

struct TrieNode {
    terminal: bool,
    children: HashMap<char, TrieNode>,
}

impl TrieNode {
    fn new() -> Self {
        return TrieNode {
            terminal: false,
            children: HashMap::new(),
        };
    }
}

struct WordDictionary {
    root: TrieNode,
}

impl WordDictionary {
    fn new() -> Self {
        return WordDictionary {
            root: TrieNode::new(),
        };
    }

    pub fn add_word(&mut self, word: String) {
        let mut current: &mut TrieNode = &mut self.root;
        for c in word.chars() {
            current = current.children.entry(c).or_insert(TrieNode::new());
        }
        current.terminal = true;
    }

    fn _search(&self, node: &TrieNode, rem: &[char]) -> bool {
        if rem.len() == 0 {
            return node.terminal;
        }
        let mut answer = false;
        match rem[0] {
            '.' => {
                for (_k, v) in node.children.iter() {
                    answer |= self._search(&v, &rem[1..]);
                    if answer {
                        break;
                    }
                }
            }
            c => {
                if node.children.contains_key(&c) {
                    answer = self._search(&node.children[&c], &rem[1..]);
                }
            }
        }
        return answer;
    }

    pub fn search(&self, word: String) -> bool {
        let chars: Vec<char> = word.chars().collect();
        return self._search(&self.root, &chars);
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::WordDictionary;

    #[test]
    fn test() {
        let mut dict = WordDictionary::new();
        dict.add_word(String::from("bad"));
        dict.add_word(String::from("dad"));
        dict.add_word(String::from("mad"));
        assert_eq!(dict.search(String::from("pad")), false);
        assert_eq!(dict.search(String::from("bad")), true);
        assert_eq!(dict.search(String::from("padsdfsadfasdf")), false);
        assert_eq!(dict.search(String::from(".ad")), true);
        assert_eq!(dict.search(String::from("b..")), true);
    }
}
