use std::cmp::min;
use std::collections::BTreeMap;
use std::io::{self, BufRead, Read, Stdin};
use std::str::FromStr;

struct InputUtils {
    stream: Stdin,
    buffer: String,
}

impl Default for InputUtils {
    fn default() -> Self {
        return InputUtils {
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
    let t: u32 = iu.input();
    for _ in 0..t {
        let n: u32 = iu.input();
        let numbers: Vec<u32> = iu.inputs(None);
        let mut answer: Vec<u32> = vec![0; numbers.len()];
        let mut number2idxs: BTreeMap<u32, Vec<usize>> = BTreeMap::new();
        // numbers are from 1 to N
        for (i, &number) in numbers.iter().enumerate() {
            number2idxs.entry(number).or_insert(Vec::new()).push(i);
        }
        let mut current_max: i32;
        let mut current_assign: i32 = -1;
        for &number in number2idxs.keys() {
            current_max = (number as i32) - 1;
            for &idx in number2idxs[&number].iter() {
                current_assign = min(current_assign + 1, current_max);
                answer[idx] = current_assign as u32;
            }
        }
        print_vec!(answer, " ");
    }
}
