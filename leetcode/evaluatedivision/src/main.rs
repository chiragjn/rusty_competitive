// https://leetcode.com/problems/evaluate-division/

// TODO: Re-write union find using a struct, too much argument passing
use std::collections::HashMap;

fn find(u: usize, parents: &mut Vec<usize>, multipliers: &mut Vec<f64>) -> (usize, f64) {
    if u != parents[u] {
        let (up, p2up_multiplier) = find(parents[u], parents, multipliers);
        multipliers[u] = multipliers[u] * p2up_multiplier;
        parents[u] = up;
    }
    return (parents[u], multipliers[u]);
}

fn union(
    u: usize,
    v: usize,
    multiplier: f64,
    parents: &mut Vec<usize>,
    ranks: &mut Vec<usize>,
    multipliers: &mut Vec<f64>,
) {
    // we know u = multiplier x v
    // we find u = u2up_multiplier x up
    // we find v = v2vp_multiplier x vp
    // u2up_multiplier x up = multiplier x v2vp_multiplier x vp
    // when we say up is the parent of vp,
    // we ask vp is what times up -> u2up_multiplier/(multiplier * v2vp_multiplier)
    let (up, u2up_multiplier) = find(u, parents, multipliers);
    let (vp, v2vp_multiplier) = find(v, parents, multipliers);
    if up != vp {
        if ranks[up] >= ranks[vp] {
            parents[vp] = up;
            ranks[up] += ranks[vp];
            multipliers[vp] = u2up_multiplier / (multiplier * v2vp_multiplier);
        } else {
            parents[up] = vp;
            ranks[vp] += ranks[up];
            multipliers[up] = (multiplier * v2vp_multiplier) / u2up_multiplier;
        }
    }
}

struct Solution {}

impl Solution {
    pub fn calc_equation(
        equations: Vec<Vec<String>>,
        values: Vec<f64>,
        queries: Vec<Vec<String>>,
    ) -> Vec<f64> {
        let mut symbols2id: HashMap<String, usize> = HashMap::new();
        let mut n: usize = 0;
        for equation in equations.iter() {
            for symbol in equation.iter() {
                if !symbols2id.contains_key(symbol) {
                    symbols2id.insert(symbol.clone(), n);
                    n += 1;
                }
            }
        }
        let mut parents: Vec<usize> = (0..n).collect();
        let mut ranks: Vec<usize> = vec![1; n];
        let mut multipliers: Vec<f64> = vec![1.0; n];
        for (equation, &value) in equations.iter().zip(values.iter()) {
            union(
                symbols2id[&equation[0]],
                symbols2id[&equation[1]],
                value,
                &mut parents,
                &mut ranks,
                &mut multipliers,
            );
        }
        for i in 0..n {
            find(i, &mut parents, &mut multipliers);
        }
        let mut answers: Vec<f64> = vec![];
        for query in queries.iter() {
            if !symbols2id.contains_key(&query[0]) || !symbols2id.contains_key(&query[1]) {
                answers.push(-1.0);
            } else {
                let (nump, num) = find(symbols2id[&query[0]], &mut parents, &mut multipliers);
                let (denp, den) = find(symbols2id[&query[1]], &mut parents, &mut multipliers);
                if nump == denp {
                    answers.push(num / den);
                } else {
                    answers.push(-1.0);
                }
            }
        }
        return answers;
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::Solution;

    fn assert_almost_vec(first: Vec<f64>, second: Vec<f64>, abs_tol: f64) {
        assert!(first
            .iter()
            .zip(second.iter())
            .all(|(&x, &y)| (x - y).abs() < abs_tol));
    }

    #[test]
    fn test() {
        assert_almost_vec(
            Solution::calc_equation(
                vec![
                    vec![String::from("a"), String::from("d")],
                    vec![String::from("ab"), String::from("cd")],
                    vec![String::from("ce"), String::from("ad")],
                ],
                vec![1.0, 2.0, 3.0],
                vec![
                    vec![String::from("a"), String::from("c")],
                    vec![String::from("a"), String::from("b")],
                ],
            ),
            vec![-1.0, -1.0],
            1e-6,
        );
        assert_almost_vec(
            Solution::calc_equation(
                vec![
                    vec![String::from("a"), String::from("b")],
                    vec![String::from("b"), String::from("c")],
                ],
                vec![2.0, 3.0],
                vec![
                    vec![String::from("a"), String::from("c")],
                    vec![String::from("b"), String::from("a")],
                    vec![String::from("a"), String::from("e")],
                    vec![String::from("a"), String::from("a")],
                    vec![String::from("x"), String::from("x")],
                ],
            ),
            vec![6.00000, 0.50000, -1.00000, 1.00000, -1.00000],
            1e-6,
        );
        assert_almost_vec(
            Solution::calc_equation(
                vec![
                    vec![String::from("a"), String::from("b")],
                    vec![String::from("b"), String::from("c")],
                    vec![String::from("bc"), String::from("cd")],
                ],
                vec![1.5, 2.5, 5.0],
                vec![
                    vec![String::from("a"), String::from("c")],
                    vec![String::from("c"), String::from("b")],
                    vec![String::from("bc"), String::from("cd")],
                    vec![String::from("cd"), String::from("bc")],
                ],
            ),
            vec![3.75000, 0.40000, 5.00000, 0.20000],
            1e-6,
        );
        assert_almost_vec(
            Solution::calc_equation(
                vec![vec![String::from("a"), String::from("b")]],
                vec![0.5],
                vec![
                    vec![String::from("a"), String::from("b")],
                    vec![String::from("b"), String::from("a")],
                    vec![String::from("a"), String::from("c")],
                    vec![String::from("x"), String::from("y")],
                ],
            ),
            vec![0.50000, 2.00000, -1.00000, -1.00000],
            1e-6,
        );
    }
}
