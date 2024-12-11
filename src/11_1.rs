const INPUT: &str = include_str!("../input.txt");

fn main() {
    let mut stones = INPUT
        .split(' ')
        .map(|it| it.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    for i in 0..25 {
        dbg!(i);
        let mut new = Vec::with_capacity(stones.len() * 2);

        for stone in stones {
            // let stone = stones[idx];
            if stone == 0 {
                new.push(1);
                // idx += 1;
                continue;
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
                new.push(stone % pow);
                new.push(stone / pow);
                continue;
            }

            new.push(stone * 2024);
        }

        stones = new;
    }

    let result = stones.len();

    println!("{}", result);
}
