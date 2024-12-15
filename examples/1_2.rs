use std::collections::HashMap;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    let mut l1: Vec<i32> = vec![];
    let mut l2_histogram: HashMap<i32, i32> = HashMap::new();

    for (n1, n2) in INPUT.split("\n").map(|l| l.split("   ")).map(|mut nums| {
        (
            nums.next().unwrap().parse::<i32>().unwrap(),
            nums.next().unwrap().parse::<i32>().unwrap(),
        )
    }) {
        l1.push(n1);
        if let Some(value) = l2_histogram.get(&n2) {
            l2_histogram.insert(n2, value + 1);
        } else {
            l2_histogram.insert(n2, 1);
        }
    }

    let res = l1
        .iter()
        .map(|it| it * l2_histogram.get(it).unwrap_or(&0))
        .sum::<i32>();

    println!("{}", res);
}
