use std::collections::{HashMap, HashSet};
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

fn resolve_mappings(
    possible_mappings: &mut HashMap<String, HashSet<String>>,
) -> Result<HashMap<String, String>, String> {
    // Whole bunch on cloning here! There's got to be a better way!
    let mut resolved: HashMap<String, String> = HashMap::new();
    while !possible_mappings.is_empty() {
        let mut ingredients_to_remove: Vec<String> = vec![];
        for (allergent, ingredients) in possible_mappings.iter() {
            if ingredients.len() == 1 {
                let ingredient = ingredients.iter().next().unwrap().clone();
                resolved.insert(ingredient.clone(), allergent.clone());
                ingredients_to_remove.push(ingredient);
            }
        }
        if ingredients_to_remove.is_empty() {
            return Err(format!(
                "Failed to resolve mappings. Last state -> {:?}",
                possible_mappings
            ));
        }
        let mut resolved_allergents: Vec<String> = vec![];
        for (allergent, ingredients) in possible_mappings.iter_mut() {
            for ingredient in ingredients_to_remove.iter() {
                ingredients.remove(ingredient);
            }
            if ingredients.len() == 0 {
                resolved_allergents.push(allergent.clone());
            }
        }
        for allergent in resolved_allergents {
            possible_mappings.remove(&allergent);
        }
    }
    return Ok(resolved);
}

fn solve(lines: Box<dyn Iterator<Item = String>>) -> i64 {
    let mut answer: i64 = 0;
    let mut possible_mappings: HashMap<String, HashSet<String>> = HashMap::new();
    let mut ingredient_counter: HashMap<String, usize> = HashMap::new();
    for line in lines {
        let mut parts = line.split(" (contains ");
        let ingredients_s = parts
            .next()
            .expect(&format!("Failed to read ingredients from line {}", line));
        let mut ingredients: HashSet<String> = HashSet::new();
        for ingredient_s in ingredients_s.split_whitespace() {
            let ingredient = ingredient_s.to_string();
            *ingredient_counter.entry(ingredient.clone()).or_default() += 1;
            ingredients.insert(ingredient);
        }
        if let Some(allergents_s) = parts.next() {
            for allergent in allergents_s.trim_end_matches(")").split(", ") {
                let allergent = allergent.to_string();
                possible_mappings
                    .entry(allergent)
                    .and_modify(|i| *i = i.intersection(&ingredients).cloned().collect())
                    .or_insert(ingredients.clone());
            }
        }
    }
    let ingredient2allergent = resolve_mappings(&mut possible_mappings).unwrap();
    for (ingredient, &count) in ingredient_counter.iter() {
        if !ingredient2allergent.contains_key(ingredient) {
            answer += count as i64;
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
        let test_input = r#"mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
trh fvjkl sbzzf mxmxvkd (contains dairy)
sqjhc fvjkl (contains soy)
sqjhc mxmxvkd sbzzf (contains fish)"#;
        let it = test_input
            .split('\n')
            .into_iter()
            .map(|part| part.to_string());
        assert_eq!(solve(Box::new(it)), 5);
    }
}
