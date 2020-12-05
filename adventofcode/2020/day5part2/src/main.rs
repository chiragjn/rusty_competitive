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

fn bsp_decode(code: &[char]) -> i64 {
    let mut low = 0;
    let mut high = (1 << code.len()) - 1;
    for &c in code.iter() {
        let mid = low + ((high - low) / 2);
        if c == 'L' || c == 'F' {
            high = mid;
        } else {
            low = mid + 1;
        }
    }
    assert!(low == high);
    return low as i64;
}

fn decode(seat_code: String) -> i64 {
    let seat_code: Vec<char> = seat_code.chars().collect();
    return bsp_decode(&seat_code[0..7]) * 8 + bsp_decode(&seat_code[7..10]);
}

fn solve(lines: Box<dyn Iterator<Item = String>>) -> i64 {
    let mut seat_numbers: Vec<i64> = lines.map(|line| decode(line)).collect();
    seat_numbers.sort();
    for i in 1..seat_numbers.len() {
        if seat_numbers[i] - seat_numbers[i - 1] > 1 {
            return seat_numbers[i] - 1;
        }
    }
    unreachable!("All seats are full!");
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
        //        let test_input = r#"BFFFBBFRRR
        //FFFBBBFRRR
        //BBFFBBFRLL"#;
        //        let it = test_input
        //            .split('\n')
        //            .into_iter()
        //            .map(|part| part.to_string());
        //        assert_eq!(solve(Box::new(it)), 820);
    }
}
