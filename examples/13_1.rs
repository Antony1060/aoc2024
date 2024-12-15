const INPUT: &str = include_str!("../input.txt");

#[derive(Debug)]
struct ProblemDef {
    a_move: (u64, u64),
    b_move: (u64, u64),
    prize: (u64, u64),
}

fn solve(problem: &ProblemDef) -> Option<u64> {
    let limit = 100;

    let mut result = u64::MAX;
    for i in 0..=limit {
        for j in 0..=limit {
            if i * problem.a_move.0 + j * problem.b_move.0 == problem.prize.0
                && i * problem.a_move.1 + j * problem.b_move.1 == problem.prize.1
            {
                result = result.min(i * 3 + j);
            }
        }
    }

    if result == u64::MAX {
        return None;
    }

    Some(result)
}

fn main() {
    let parts = INPUT
        .split("\n\n")
        .map(|it| {
            let [a, b, prize] = it.split("\n").collect::<Vec<_>>()[..] else {
                unreachable!()
            };

            let a_split = a.split("+").collect::<Vec<_>>();
            let b_split = b.split("+").collect::<Vec<_>>();
            let prize_split = prize.split("=").collect::<Vec<_>>();

            ProblemDef {
                a_move: (
                    a_split[1].split(",").next().unwrap().parse().unwrap(),
                    a_split[a_split.len() - 1].parse().unwrap(),
                ),
                b_move: (
                    b_split[1].split(",").next().unwrap().parse().unwrap(),
                    b_split[a_split.len() - 1].parse().unwrap(),
                ),
                prize: (
                    prize_split[1].split(",").next().unwrap().parse().unwrap(),
                    prize_split[a_split.len() - 1].parse().unwrap(),
                ),
            }
        })
        .collect::<Vec<_>>();

    let result = parts.iter().filter_map(solve).sum::<u64>();

    println!("{}", result);
}
