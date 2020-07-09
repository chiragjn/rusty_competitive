// https://codeforces.com/contest/1352/problem/B

use std::io::{self, BufRead, Read, Stdin};
use std::iter;
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
    ($x: expr) => {
        println!(
            "{}",
            $x.iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join(" ")
        );
    };

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
    let t: i32 = iu.input();
    let (mut n, mut k, mut r): (i32, i32, i32);
    let mut vec_: Vec<i32>;
    for ti in 0..t {
        vec_ = iu.inputs(None);
        if let [n, k] = &vec_[..] {
            r = n - (2 * (k - 1));
            if r > 0 && r % 2 == 0 {
                println!("YES");
                for _ in 0..(k - 1) {
                    print!("{} ", 2);
                }
                println!("{}", r);
                continue;
            }

            r = n - (k - 1);
            if r > 0 && r % 2 == 1 {
                println!("YES");
                for _ in 0..(k - 1) {
                    print!("{} ", 1);
                }
                println!("{}", r);
                continue;
            }

            println!("NO");
        }
    }
}
