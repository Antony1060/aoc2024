const INPUT: &str = include_str!("../input.txt");

const LOCK_HEIGHT: i64 = 7;

fn parse_lock_or_key(grid: &str, key_char: char) -> Vec<i64> {
    let lines = grid.lines().collect::<Vec<_>>();
    let mut res = Vec::with_capacity(lines[0].len());
    for _ in 0..lines[0].len() {
        res.push(0);
    }

    for line in lines {
        for (idx, c) in line.chars().enumerate() {
            if c == key_char {
                res[idx] += 1;
            }
        }
    }

    res
}

fn main() {
    let grids = INPUT.split("\n\n").collect::<Vec<_>>();

    let locks = grids
        .iter()
        .filter(|it| it.starts_with("#"))
        .map(|&it| parse_lock_or_key(it, '#'))
        .collect::<Vec<_>>();

    let keys = grids
        .iter()
        .filter(|it| it.starts_with("."))
        .map(|&it| parse_lock_or_key(it, '.'))
        .map(|key| {
            key.into_iter()
                .map(|it| LOCK_HEIGHT - it)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut res = 0;

    for lock in locks {
        for key in keys.iter() {
            let good = lock.iter().zip(key).all(|(l, k)| l + k <= LOCK_HEIGHT);

            if good {
                res += 1;
            }
        }
    }

    println!("{}", res);
}
