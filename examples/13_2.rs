use nalgebra::{Matrix, Matrix2, Matrix2x1};

const INPUT: &str = include_str!("../input.txt");

#[derive(Debug)]
struct ProblemDef {
    a_move: (u64, u64),
    b_move: (u64, u64),
    prize: (u64, u64),
}

fn solve(problem: &ProblemDef) -> Option<u64> {
    let move_mat = Matrix2::new(
        problem.a_move.0 as f64,
        problem.b_move.0 as f64,
        problem.a_move.1 as f64,
        problem.b_move.1 as f64,
    );

    let prize_mat = Matrix2x1::new(problem.prize.0 as f64, problem.prize.1 as f64);

    let move_mat_inv = &Matrix::try_inverse(move_mat).unwrap();

    let res = move_mat_inv * prize_mat;

    let [x, y] = [res.x.round() as u64, res.y.round() as u64];

    if x * problem.a_move.0 + y * problem.b_move.0 != problem.prize.0
        || x * problem.a_move.1 + y * problem.b_move.1 != problem.prize.1
    {
        return None;
    }

    Some(x * 3 + y)
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
                    prize_split[1]
                        .split(",")
                        .next()
                        .unwrap()
                        .parse::<u64>()
                        .unwrap()
                        + 10000000000000,
                    prize_split[a_split.len() - 1].parse::<u64>().unwrap() + 10000000000000,
                ),
            }
        })
        .collect::<Vec<_>>();

    let result = parts.iter().filter_map(solve).sum::<u64>();

    println!("{}", result);
}
