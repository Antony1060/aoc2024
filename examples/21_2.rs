use std::collections::{HashMap, LinkedList};

const INPUT: &str = include_str!("../input.txt");

const ITER: usize = 25;

fn coord_of(c: char) -> (usize, usize) {
    match c {
        'A' => (0, 2),
        '^' => (0, 1),
        '<' => (1, 0),
        'v' => (1, 1),
        '>' => (1, 2),
        _ => unreachable!(),
    }
}

fn expand_a(source: &str, iter: usize, memo: &mut HashMap<(String, usize), usize>) -> usize {
    if let Some(val) = memo.get(&(source.to_string(), iter)) {
        return *val;
    }

    if !source.ends_with("A") {
        unreachable!("source must end with A");
    };

    if iter == 0 {
        memo.insert((source.to_string(), iter), source.len());
        return source.len();
    }

    let mut curr = coord_of('A');
    let mut res = Vec::new();
    for input_char in source.chars() {
        let mut sequence = String::new();

        let next = coord_of(input_char);

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
        res.push(sequence);

        curr = next;
    }

    let result = res
        .into_iter()
        .map(|it| expand_a(&it, iter - 1, memo))
        .sum();
    memo.insert((source.to_string(), iter), result);
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

    let mut result = 0;

    let mut memo: HashMap<(String, usize), usize> = HashMap::new();

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

        let computed = all_paths
            .into_iter()
            .map(|it| expand_a(&it, ITER, &mut memo))
            .min()
            .unwrap();

        result += computed;

        curr = next
    }

    result
}

fn main() {
    let codes = INPUT.split("\n").collect::<Vec<_>>();

    let result = codes
        .into_iter()
        .map(|it| it[..it.len() - 1].parse::<u64>().unwrap() * dbg!(robot_code(it)) as u64)
        .sum::<u64>();

    println!("{}", result);
}
