const INPUT: &str = include_str!("../input.txt");

const EMPTY: u64 = u64::MAX;

fn main() {
    let mut blocks: Vec<u64> = INPUT
        .chars()
        .map(|char| (char as u8 - b'0') as usize)
        .enumerate()
        .flat_map(|(idx, num)| {
            if idx % 2 == 1 {
                vec![EMPTY].into_iter().cycle().take(num)
            } else {
                vec![(idx / 2) as u64].into_iter().cycle().take(num)
            }
        })
        .collect();

    let mut checksum: u64 = 0;

    let mut j: isize = (blocks.len() - 1) as isize;

    while j >= 0 {
        if blocks[j as usize] == EMPTY {
            j -= 1;
            continue;
        }

        let mut size = 0;
        let key = blocks[j as usize];
        while j >= 0 && blocks[j as usize] == key {
            size += 1;
            j -= 1;
        }

        let j_usize = (j + 1) as usize;

        let mut i: usize = 0;
        let mut empty_size = 0;
        while empty_size < size && i < j_usize {
            empty_size = 0;

            while i < j_usize && blocks[i] != EMPTY {
                i += 1;
            }

            while i < j_usize && blocks[i] == EMPTY {
                empty_size += 1;
                i += 1;
            }
        }

        if empty_size < size {
            continue;
        }

        i -= empty_size;

        for k in 0..size {
            blocks.swap(i + k, j_usize + k);
        }
    }

    for (idx, val) in blocks.iter().enumerate() {
        if *val == EMPTY {
            continue;
        }
        checksum += idx as u64 * val;
    }

    println!("{}", checksum);
}
