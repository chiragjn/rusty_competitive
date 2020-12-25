use std::collections::HashMap;
use std::io::{self, BufRead, Stdin};

struct InputUtils {
    stream: Stdin,
    buffer: String,
}

impl Default for InputUtils {
    fn default() -> Self {
        return Self {
            stream: io::stdin(),
            buffer: String::new(),
        };
    }
}

impl Iterator for InputUtils {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        match self.stream.lock().lines().next() {
            Some(line) => Some(line.unwrap().trim().to_string()),
            None => None,
        }
    }
}

/// finds x in a ^ x ≡ b (mod m) when a and m are coprime
/// https://cp-algorithms.com/algebra/discrete-log.html
fn discrete_log(a: i64, b: i64, m: i64) -> Result<i64, i64> {
    let a = a % m;
    let b = b % m;
    let n = (m as f64).sqrt() as i64 + 1;
    let mut a_pow_n = 1;
    for _ in 0..n {
        a_pow_n = (a_pow_n * a) % m;
    }
    let mut values: HashMap<i64, i64> = HashMap::new();
    let mut current: i64 = b;
    for q in 0..=n {
        values.insert(current, q);
        current = (current * a) % m;
    }
    current = 1;
    for p in 1..=n {
        current = (current * a_pow_n) % m;
        if values.contains_key(&current) {
            return Ok(n * p - values[&current]);
        }
    }
    return Err(-1);
}

fn encrypt(a: i64, b: i64, m: i64) -> i64 {
    let mut ap = a % m;
    let mut b = b;
    let mut answer = 1;
    while b > 0 {
        if b & 1 == 1 {
            answer = (answer * ap) % m;
        }
        ap = (ap * ap) % m;
        b >>= 1;
    }
    return answer;
}

fn solve(mut lines: Box<dyn Iterator<Item = String>>) -> i64 {
    let a: i64 = 7;
    let m: i64 = 20201227;
    let card_key: i64 = lines
        .next()
        .and_then(|line| line.parse().ok())
        .expect("Failed to read card public key!");
    let door_key: i64 = lines
        .next()
        .and_then(|line| line.parse().ok())
        .expect("Failed to read card public key!");
    let card_loop_size = discrete_log(a, card_key, m).expect("Failed to find a solution!");
    return encrypt(door_key, card_loop_size, m);
}

fn main() {
    let iu = InputUtils::default();
    let boxed_iter = Box::new(iu);
    println!("{}", solve(boxed_iter));
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test() {
        let test_input = r#"5764801
17807724"#;
        let it = test_input
            .split('\n')
            .into_iter()
            .map(|part| part.to_string());
        assert_eq!(solve(Box::new(it)), 14897079);
    }
}
