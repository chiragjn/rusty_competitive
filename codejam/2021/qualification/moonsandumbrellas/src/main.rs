use std::cmp::min;
use std::io::{self, BufRead, Read, Stdin};
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

fn solve(s: Vec<usize>, cj: i64, jc: i64, cache: &mut Vec<Vec<i64>>) -> i64 {
    let INF = 1000000000;
    let last = s.len();
    cache[last][0] = 0;
    cache[last][1] = 0;
    let mut cj_c = 0;
    let mut jc_c = 0;
    for idx in (0..last).rev() {
        cache[idx][0] = INF;
        cache[idx][1] = INF;
        if s[idx] == 0 || s[idx] == 2 {
            cache[idx][0] = min(INF, min(cache[idx + 1][0], cj_c + cache[idx + 1][1]));
        }
        if s[idx] == 1 || s[idx] == 2 {
            cache[idx][1] = min(INF, min(jc_c + cache[idx + 1][0], cache[idx + 1][1]));
        }
        cj_c = cj;
        jc_c = jc;
    }
    return *cache[0].iter().min().unwrap();
}

fn main() {
    let mut iu = InputUtils::default();
    let t: i32 = iu.input();
    let mut cj: i64;
    let mut jc: i64;
    let mut s: Vec<usize>;
    let mut cache: Vec<Vec<i64>> = vec![vec![0; 2]; 1005];
    for ti in 1..t + 1 {
        cj = iu.input();
        jc = iu.input();
        s = iu
            .input::<String>()
            .chars()
            .map(|c| match c {
                'C' => 0,
                'J' => 1,
                '?' => 2,
                _ => unreachable!("invalid char"),
            })
            .collect();
        println!("Case #{}: {}", ti, solve(s, cj, jc, &mut cache));
    }
}
