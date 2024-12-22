use std::cmp::max;
use std::collections::{HashMap, HashSet, VecDeque};

const INPUT: &str = include_str!("../input.txt");

fn gen_secret(
    curr: u64,
    iter: u64,
    seq: &mut VecDeque<i8>,
    ret: &mut HashMap<(i8, i8, i8, i8), u8>,
) {
    let price_curr = (curr % 10) as u8;

    if seq.len() == 4 {
        let curr_seq = (seq[0], seq[1], seq[2], seq[3]);
        if !ret.contains_key(&curr_seq) {
            ret.insert((seq[0], seq[1], seq[2], seq[3]), price_curr);
        }
    }

    if iter == 0 {
        return;
    }

    let mut result = curr;
    result ^= result * 64;
    result %= 16777216;
    result ^= result / 32;
    result %= 16777216;
    result ^= result * 2048;
    result %= 16777216;

    let price_result = (result % 10) as i8;

    let diff = price_result - price_curr as i8;
    seq.push_back(diff);

    if seq.len() > 4 {
        seq.pop_front();
    }

    gen_secret(result, iter - 1, seq, ret);
}

fn main() {
    let nums = INPUT
        .split("\n")
        .map(|it| it.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    let mut all_sequences: HashSet<(i8, i8, i8, i8)> = HashSet::new();

    let mut res_maps = Vec::new();

    for num in nums {
        let mut res = HashMap::new();
        gen_secret(num, 2000, &mut VecDeque::new(), &mut res);
        all_sequences.extend(res.keys());
        res_maps.push(res);
    }

    println!("gen done: {}", all_sequences.len());

    let mut result: u64 = 0;

    for sequence in all_sequences {
        let mut bananas: u64 = 0;

        for res_map in &res_maps {
            if let Some(val) = res_map.get(&sequence) {
                bananas += *val as u64;
            }
        }

        result = max(result, bananas);
    }

    println!("{:?}", result);
}
