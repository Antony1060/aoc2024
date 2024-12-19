use std::collections::HashMap;

const INPUT: &str = include_str!("../input.txt");

fn solve(towels: &[String], rest: &str, curr: &str, memo: &mut HashMap<String, bool>) -> bool {
    if let Some(val) = memo.get(rest) {
        return *val;
    }

    if rest.is_empty() {
        memo.insert(rest.to_string(), true);
        return true;
    }

    let available = towels
        .iter()
        .filter(|&towel| rest.starts_with(towel) && rest.len() >= towel.len());

    for it in available {
        if solve(towels, &rest[it.len()..], &format!("{}{}", curr, it), memo) {
            memo.insert(rest.to_string(), true);
            return true;
        }
    }

    memo.insert(rest.to_string(), false);
    false
}

fn main() {
    let [towels, patterns] = INPUT.split("\n\n").collect::<Vec<&str>>()[..] else {
        unreachable!()
    };

    let towels: Vec<String> = towels.split(", ").map(|it| it.to_string()).collect();
    let patterns: Vec<String> = patterns.split("\n").map(|it| it.to_string()).collect();

    let mut memo: HashMap<String, bool> = HashMap::new();

    let result = patterns
        .iter()
        .filter(|pattern| solve(&towels, pattern, "", &mut memo))
        .count();

    println!("{}", result);
}
