use regex::Regex;
use std::collections::HashMap;
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

fn is_invalid(field2ranges: &HashMap<String, [(i64, i64); 2]>, values: &Vec<i64>) -> bool {
    'outer: for &value in values {
        for &ranges in field2ranges.values() {
            for &range in ranges.iter() {
                if value >= range.0 && value <= range.1 {
                    continue 'outer;
                }
            }
        }
        return true;
    }
    return false;
}

fn map_possible_fields<'a>(
    field2ranges: &'a HashMap<String, [(i64, i64); 2]>,
    valid_values: &Vec<Vec<i64>>,
) -> HashMap<usize, Vec<&'a String>> {
    let mut idx2fields: HashMap<usize, Vec<&'a String>> = HashMap::new();
    if !valid_values.is_empty() {
        for j in 0..valid_values[0].len() {
            'to_next_field: for (name, ranges) in field2ranges.iter() {
                for i in 0..valid_values.len() {
                    let v = valid_values[i][j];
                    if !((v >= ranges[0].0 && v <= ranges[0].1)
                        || (v >= ranges[1].0 && v <= ranges[1].1))
                    {
                        continue 'to_next_field;
                    }
                }
                idx2fields.entry(j).or_default().push(name);
            }
        }
    }
    return idx2fields;
}

fn into_alignment<'a>(
    mut idx2fields: HashMap<usize, Vec<&'a String>>,
    n: usize,
) -> Result<HashMap<&'a String, usize>, String> {
    let mut alignment: HashMap<&'a String, usize> = HashMap::new();
    let mut state_changed;
    let mut names_to_remove: Vec<&'a String> = vec![];
    let mut idxs_to_remove: Vec<usize> = vec![];
    while alignment.len() != n {
        state_changed = false;
        for (idx, names) in idx2fields.iter() {
            if names.len() == 1 {
                alignment.insert(names[0], *idx);
                names_to_remove.push(names[0]);
            }
        }
        if !names_to_remove.is_empty() {
            state_changed = true;
            for name_to_remove in names_to_remove.drain(..) {
                for (idx, names) in idx2fields.iter_mut() {
                    names.retain(|&name| name != name_to_remove);
                    if names.is_empty() {
                        idxs_to_remove.push(*idx);
                    }
                }
            }
            for idx in idxs_to_remove.drain(..) {
                idx2fields.remove(&idx);
            }
        }
        if !state_changed {
            return Err(format!("Failed to create a alignment, no single pair left in mappings to converge further. Last state: {:?}", idx2fields));
        }
    }
    return Ok(alignment);
}

fn solve(mut lines: Box<dyn Iterator<Item = String>>) -> i64 {
    let mut answer = 1;
    let mut field2ranges: HashMap<String, [(i64, i64); 2]> = HashMap::new();
    let pattern = Regex::new(
        r"(?P<field>[a-z\s]+): (?P<low1>\d+)-(?P<high1>\d+) or (?P<low2>\d+)-(?P<high2>\d+)",
    )
    .unwrap();
    loop {
        if let Some(sline) = lines.next() {
            if sline == "" {
                break;
            }
            let captured = pattern
                .captures(&sline)
                .expect(&format!("Failed to parse line {:?}", sline));
            let low1: i64 = captured["low1"].parse().expect("Failed to cast to int");
            let high1: i64 = captured["high1"].parse().expect("Failed to cast to int");
            let low2: i64 = captured["low2"].parse().expect("Failed to cast to int");
            let high2: i64 = captured["high2"].parse().expect("Failed to cast to int");
            field2ranges.insert(
                captured["field"].to_string(),
                [(low1, high1), (low2, high2)],
            );
        } else {
            unreachable!("Input terminated after first empty line!")
        }
    }
    lines.next();
    let mut valid_values: Vec<Vec<i64>> = vec![];
    let my_values: Vec<i64> = lines
        .next()
        .expect("Failed to get values for 'your ticket'")
        .split(",")
        .map(|part| {
            part.parse()
                .expect(&format!("failed to parse {} to i64", part))
        })
        .collect();
    valid_values.push(my_values);
    lines.next();
    lines.next();
    for line in lines {
        let values: Vec<i64> = line
            .split(",")
            .map(|part| {
                part.parse()
                    .expect(&format!("failed to parse {} to i64", part))
            })
            .collect();
        if !is_invalid(&field2ranges, &values) {
            valid_values.push(values);
        }
    }
    let alignment = into_alignment(
        map_possible_fields(&field2ranges, &valid_values),
        field2ranges.len(),
    )
    .unwrap();
    for (name, &idx) in alignment.iter() {
        if name.contains("departure") {
            answer *= valid_values[0][idx];
        }
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
        let test_input = r#"class: 0-1 or 4-19
departure row: 0-5 or 8-19
departure seat: 0-13 or 16-19

your ticket:
11,12,13

nearby tickets:
3,9,18
15,1,5
5,14,9"#;
        let it = test_input
            .split('\n')
            .into_iter()
            .map(|part| part.to_string());
        assert_eq!(solve(Box::new(it)), 143);
    }
}
