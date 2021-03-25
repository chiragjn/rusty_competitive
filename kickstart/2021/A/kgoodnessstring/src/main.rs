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

fn main() {
    let mut iu = InputUtils::default();
    let t: i32 = iu.input();
    let mut k: i32;
    let mut s: Vec<char>;
    for ti in 1..t + 1 {
        iu.input::<i32>();
        k = iu.input();
        s = iu.line().chars().collect();
        let good = (0..(s.len() / 2))
            .map(|i| (s[i] != s[s.len() - 1 - i]) as i32)
            .sum::<i32>();
        println!("Case #{}: {}", ti, (k - good).abs());
    }
}
