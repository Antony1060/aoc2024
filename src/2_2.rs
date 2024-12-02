const INPUT: &str = include_str!("../input.txt");

// lol, this is a very bad solution
fn main() {
    let levels = INPUT
        .split("\n")
        .map(|line| {
            line.split(" ")
                .map(|it| it.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let variants = levels
        .into_iter()
        .map(|line| {
            // I am so smart
            let mut res: Vec<Vec<i32>> = vec![];

            res.push(line.clone());

            for i in 0..line.len() {
                let mut copy = line.clone();
                copy.remove(i);

                res.push(copy);
            }

            res
        })
        .collect::<Vec<_>>();

    let diffs = variants
        .into_iter()
        .map(|set| {
            set.into_iter()
                .map(|line| {
                    line.windows(2)
                        .map(|pair| pair[1] - pair[0])
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let correct = diffs
        .into_iter()
        .map(|set| {
            set.into_iter().map(|line| {
                (line.iter().all(|val| *val >= 0) || line.iter().all(|val| *val <= 0))
                    && line.iter().map(|it| it.abs()).all(|it| matches!(it, 1..=3))
            })
        })
        .map(|mut set| set.any(|it| it))
        .collect::<Vec<_>>();

    println!("{}", correct.into_iter().map(i32::from).sum::<i32>())
}
