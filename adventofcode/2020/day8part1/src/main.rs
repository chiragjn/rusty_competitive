use std::{
    io::{self, BufRead, Stdin},
    str::FromStr,
};

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

#[derive(Eq, PartialEq)]
enum Instruction {
    NOP,
    JMP(i64),
    ACC(i64),
}

impl FromStr for Instruction {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_whitespace();
        let instruction = parts
            .next()
            .expect(&format!("Failed to get instruction part from {}", s));
        let offset: i64 = parts
            .next()
            .expect(&format!("Failed to get offset part from {}", s))
            .parse()
            .expect(&format!("Failed to cast offset to i32 for {}", s));
        match instruction {
            "nop" => Ok(Instruction::NOP),
            "jmp" => Ok(Instruction::JMP(offset)),
            "acc" => Ok(Instruction::ACC(offset)),
            _ => Err(format!("Failed to convert {} to Instruction", s)),
        }
    }
}

fn solve(lines: Box<dyn Iterator<Item = String>>) -> i64 {
    let instructions: Vec<Instruction> = lines
        .map(|line| Instruction::from_str(&line).unwrap())
        .collect();
    let mut executed = vec![false; instructions.len()];
    let mut acc: i64 = 0;
    let mut ptr: i64 = 0;
    while !executed[ptr as usize] {
        executed[ptr as usize] = true;
        match instructions[ptr as usize] {
            Instruction::JMP(o) => {
                ptr += o;
            }
            Instruction::ACC(d) => {
                acc += d;
                ptr += 1;
            }
            _ => {
                ptr += 1;
            }
        }
        if ptr < 0 {
            panic!("ptr dropped below zero!");
        }
    }
    return acc;
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
        let test_input = r#"nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6"#;
        let it = test_input
            .split('\n')
            .into_iter()
            .map(|part| part.to_string());
        assert_eq!(solve(Box::new(it)), 5);
    }
}
