use itertools::Itertools;
use regex::Regex;
use std::{
    cell::RefCell,
    collections::VecDeque,
    io::{self, BufRead, Stdin},
    rc::Rc,
};

struct InputUtils {
    stream: Stdin,
}

impl Default for InputUtils {
    fn default() -> Self {
        Self {
            stream: io::stdin(),
        }
    }
}

impl Iterator for InputUtils {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        self.stream
            .lock()
            .lines()
            .next()
            .map(|line| line.unwrap().trim().to_string())
    }
}

enum Operand {
    Old,
    Scalar(i64),
}

enum Op {
    Add,
    Subtract,
    Multiply,
    Divide,
}

struct Operation {
    left: Operand,
    op: Op,
    right: Operand,
}

struct Monkey {
    inspect_counter: u64,
    items: VecDeque<i64>,
    operation: Operation,
    divisor: i64,
    true_to: usize,
    false_to: usize,
}

fn compute(worry: i64, operation: &Operation) -> i64 {
    let left = match operation.left {
        Operand::Old => worry,
        Operand::Scalar(x) => x,
    };
    let right = match operation.right {
        Operand::Old => worry,
        Operand::Scalar(x) => x,
    };
    let result = match operation.op {
        Op::Add => left + right,
        Op::Subtract => left - right,
        Op::Multiply => left * right,
        Op::Divide => left / right,
    };
    result / 3
}

fn simulate(monkeys: &[Rc<RefCell<Monkey>>], rounds: i64) -> u64 {
    for _ in 0..rounds {
        for monkey in monkeys {
            while !monkey.borrow().items.is_empty() {
                let mut monke = monkey.borrow_mut();
                let mut item = monke.items.pop_front().unwrap();
                monke.inspect_counter += 1;
                item = compute(item, &monke.operation);
                if item % monke.divisor == 0 {
                    monkeys[monke.true_to].borrow_mut().items.push_back(item);
                } else {
                    monkeys[monke.false_to].borrow_mut().items.push_back(item);
                }
            }
        }
    }
    let mut inspections: Vec<u64> = monkeys.iter().map(|m| m.borrow().inspect_counter).collect();
    inspections.sort();
    inspections.iter().rev().take(2).product()
}

fn parse_operand(value: &str) -> Operand {
    match value {
        "old" => Operand::Old,
        x => Operand::Scalar(x.parse::<i64>().expect("failed to cast integer operand")),
    }
}

fn parse_op(value: &str) -> Op {
    match value {
        "+" => Op::Add,
        "-" => Op::Subtract,
        "*" => Op::Multiply,
        "/" => Op::Divide,
        _ => unreachable!("Op should be one of +, -, *, /"),
    }
}

fn parse_monkey(text: String) -> Monkey {
    let pattern = Regex::new(r"Monkey \d+:\nStarting items: (?P<items>[\d, ]+)\nOperation: new = (?P<left>(?:old|\d+)) (?P<op>\+|\-|\*|\\) (?P<right>(?:old|\d+))\nTest: divisible by (?P<divisor>\d+)\nIf true: throw to monkey (?P<true_to>\d+)\nIf false: throw to monkey (?P<false_to>\d+)\n?").unwrap();
    let captured = pattern
        .captures(&text)
        .unwrap_or_else(|| panic!("Failed to parse group {:?}", text));
    let items: VecDeque<i64> = captured["items"]
        .split(", ")
        .map(|item| item.parse::<i64>().expect("failed to cast item"))
        .collect();
    let operation = Operation {
        left: parse_operand(&captured["left"]),
        op: parse_op(&captured["op"]),
        right: parse_operand(&captured["right"]),
    };
    let divisor = captured["divisor"]
        .parse::<i64>()
        .expect("failed to cast divisor");
    let true_to = captured["true_to"]
        .parse::<usize>()
        .expect("failed to cast test true pass to");
    let false_to = captured["false_to"]
        .parse::<usize>()
        .expect("failed to cast test true pass to");
    Monkey {
        inspect_counter: 0,
        items,
        operation,
        divisor,
        true_to,
        false_to,
    }
}

fn solve(lines: Box<dyn Iterator<Item = String>>) -> u64 {
    let mut monkeys: Vec<Rc<RefCell<Monkey>>> = vec![];
    for chunk in lines.chunks(7).into_iter() {
        let monke_text = chunk.into_iter().join("\n");
        monkeys.push(Rc::new(RefCell::new(parse_monkey(monke_text))));
    }
    simulate(&monkeys, 20)
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
        let test_input = r#"Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1"#;
        let it = test_input
            .split('\n')
            .into_iter()
            .map(|part| part.to_string());
        assert_eq!(solve(Box::new(it)), 10605);
    }
}
