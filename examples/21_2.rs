use std::collections::{HashMap, LinkedList};

const INPUT: &str = include_str!("../input.txt");

const ITER: usize = 25;

fn all_paths_bfs<N>(neighbours: N, start: (u64, u64), end: (u64, u64)) -> Vec<String>
where
    N: Fn(&(u64, u64)) -> Vec<(u64, u64)>,
{
    let mut paths = Vec::new();

    let mut queue: LinkedList<Vec<(u64, u64)>> = LinkedList::new();
    queue.push_back(vec![start]);

    while let Some(path) = queue.pop_front() {
        let node = path.last().unwrap();

        if node == &end {
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
}

// NUM_COORDS[digit] is coordinate of digit
//  except for A, that's NUM_COORDS[10]
const NUM_COORDS: [(u64, u64); 11] = [
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

fn dir_coord_of(c: char) -> (u64, u64) {
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
        let result = source.len();
        memo.insert((source.to_string(), iter), result);
        return source.len();
    }

    let neighbours = |symbol: &(u64, u64)| match *symbol {
        it if it == dir_coord_of('A') => vec![dir_coord_of('>'), dir_coord_of('^')],
        it if it == dir_coord_of('^') => vec![dir_coord_of('A'), dir_coord_of('v')],
        it if it == dir_coord_of('>') => vec![dir_coord_of('A'), dir_coord_of('v')],
        it if it == dir_coord_of('<') => vec![dir_coord_of('v')],
        it if it == dir_coord_of('v') => {
            vec![dir_coord_of('>'), dir_coord_of('^'), dir_coord_of('<')]
        }
        _ => unreachable!(),
    };

    let mut curr = dir_coord_of('A');
    let mut paths: Vec<Vec<String>> = Vec::new();
    for input_char in source.chars() {
        let next = dir_coord_of(input_char);

        let all_paths = all_paths_bfs(neighbours, curr, next);

        paths.push(all_paths);

        curr = next;
    }

    let result = paths
        .into_iter()
        .map(|all_paths| {
            // find the smallest path
            all_paths
                .iter()
                .map(|it| expand_a(it, iter - 1, memo))
                .min()
                .unwrap()
        })
        .sum();
    memo.insert((source.to_string(), iter), result);
    result
}

fn robot_code(code: &str) -> usize {
    let mut curr = NUM_COORDS[10];

    let mut result = 0;

    let mut memo: HashMap<(String, usize), usize> = HashMap::new();

    for it in code.chars() {
        let next = match it {
            'A' => NUM_COORDS[10],
            _ => NUM_COORDS[(it as u8 - b'0') as usize],
        };

        let neighbours = |digit: &(u64, u64)| match *digit {
            it if it == NUM_COORDS[10] => vec![NUM_COORDS[0], NUM_COORDS[3]],
            it if it == NUM_COORDS[0] => vec![NUM_COORDS[2], NUM_COORDS[10]],
            it if it == NUM_COORDS[1] => vec![NUM_COORDS[2], NUM_COORDS[4]],
            it if it == NUM_COORDS[2] => {
                vec![NUM_COORDS[0], NUM_COORDS[1], NUM_COORDS[3], NUM_COORDS[5]]
            }
            it if it == NUM_COORDS[3] => vec![NUM_COORDS[2], NUM_COORDS[6], NUM_COORDS[10]],
            it if it == NUM_COORDS[4] => vec![NUM_COORDS[1], NUM_COORDS[5], NUM_COORDS[7]],
            it if it == NUM_COORDS[5] => {
                vec![NUM_COORDS[2], NUM_COORDS[4], NUM_COORDS[6], NUM_COORDS[8]]
            }
            it if it == NUM_COORDS[6] => vec![NUM_COORDS[3], NUM_COORDS[5], NUM_COORDS[9]],
            it if it == NUM_COORDS[7] => vec![NUM_COORDS[4], NUM_COORDS[8]],
            it if it == NUM_COORDS[8] => vec![NUM_COORDS[5], NUM_COORDS[7], NUM_COORDS[9]],
            it if it == NUM_COORDS[9] => vec![NUM_COORDS[6], NUM_COORDS[8]],
            _ => unreachable!(),
        };
        let all_paths = all_paths_bfs(neighbours, curr, next);

        result += all_paths
            .into_iter()
            .map(|it| expand_a(&it, ITER, &mut memo))
            .min()
            .unwrap();

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
