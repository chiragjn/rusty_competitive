use std::collections::{HashMap, HashSet};
use std::io::{self, BufRead, Stdin};
use std::iter::FromIterator;

struct InputUtils {
    stream: Stdin,
}

impl Default for InputUtils {
    fn default() -> Self {
        return Self {
            stream: io::stdin(),
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

fn is_subset(a: &str, b: &str) -> bool {
    let aset: HashSet<char> = HashSet::from_iter(a.chars());
    let bset: HashSet<char> = HashSet::from_iter(b.chars());
    return aset.is_subset(&bset);
}

fn decode(observations: &Vec<&str>, to_decode: &Vec<&str>) -> u64 {
    // 1, 4, 7, 8, 9, 0, 6, 5, 3, 2
    let mut digit2combo: HashMap<u64, &str> = HashMap::new();
    let mut len_six: HashSet<&str> = HashSet::new();
    let mut len_five: HashSet<&str> = HashSet::new();
    for &combo in observations.iter() {
        match combo.len() {
            2 => {
                digit2combo.insert(1, combo);
            }
            4 => {
                digit2combo.insert(4, combo);
            }
            3 => {
                digit2combo.insert(7, combo);
            }
            7 => {
                digit2combo.insert(8, combo);
            }
            6 => {
                len_six.insert(combo);
            }
            5 => {
                len_five.insert(combo);
            }
            _ => {
                unreachable!(format!("Got an invalid combo {}", combo));
            }
        }
    }
    let nine_combo = len_six
        .iter()
        .filter(|&&b| is_subset(digit2combo[&4], b))
        .map(|&c| c)
        .next()
        .unwrap();
    digit2combo.insert(9, nine_combo);
    len_six.remove(nine_combo);

    let zero_combo = len_six
        .iter()
        .filter(|&&b| is_subset(digit2combo[&7], b))
        .map(|&c| c)
        .next()
        .unwrap();
    digit2combo.insert(0, zero_combo);
    len_six.remove(zero_combo);

    digit2combo.insert(6, len_six.iter().next().unwrap());

    let five_combo = len_five
        .iter()
        .filter(|&&a| is_subset(a, digit2combo[&6]))
        .map(|&c| c)
        .next()
        .unwrap();
    digit2combo.insert(5, five_combo);
    len_five.remove(five_combo);

    let three_combo = len_five
        .iter()
        .filter(|&&b| is_subset(digit2combo[&7], b))
        .map(|&c| c)
        .next()
        .unwrap();
    digit2combo.insert(3, three_combo);
    len_five.remove(three_combo);

    digit2combo.insert(2, len_five.iter().next().unwrap());

    let mut combo2digit: HashMap<String, u64> = HashMap::new();
    for (&digit, &combo) in digit2combo.iter() {
        let mut combo_chars: Vec<char> = combo.chars().collect();
        combo_chars.sort();
        let sorted_combo: String = combo_chars.iter().collect();
        combo2digit.insert(sorted_combo, digit);
    }
    let mut decoded = 0;
    for &combo in to_decode.iter() {
        let mut combo_chars: Vec<char> = combo.chars().collect();
        combo_chars.sort();
        let sorted_combo: String = combo_chars.iter().collect();
        decoded = decoded * 10 + combo2digit[&sorted_combo];
    }
    return decoded;
}

fn solve(lines: Box<dyn Iterator<Item = String>>) -> u64 {
    let mut answer: u64 = 0;
    for line in lines {
        let mut parts = line.split(" | ").into_iter();
        let observations: Vec<&str> = parts
            .next()
            .expect("Failed to get first part")
            .trim()
            .split_whitespace()
            .collect();
        let to_decode: Vec<&str> = parts
            .next()
            .expect("Failed to get second part")
            .trim()
            .split_whitespace()
            .collect();
        answer += decode(&observations, &to_decode);
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
        let test_input = r#"be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce"#;
        let it = test_input
            .split('\n')
            .into_iter()
            .map(|part| part.to_string());
        assert_eq!(solve(Box::new(it)), 61229);
    }
}
