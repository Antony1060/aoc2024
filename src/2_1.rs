const INPUT: &str = include_str!("../input.txt");

fn main() {
    let levels = INPUT
        .split("\n")
        .map(|line| {
            line.split(" ")
                .map(|it| it.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let diffs = levels
        .into_iter()
        .map(|line| {
            line.windows(2)
                .map(|pair| pair[1] - pair[0])
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let correct = diffs
        .into_iter()
        .map(|line| {
            (line.iter().all(|val| *val >= 0) || line.iter().all(|val| *val <= 0))
                && line.iter().map(|it| it.abs()).all(|it| matches!(it, 1..=3))
        })
        .collect::<Vec<_>>();

    println!("{}", correct.into_iter().map(i32::from).sum::<i32>())
}
