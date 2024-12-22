use std::collections::HashMap;

const INPUT: &str = include_str!("../input.txt");

fn gen_secret(curr: u64, iter: u64, memo: &mut HashMap<(u64, u64), u64>) -> u64 {
    if let Some(val) = memo.get(&(curr, iter)) {
        return *val;
    }

    if iter == 0 {
        memo.insert((curr, iter), curr);
        return curr;
    }

    let mut result = curr;
    result ^= result * 64;
    result %= 16777216;
    result ^= result / 32;
    result %= 16777216;
    result ^= result * 2048;
    result %= 16777216;

    let res = gen_secret(result, iter - 1, memo);
    memo.insert((curr, iter), res);
    res
}

fn main() {
    let nums = INPUT
        .split("\n")
        .map(|it| it.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    let mut memo = HashMap::new();

    let mut sum = 0;
    for num in nums {
        let secret = gen_secret(num, 2000, &mut memo);
        // println!("{num}: {secret}");
        sum += secret;
    }

    println!("{}", sum);
}
