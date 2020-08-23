use std::collections::HashMap;
use std::ops::Deref;

#[derive(Debug, Clone)]
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

    fn insert(&mut self, word: String) {
        let mut current = self;
        for c in word.chars() {
            current = current.children.entry(c).or_insert(TrieNode::new());
        }
        current.terminal = true;
    }
}

struct Immutable<T> {
    value: T,
}

impl<T> Immutable<T> {
    pub fn new(value: T) -> Self {
        Immutable { value }
    }
}

impl<T> Deref for Immutable<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}
struct StreamChecker<'t> {
    root: Immutable<TrieNode>,
    threads: Vec<&'t TrieNode>,
}

impl<'t> StreamChecker<'t> {
    fn new(words: Vec<String>) -> Self {
        let mut root = TrieNode::new();
        for word in words {
            root.insert(word);
        }
        return Self {
            root: Immutable::new(root),
            threads: vec![],
        };
    }

    fn query(&mut self, letter: char) -> bool {
        self.threads.push(&self.root);
        let mut i: usize = 0;
        loop {
            let n = self.threads.len();
            if i >= n {
                break;
            }

            if self.threads[i].children.contains_key(&letter) {
                self.threads[i] = &self.threads[i].children[&letter];
                i += 1;
            } else {
                self.threads.swap(i, n - 1);
                self.threads.pop();
            }
        }

        return self.threads.iter().any(|&node| node.terminal);
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::StreamChecker;

    #[test]
    fn test() {
        let mut stream_checker = StreamChecker::new(vec![
            String::from("cd"),
            String::from("f"),
            String::from("kl"),
        ]);
        assert_eq!(stream_checker.query('a'), false); // return false
        assert_eq!(stream_checker.query('b'), false); // return false
        assert_eq!(stream_checker.query('c'), false); // return false
        assert_eq!(stream_checker.query('d'), true); // return true, because 'cd' is in the wordlist
        assert_eq!(stream_checker.query('e'), false); // return false
        assert_eq!(stream_checker.query('f'), true); // return true, because 'f' is in the wordlist
        assert_eq!(stream_checker.query('g'), false); // return false
        assert_eq!(stream_checker.query('h'), false); // return false
        assert_eq!(stream_checker.query('i'), false); // return false
        assert_eq!(stream_checker.query('j'), false); // return false
        assert_eq!(stream_checker.query('k'), false); // return false
        assert_eq!(stream_checker.query('l'), true); // return true, because 'kl' is in the wordlist
    }
}
