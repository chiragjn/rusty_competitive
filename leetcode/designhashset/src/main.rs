// https://leetcode.com/problems/design-hashset/

// All values will be in the range of [0, 1000000].
// The number of operations will be in the range of [1, 10000].
// Constraints make it too simple to implement a HashSet naively not caring about reference
// stability, growing, collison resolution, cache efficiency.

struct MyHashSet {
    mem: Vec<bool>,
}

impl MyHashSet {
    fn new() -> Self {
        let sz: usize = 1 << 20;
        return MyHashSet {
            mem: vec![false; sz],
        };
    }

    fn add(&mut self, key: i32) {
        self.mem[key as usize] = true;
    }

    fn remove(&mut self, key: i32) {
        self.mem[key as usize] = false;
    }

    fn contains(&self, key: i32) -> bool {
        return self.mem[key as usize];
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::MyHashSet;

    #[test]
    fn test() {
        let mut hs = MyHashSet::new();
        assert_eq!(hs.contains(100), false);
        hs.add(100);
        assert_eq!(hs.contains(100), true);
        hs.add(100);
        assert_eq!(hs.contains(100), true);
        hs.add(10);
        assert_eq!(hs.contains(100), true);
        hs.remove(100);
        assert_eq!(hs.contains(100), true);
        hs.remove(100);
        assert_eq!(hs.contains(100), false);
    }
}
