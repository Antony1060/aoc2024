use regex::Regex;

const INPUT: &str = include_str!("../input.txt");

fn do_line(line: &str) -> i32 {
    let re = Regex::new("(don't)(\\(\\))|(do)(\\(\\))|mul\\((\\d{1,3}),(\\d{1,3})\\)");

    let mut sum = 0;

    let mut enabled = true;

    for (_, [n1, n2]) in re.unwrap().captures_iter(line).map(|c| c.extract()) {
        if n1 == "don't" {
            enabled = false;
            continue;
        }

        if n1 == "do" {
            enabled = true;
            continue;
        }

        if !enabled {
            continue;
        }

        sum += n1.parse::<i32>().unwrap() * n2.parse::<i32>().unwrap();
    }

    sum
}

fn main() {
    let sum: i32 = do_line(
        &INPUT
            .split("\n")
            .fold(String::new(), |acc, curr| acc + curr),
    );

    println!("{}", sum);
}
