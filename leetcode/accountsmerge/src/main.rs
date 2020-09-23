// https://leetcode.com/problems/accounts-merge/

use std::collections::HashMap;

struct Solution {}

fn find(u: usize, parents: &mut Vec<usize>) -> usize {
    if parents[u] == u {
        return u;
    }
    parents[u] = find(parents[u], parents);
    return parents[u];
}

fn union(u: usize, v: usize, ranks: &mut Vec<u32>, parents: &mut Vec<usize>) {
    let up = find(u, parents);
    let vp = find(v, parents);
    if up != vp {
        if ranks[up] >= ranks[vp] {
            parents[vp] = up;
            ranks[up] += ranks[vp];
        } else {
            parents[up] = vp;
            ranks[vp] += ranks[up];
        }
    }
}

impl Solution {
    pub fn accounts_merge(accounts: Vec<Vec<String>>) -> Vec<Vec<String>> {
        let mut email2idx: HashMap<&String, usize> = HashMap::new();
        let mut idx2email: HashMap<usize, &String> = HashMap::new();
        let mut email2name: HashMap<usize, &String> = HashMap::new();
        let mut counter: usize = 0;
        for account in accounts.iter() {
            let name = &account[0];
            for email in account[1..].iter() {
                if !email2idx.contains_key(&email) {
                    email2idx.insert(email, counter);
                    email2name.insert(counter, name);
                    idx2email.insert(counter, email);
                    counter += 1;
                }
            }
        }
        let mut parents: Vec<usize> = (0..counter).collect();
        let mut ranks: Vec<u32> = vec![1; counter];

        // merge accounts
        for account in accounts.iter() {
            let u = email2idx[&account[1]];
            for email in account[1..].iter() {
                let v = email2idx[&email];
                union(u, v, &mut ranks, &mut parents);
            }
        }

        // flatten everything
        for i in 0..counter {
            find(i, &mut parents);
        }

        let mut parent2children: HashMap<usize, Vec<usize>> = HashMap::new();
        for (i, &parent) in parents.iter().enumerate() {
            parent2children.entry(parent).or_default().push(i);
        }

        let mut answer: Vec<Vec<String>> = vec![];
        for (parent, children) in parent2children.iter() {
            let mut row: Vec<String> = vec![];
            row.push(email2name[parent].clone());
            for child in children.iter() {
                row.push(idx2email[child].clone());
            }
            row.sort();
            answer.push(row);
        }
        answer.sort_by(|x, y| x.cmp(&y));
        return answer;
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::Solution;

    macro_rules! S {
        ($($s:expr),*) => {
            vec![$(String::from($s),)*]
        };
    }

    #[test]
    fn test() {
        assert_eq!(
            Solution::accounts_merge(vec![
                S!["John", "johnsmith@mail.com", "john00@mail.com"],
                S!["John", "johnnybravo@mail.com"],
                S!["John", "johnsmith@mail.com", "john_newyork@mail.com"],
                S!["Mary", "mary@mail.com"]
            ]),
            vec![
                S![
                    "John",
                    "john00@mail.com",
                    "john_newyork@mail.com",
                    "johnsmith@mail.com"
                ],
                S!["John", "johnnybravo@mail.com"],
                S!["Mary", "mary@mail.com"]
            ]
        )
    }
}
