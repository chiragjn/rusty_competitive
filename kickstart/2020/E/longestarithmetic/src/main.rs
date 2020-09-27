use std::cmp::{max, min, Ordering};
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

fn solve(array: &Vec<i32>) -> i32 {
    let mut counter = 1;
    let mut best = 2;
    let mut prev = array[1] - array[0];
    let mut this;
    for i in 2..array.len() {
        this = array[i] - array[i - 1];
        if this == prev {
            counter += 1;
        } else {
            counter = 1;
        }
        prev = this;
        best = max(best, counter + 1);
    }
    return best;
}

fn main() {
    let mut iu = InputUtils::default();
    let t: i32 = iu.input();
    let mut k: usize;
    for ti in 1..t + 1 {
        k = iu.input();
        let array: Vec<i32> = iu.inputs(None);
        println!("Case #{}: {}", ti, solve(&array));
    }
}
