// https://leetcode.com/problems/largest-component-size-by-common-factor/

use std::collections::HashMap;

struct Solution {}

fn find(a: usize, parent: &mut Vec<usize>) -> usize {
    if parent[a] != a {
        parent[a] = find(parent[a], parent);
    }
    return parent[a];
}

fn union(a: usize, b: usize, parent: &mut Vec<usize>, rank: &mut Vec<i32>) {
    if a == b {
        return;
    }
    let aroot = find(a, parent);
    let broot = find(b, parent);
    if aroot == broot {
        return;
    }
    if rank[aroot] >= rank[broot] {
        rank[aroot] += rank[broot];
        parent[broot] = aroot;
    } else {
        rank[broot] += rank[aroot];
        parent[aroot] = broot;
    }
}
impl Solution {
    pub fn largest_component_size(a: Vec<i32>) -> i32 {
        if a.len() == 0 {
            return 0;
        }
        let n: usize = 100005;
        let mut is_prime = vec![true; 100005];
        is_prime[0] = false;
        is_prime[1] = false;
        let k = (n as f32).sqrt() as usize;
        for i in (3..(k + 1)).step_by(2) {
            if is_prime[i] {
                for j in ((i * i)..n).step_by(i) {
                    is_prime[j] = false;
                }
            }
        }
        let mut primes: Vec<usize> = vec![2];
        primes.extend((3..n).step_by(2).filter(|&x| is_prime[x]));
        let mut factor2elemidx: HashMap<usize, usize> = HashMap::new();
        let mut parent: Vec<usize> = (0..a.len()).collect();
        let mut rank: Vec<i32> = vec![1; a.len()];
        for (i, num) in a.iter().enumerate() {
            let mut numba = *num as usize;
            if numba % 2 == 1 && is_prime[numba] {
                let p = numba;
                factor2elemidx.entry(p).or_insert(i);
                union(i, factor2elemidx[&p], &mut parent, &mut rank);
            } else {
                let mut pidx = 0;
                while numba > 1 {
                    let p = primes[pidx];
                    if numba % p == 0 {
                        factor2elemidx.entry(p).or_insert(i);
                        union(i, factor2elemidx[&p], &mut parent, &mut rank);
                    }
                    while numba % p == 0 {
                        numba /= p;
                    }
                    pidx += 1;
                }
            }
        }
        let answer = (0..a.len())
            .map(|x| rank[find(x, &mut parent)])
            .max()
            .unwrap();
        return answer as i32;
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test() {
        assert_eq!(
            Solution::largest_component_size(vec![2, 3, 6, 7, 4, 12, 21, 39]),
            8
        );
        assert_eq!(Solution::largest_component_size(vec![4, 6, 15, 35]), 4);
        assert_eq!(Solution::largest_component_size(vec![20, 50, 9, 63]), 2);
    }

    #[test]
    fn test_large() {
        assert_eq!(
            Solution::largest_component_size((2..20001).step_by(2).collect()),
            10000
        );
    }
}
