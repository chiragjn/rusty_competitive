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

fn modulo_inverse(a: i64, m: i64) -> i64 {
    // using extended euclidean algorithm
    let mut a = a;
    let mut m = m;
    let mc = m;
    let mut x = 0;
    let mut y = 1;
    let mut temp;

    if m == 1 {
        return 0;
    }

    while a > 1 {
        let quotient = a / m;
        temp = m;
        m = a % m;
        a = temp;
        temp = x;
        x = y - quotient * x;
        y = temp;
    }
    while y < 0 {
        y += mc;
    }
    return y;
}

fn extract_bus_ids(line: String) -> Option<Vec<(i64, i64)>> {
    let mut parsed: Vec<(i64, i64)> = vec![];
    for (i, part) in line.split(",").enumerate() {
        if part != "x" {
            let id = part
                .parse::<i64>()
                .expect(&format!("Failed to cast bus id {} to i64", part));
            let mut rem = id - i as i64;
            rem %= id;
            while rem < 0 {
                rem += id;
            }
            parsed.push((id, rem));
        }
    }
    return Some(parsed);
}

fn solve(mut lines: Box<dyn Iterator<Item = String>>) -> i64 {
    lines.next();
    let bus_ids_and_rems: Vec<(i64, i64)> = lines
        .next()
        .and_then(extract_bus_ids)
        .expect("Failed to parse bus ids");
    // chinese reaminder theorem
    let mut product: i64 = 1;
    for &(num, _) in bus_ids_and_rems.iter() {
        product *= num;
    }
    let mut result: i64 = 0;
    for &(num, rem) in bus_ids_and_rems.iter() {
        if rem != 0 {
            let p = product / num;
            result = result + rem * modulo_inverse(p, num) * p;
        }
    }
    result = result % product;
    return result;
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
        let tests: Vec<(&str, i64)> = vec![
            ("-\n7,13,x,x,59,x,31,19", 1068781),
            ("-\n17,x,13,19", 3417),
            ("-\n67,7,59,61", 754018),
            ("-\n67,x,7,59,61", 779210),
            ("-\n67,7,x,59,61", 1261476),
            ("-\n1789,37,47,1889", 1202161486),
        ];
        for (test_input, expected) in tests {
            let it = test_input
                .split('\n')
                .into_iter()
                .map(|part| part.to_string());
            assert_eq!(solve(Box::new(it)), expected);
        }
    }
}
