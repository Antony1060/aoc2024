use std::collections::HashMap;

const INPUT: &str = include_str!("../input.txt");

fn process_stone(stone: u64, iter_left: u64, memo: &mut HashMap<(u64, u64), u64>) -> u64 {
    if let Some(val) = memo.get(&(stone, iter_left)) {
        return *val;
    }

    if iter_left == 0 {
        memo.insert((stone, iter_left), 1);
        return 1;
    }

    if stone == 0 {
        let result = process_stone(1, iter_left - 1, memo);
        memo.insert((stone, iter_left), result);
        return result;
    }

    let len = {
        let mut sum = 0;
        let mut copy = stone;

        while copy != 0 {
            sum += 1;
            copy /= 10;
        }

        sum
    };

    if len % 2 == 0 {
        let pow = 10u64.pow(len / 2);
        let result = process_stone(stone % pow, iter_left - 1, memo)
            + process_stone(stone / pow, iter_left - 1, memo);
        memo.insert((stone, iter_left), result);
        return result;
    }

    let result = process_stone(stone * 2024, iter_left - 1, memo);
    memo.insert((stone, iter_left), result);
    result
}

fn main() {
    let stones = INPUT
        .split(' ')
        .map(|it| it.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    let result = stones
        .into_iter()
        .map(|it| process_stone(it, 75, &mut HashMap::new()))
        .sum::<u64>();

    println!("{}", result);
}
