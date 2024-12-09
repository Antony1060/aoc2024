const INPUT: &str = include_str!("../input.txt");

const EMPTY: u64 = u64::MAX;

fn main() {
    let blocks: Vec<u64> = INPUT
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
    let mut i: usize = 0;
    let mut j: usize = blocks.len() - 1;

    while i < j {
        let i_val = blocks[i];
        let mut j_val = blocks[j];

        if i_val != EMPTY {
            checksum += i as u64 * i_val;
            i += 1;
            continue;
        }

        while j_val == EMPTY && i < j {
            j -= 1;
            j_val = blocks[j];
        }

        if j_val == EMPTY {
            break;
        }

        checksum += i as u64 * j_val;
        j -= 1;
        i += 1;
    }

    println!("{}", checksum);
}
