// https://codeforces.com/contest/166/problem/E

use std::io::{self, BufRead, Read, Stdin};
use std::str::FromStr;

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

impl InputUtils {
    fn read(&mut self) -> &str {
        self.buffer.clear();
        self.stream
            .lock()
            .read_to_string(&mut self.buffer)
            .expect("Failed to read till EOF");
        return self.buffer.trim();
    }

    fn input<T>(&mut self) -> T
    where
        T: FromStr,
        T::Err: std::fmt::Debug,
    {
        return self
            .into_iter()
            .next()
            .unwrap()
            .parse::<T>()
            .expect("Failed to parse line to type");
    }

    fn inputs<T>(&mut self, delimiter: Option<&str>) -> Vec<T>
    where
        T: FromStr,
        T::Err: std::fmt::Debug,
    {
        let line = self.into_iter().next().unwrap();
        let parts = match delimiter {
            Some(delim) => line
                .split(delim)
                .map(|part| part.parse::<T>().expect("Failed to parse part"))
                .collect(),
            None => line
                .split_whitespace()
                .map(|part| part.parse::<T>().expect("Failed to parse part"))
                .collect(),
        };
        return parts;
    }
}

macro_rules! print_vec {
    ($x:expr, $y:expr) => {
        println!(
            "{}",
            $x.iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join($y)
        );
    };
}

fn main() {
    let mut iu = InputUtils::default();
    let modulo: i64 = 1000000007;
    let n: i32 = iu.input();
    let mut a: i64 = 1;
    let mut an: i64;
    let mut other: i64 = 0;
    let mut othern: i64;
    for _ in 0..n {
        an = other % modulo;
        othern = (((3i64 * a) % modulo) + ((2i64 * other) % modulo)) % modulo;
        a = an;
        other = othern;
    }
    println!("{}", a);
}
