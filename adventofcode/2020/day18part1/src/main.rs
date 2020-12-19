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

fn eval_op(a: i64, b: i64, op: char) -> i64 {
    match op {
        '+' => a + b,
        '*' => a * b,
        _ => {
            unreachable!("Invalid op!");
        }
    }
}

fn eval(expr: &Vec<&str>, idx: usize) -> (i64, usize) {
    let mut idx = idx;
    let mut result: i64 = 0;
    let mut current_op: char = '+';
    while idx < expr.len() {
        match expr[idx] {
            "(" => {
                let (num, eidx) = eval(expr, idx + 1);
                result = eval_op(result, num, current_op);
                idx = eidx;
            }
            ")" => {
                break;
            }
            "+" => {
                current_op = '+';
            }
            "*" => {
                current_op = '*';
            }
            num_s => {
                let num: i64 = num_s.parse().expect(&format!(
                    "Failed to cast part at position {} in {:?} to i64",
                    idx, expr
                ));
                result = eval_op(result, num, current_op);
            }
        }
        idx += 1;
    }
    return (result, idx);
}

fn solve(lines: Box<dyn Iterator<Item = String>>) -> i64 {
    let mut answer = 0;
    for line in lines {
        let line = line
            .replace("(", "( ")
            .replace(")", " )")
            .trim()
            .to_string();
        let expr: Vec<&str> = line.split_whitespace().collect();
        let (result, _) = eval(&expr, 0);
        answer += result;
    }
    return answer;
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
        let test_input = r#"1 + (2 * 3) + (4 * (5 + 6))
2 * 3 + (4 * 5)
5 + (8 * 3 + 9 + 3 * 4 * 3)
5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))
((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"#;
        let it = test_input
            .split('\n')
            .into_iter()
            .map(|part| part.to_string());
        assert_eq!(solve(Box::new(it)), 26386);
    }
}
