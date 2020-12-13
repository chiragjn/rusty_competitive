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

fn extract_bus_ids(line: String) -> Option<Vec<i64>> {
    let parsed: Vec<i64> = line
        .split(",")
        .filter(|&part| part != "x")
        .map(|part| {
            part.parse::<i64>()
                .expect(&format!("Failed to cast bus id {} to i64", part))
        })
        .collect();
    return Some(parsed);
}

fn multiple_of_a_gte_b(a: i64, b: i64) -> i64 {
    return a * ((b as f64 / a as f64).ceil() as i64);
}

fn solve(mut lines: Box<dyn Iterator<Item = String>>) -> i64 {
    let depart_after: i64 = lines
        .next()
        .and_then(|line| line.parse().ok())
        .expect("Failed to get the time to depart after");
    let bus_ids: Vec<i64> = lines
        .next()
        .and_then(extract_bus_ids)
        .expect("Failed to parse bus ids");
    let (next_pickup, bus_id) = bus_ids
        .iter()
        .map(|&bid| multiple_of_a_gte_b(bid, depart_after))
        .zip(bus_ids.iter())
        .min()
        .expect("No buses to choose from!");
    return (next_pickup - depart_after) * bus_id;
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
        let test_input = r#"939
7,13,x,x,59,x,31,19"#;
        let it = test_input
            .split('\n')
            .into_iter()
            .map(|part| part.to_string());
        assert_eq!(solve(Box::new(it)), 295);
    }
}
