use std::io::{self, BufRead, Read, Stdin, StdinLock};
use std::str::FromStr;

struct InputUtils {
    stream: Stdin,
    _buffer: String,
    _vec_buffer: Vec<String>,
}

impl Default for InputUtils {
    fn default() -> Self {
        return Self {
            stream: io::stdin(),
            _buffer: String::new(),
            _vec_buffer: vec![],
        };
    }
}

impl Iterator for InputUtils {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        self._drain_vec_buffer();
        match self.stream.lock().lines().next() {
            Some(line) => Some(line.unwrap().trim().to_string()),
            None => None,
        }
    }
}

impl InputUtils {
    fn _drain_vec_buffer(&mut self) {
        self._vec_buffer.drain(..);
    }

    fn read(&mut self) -> &str {
        self._drain_vec_buffer();
        self._buffer.clear();
        self.stream
            .lock()
            .read_to_string(&mut self._buffer)
            .expect("Failed to read till EOF");
        return self._buffer.trim();
    }

    fn line(&mut self) -> String {
        return self.into_iter().next().unwrap();
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

    fn input<T>(&mut self) -> T
    where
        T: FromStr,
        T::Err: std::fmt::Debug,
    {
        loop {
            if let Some(token) = self._vec_buffer.pop() {
                return token.parse().ok().expect("Failed parse");
            }
            self._vec_buffer = self
                .into_iter()
                .next()
                .unwrap()
                .split_whitespace()
                .rev()
                .map(String::from)
                .collect();
        }
    }
}

fn relative_min_pos(slice: &[i32]) -> Option<usize> {
    if slice.len() > 0 {
        return slice
            .into_iter()
            .position(|e| e == slice.into_iter().min().unwrap());
    }
    return None;
}

fn reversort(numbers: &mut Vec<i32>) -> usize {
    if numbers.len() == 0 {
        return 0;
    }
    let mut min_idx: usize;
    let mut cost: usize = 0;
    for i in 0..numbers.len() - 1 {
        min_idx = i + relative_min_pos(&numbers[i..]).unwrap();
        &numbers[i..(min_idx + 1)].reverse();
        cost += min_idx - i + 1;
    }
    return cost;
}

fn main() {
    let mut iu = InputUtils::default();
    let t: i32 = iu.input();
    for ti in 1..t + 1 {
        let _: usize = iu.input();
        let mut numbers: Vec<i32> = iu.inputs(None);
        println!("Case #{}: {}", ti, reversort(&mut numbers));
    }
}
