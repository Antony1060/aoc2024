use std::collections::HashMap;

const INPUT: &str = include_str!("../input.txt");

fn solve(towels: &[String], rest: &str, curr: &str, memo: &mut HashMap<String, usize>) -> usize {
    if let Some(val) = memo.get(rest) {
        return *val;
    }

    if rest.is_empty() {
        memo.insert(rest.to_string(), 1);
        return 1;
    }

    let available = towels
        .iter()
        .filter(|&towel| rest.starts_with(towel) && rest.len() >= towel.len());

    let possible = available
        .map(|it| solve(towels, &rest[it.len()..], &format!("{}{}", curr, it), memo))
        .sum();

    memo.insert(rest.to_string(), possible);
    possible
}

fn main() {
    let [towels, patterns] = INPUT.split("\n\n").collect::<Vec<&str>>()[..] else {
        unreachable!()
    };

    let towels: Vec<String> = towels.split(", ").map(|it| it.to_string()).collect();
    let patterns: Vec<String> = patterns.split("\n").map(|it| it.to_string()).collect();

    let mut memo: HashMap<String, usize> = HashMap::new();

    let result: usize = patterns
        .iter()
        .map(|pattern| solve(&towels, pattern, "", &mut memo))
        .sum();

    println!("{}", result);
}
