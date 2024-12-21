use std::collections::{HashMap, LinkedList};

const INPUT: &str = include_str!("../input.txt");

const ITER: usize = 2;

fn directional_on_directional_cnt(
    came_from: char,
    char: char,
    iter: usize,
    memo: &mut HashMap<(char, char, usize), usize>,
) -> usize {
    if let Some(val) = memo.get(&(came_from, char, iter)) {
        return *val;
    }

    if iter == 0 {
        memo.insert((came_from, char, iter), 1);
        return 1;
    }

    let get_coords = |char: char| match char {
        'A' => (0, 2),
        '^' => (0, 1),
        '<' => (1, 0),
        'v' => (1, 1),
        '>' => (1, 2),
        _ => unreachable!(),
    };

    let curr = get_coords(came_from);
    let mut sequence = String::new();
    sequence.push(char);

    let next = get_coords(char);

    if next.1 > curr.1 {
        for _ in 0..(next.1 - curr.1) {
            sequence.push('>');
        }
    }

    if next.0 > curr.0 {
        for _ in 0..(next.0 - curr.0) {
            sequence.push('v');
        }
    }

    if next.1 <= curr.1 {
        for _ in 0..(curr.1 - next.1) {
            sequence.push('<');
        }
    }

    if next.0 <= curr.0 {
        for _ in 0..(curr.0 - next.0) {
            sequence.push('^');
        }
    }
    sequence.push('A');

    let result = sequence
        .chars()
        .collect::<Vec<_>>()
        .windows(2)
        .map(|win| directional_on_directional_cnt(win[0], win[1], iter - 1, memo))
        .sum();

    memo.insert((came_from, char, iter), result);
    result
}

fn robot_code(code: &str) -> usize {
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

    let mut curr = COORDS[10];

    let mut all_all_paths = Vec::new();
    for c in code.chars() {
        let next = match c {
            'A' => COORDS[10],
            _ => COORDS[(c as u8 - b'0') as usize],
        };

        // yes, this is a bfs, stfu
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

                // println!("neighbours {c}: {:?}", neighbours(node));
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

        all_all_paths.push(all_paths);

        curr = next
    }

    let [n1, n2, n3, n4] = &all_all_paths[..] else {
        unreachable!()
    };

    let n34 = n3
        .iter()
        .flat_map(|n3| {
            n4.iter().map(move |n4| {
                let mut s = n3.clone();
                s.push_str(n4);
                s
            })
        })
        .collect::<Vec<_>>();

    let n234 = n2
        .iter()
        .flat_map(|n2| {
            n34.iter().map(move |n34| {
                let mut s = n2.clone();
                s.push_str(n34);
                s
            })
        })
        .collect::<Vec<_>>();

    let all_combinations = n1
        .iter()
        .flat_map(|n1| {
            n234.iter().map(move |n234| {
                let mut s = n1.clone();
                s.push_str(n234);
                s
            })
        })
        .collect::<Vec<_>>();

    let mut memo: HashMap<(char, char, usize), usize> = HashMap::new();

    let computed = all_combinations
        .into_iter()
        .map(|it| {
            ("A".to_string() + &it)
                .chars()
                .collect::<Vec<_>>()
                .windows(2)
                .map(|win| directional_on_directional_cnt(win[0], win[1], ITER, &mut memo))
                .sum::<usize>()
        })
        .collect::<Vec<_>>();

    // dbg!(&computed);

    *computed.iter().min().unwrap()
}

fn main() {
    let codes = INPUT.split("\n").collect::<Vec<_>>();

    let result = codes
        .into_iter()
        .map(|it| it[..it.len() - 1].parse::<u64>().unwrap() * dbg!(robot_code(it)) as u64)
        .sum::<u64>();

    println!("{}", result);
}
