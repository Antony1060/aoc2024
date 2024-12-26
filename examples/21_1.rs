use std::collections::LinkedList;

const INPUT: &str = include_str!("../input.txt");

fn directional_on_directional(input: &str) -> String {
    let get_coords = |char: char| match char {
        'A' => (0, 2),
        '^' => (0, 1),
        '<' => (1, 0),
        'v' => (1, 1),
        '>' => (1, 2),
        _ => unreachable!(),
    };

    let mut curr = get_coords('A');
    let mut sequence = String::new();
    for input_char in input.chars() {
        let next = get_coords(input_char);

        if next.0 > curr.0 {
            for _ in 0..(next.0 - curr.0) {
                sequence.push('v');
            }
        }

        if next.1 > curr.1 {
            for _ in 0..(next.1 - curr.1) {
                sequence.push('>');
            }
        }

        if next.0 <= curr.0 {
            for _ in 0..(curr.0 - next.0) {
                sequence.push('^');
            }
        }

        if next.1 <= curr.1 {
            for _ in 0..(curr.1 - next.1) {
                sequence.push('<');
            }
        }
        sequence.push('A');

        curr = next;
    }

    sequence
}

fn robot_code(code: &str) -> String {
    const COORDS: [(u64, u64); 11] = [
        (3, 1),
        (2, 0),
        (2, 1),
        (2, 2),
        (1, 0),
        (1, 1),
        (1, 2),
        (0, 0),
        (0, 1),
        (0, 2),
        (3, 2),
    ];

    let mut result = String::new();
    let mut curr = COORDS[10];

    for c in code.chars() {
        let next = match c {
            'A' => COORDS[10],
            _ => COORDS[(c as u8 - b'0') as usize],
        };

        let all_paths = {
            let neighbours = |digit: &(u64, u64)| match *digit {
                it if it == COORDS[10] => vec![COORDS[0], COORDS[3]],
                it if it == COORDS[0] => vec![COORDS[2], COORDS[10]],
                it if it == COORDS[1] => vec![COORDS[2], COORDS[4]],
                it if it == COORDS[2] => vec![COORDS[0], COORDS[1], COORDS[3], COORDS[5]],
                it if it == COORDS[3] => vec![COORDS[2], COORDS[6], COORDS[10]],
                it if it == COORDS[4] => vec![COORDS[1], COORDS[5], COORDS[7]],
                it if it == COORDS[5] => vec![COORDS[2], COORDS[4], COORDS[6], COORDS[8]],
                it if it == COORDS[6] => vec![COORDS[3], COORDS[5], COORDS[9]],
                it if it == COORDS[7] => vec![COORDS[4], COORDS[8]],
                it if it == COORDS[8] => vec![COORDS[5], COORDS[7], COORDS[9]],
                it if it == COORDS[9] => vec![COORDS[6], COORDS[8]],
                _ => unreachable!(),
            };

            let mut paths = Vec::new();

            let mut queue: LinkedList<Vec<(u64, u64)>> = LinkedList::new();
            queue.push_back(vec![curr]);

            while let Some(path) = queue.pop_front() {
                let node = path.last().unwrap();

                if node == &next {
                    let char_path = {
                        let mut s = String::new();

                        for (i, n) in path[1..].iter().enumerate() {
                            let prev = path[i];
                            if prev.0 < n.0 {
                                s.push('v');
                            } else if prev.0 > n.0 {
                                s.push('^');
                            } else if prev.1 < n.1 {
                                s.push('>');
                            } else if prev.1 > n.1 {
                                s.push('<');
                            }
                        }

                        s.push('A');

                        s
                    };
                    paths.push(char_path);
                    continue;
                }

                for neighbour in neighbours(node) {
                    if path.contains(&neighbour) {
                        continue;
                    }

                    let mut cloned = path.clone();
                    cloned.push(neighbour);

                    queue.push_back(cloned);
                }
            }

            paths
        };

        let computed = all_paths
            .into_iter()
            .map(|it| {
                let mut res = it;

                for _ in 0..2 {
                    res = directional_on_directional(&res)
                }

                res
            })
            .min_by_key(|a| a.len())
            .unwrap();

        result.push_str(&computed);

        curr = next
    }

    result
}

fn main() {
    let codes = INPUT.split("\n").collect::<Vec<_>>();

    let result = codes
        .into_iter()
        .map(|it| {
            it[..it.len() - 1].parse::<u64>().unwrap() * dbg!(dbg!(robot_code(it)).len()) as u64
        })
        .sum::<u64>();

    println!("{}", result);
}
