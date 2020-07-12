use std::cmp::{min, Ordering};
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

struct Solver {
    n: usize,
    m: usize,
    cache: Vec<Vec<i32>>,
}

impl Solver {
    fn new(n: usize, m: usize) -> Self {
        Self {
            n: n,
            m: m,
            cache: vec![vec![0; m]; n],
        }
    }

    fn _cost(&self, comp: Ordering, z: usize, j: usize) -> i32 {
        match comp {
            Ordering::Equal => {
                if z != j {
                    return 1;
                }
            }
            Ordering::Less => {
                if z <= j {
                    return 1;
                }
            }
            Ordering::Greater => {
                if z >= j {
                    return 1;
                }
            }
        }
        return 0;
    }

    fn solve(&mut self, notes: &Vec<i32>) -> i32 {
        let k = notes.len();
        if k == 0 {
            return 0;
        }
        self.cache[k - 1].iter_mut().map(|x| *x = 0).count();
        for i in (0..k - 1).rev() {
            let comp = notes[i].cmp(&notes[i + 1]);
            for j in 0..self.m {
                self.cache[i][j] = std::i32::MAX;
                for z in 0..self.m {
                    self.cache[i][j] = min(
                        self.cache[i][j],
                        self._cost(comp, z, j) + self.cache[i + 1][z],
                    );
                }
            }
            // println!("{:?}", self.cache[i]);
        }
        return *self.cache[0].iter().min().unwrap();
    }
}

fn main() {
    let mut iu = InputUtils::default();
    let t: i32 = iu.input();
    let mut k: usize;
    let mut solver = Solver::new(10010, 4);
    for ti in 1..t + 1 {
        k = iu.input();
        let notes: Vec<i32> = iu.inputs(None);
        println!("Case #{}: {}", ti, solver.solve(&notes));
    }
}
