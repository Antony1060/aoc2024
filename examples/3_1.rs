use regex::Regex;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    let re = Regex::new("mul\\((\\d{1,3}),(\\d{1,3})\\)");

    let mut sum = 0;

    for (_, [n1, n2]) in re
        .unwrap()
        .captures_iter(
            &INPUT
                .split("\n")
                .fold(String::new(), |acc, curr| acc + curr),
        )
        .map(|c| c.extract())
    {
        sum += n1.parse::<i32>().unwrap() * n2.parse::<i32>().unwrap();
    }

    println!("{}", sum);
}
