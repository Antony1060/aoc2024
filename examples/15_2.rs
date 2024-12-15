use std::collections::HashSet;

const INPUT: &str = include_str!("../input.txt");

fn find_move_boxes(
    matrix: &[Vec<char>],
    mut curr: (usize, usize),
    dir: (isize, isize),
) -> Option<HashSet<(usize, usize)>> {
    let mut ret = HashSet::new();

    if matrix[curr.0][curr.1] == '#' {
        if dir.1 != 0 && matrix[curr.0][(curr.1 as isize - dir.1) as usize] == '.' {
            return Some(ret);
        }
        return None;
    }

    if matrix[curr.0][curr.1] == ']' {
        if dir.1 != 0 {
            return Some(ret);
        }
        curr.1 -= 1;
    }

    if matrix[curr.0][curr.1] == '.' {
        return Some(ret);
    }

    ret.insert((curr.0, curr.1));

    match dir {
        (1, 0) => {
            ret.extend(find_move_boxes(matrix, (curr.0 + 1, curr.1), dir)?);
            ret.extend(find_move_boxes(matrix, (curr.0 + 1, curr.1 + 1), dir)?);
        }
        (-1, 0) => {
            ret.extend(find_move_boxes(matrix, (curr.0 - 1, curr.1), dir)?);
            ret.extend(find_move_boxes(matrix, (curr.0 - 1, curr.1 + 1), dir)?);
        }
        (0, 1) => {
            ret.extend(find_move_boxes(matrix, (curr.0, curr.1 + 2), dir)?);
        }
        (0, -1) => {
            ret.extend(find_move_boxes(matrix, (curr.0, curr.1 - 2), dir)?);
        }
        _ => unreachable!(),
    }

    Some(ret)
}

fn main() {
    let [board, moves] = INPUT.split("\n\n").collect::<Vec<_>>()[..] else {
        unreachable!()
    };

    let mut matrix = board
        .lines()
        .map(|it| {
            it.chars()
                .flat_map(|c| {
                    match c {
                        '.' => "..",
                        '#' => "##",
                        'O' => "[]",
                        '@' => "@.",
                        _ => unreachable!(),
                    }
                    .chars()
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let moves = moves.split("\n").fold(String::new(), |acc, it| acc + it);

    let mut curr = (0, 0);

    'outer: for (i, line) in matrix.iter().enumerate() {
        for (j, val) in line.iter().enumerate() {
            if *val == '@' {
                curr = (i, j);
                break 'outer;
            }
        }
    }

    for it in moves.chars() {
        let dir: (isize, isize) = match it {
            '>' => (0, 1),
            '<' => (0, -1),
            '^' => (-1, 0),
            'v' => (1, 0),
            _ => unreachable!(),
        };

        let mut lookup = (curr.0 as isize + dir.0, curr.1 as isize + dir.1);

        match matrix[lookup.0 as usize][lookup.1 as usize] {
            '.' => {
                matrix[curr.0][curr.1] = '.';
                curr = (
                    (curr.0 as isize + dir.0) as usize,
                    (curr.1 as isize + dir.1) as usize,
                );
                matrix[curr.0][curr.1] = '@';
            }
            it @ '[' | it @ ']' => 'm: {
                if it == ']' {
                    lookup.1 -= 1;
                }

                let moves = find_move_boxes(&matrix, (lookup.0 as usize, lookup.1 as usize), dir);

                let Some(moves) = moves else {
                    break 'm;
                };

                for (i, j) in moves.iter() {
                    matrix[*i][*j] = '.';
                    matrix[*i][*j + 1] = '.';
                }

                for (i, j) in moves {
                    let new_pos = (i as isize + dir.0, j as isize + dir.1);
                    matrix[new_pos.0 as usize][new_pos.1 as usize] = '[';
                    matrix[new_pos.0 as usize][new_pos.1 as usize + 1] = ']';
                }

                matrix[curr.0][curr.1] = '.';
                curr = (
                    (curr.0 as isize + dir.0) as usize,
                    (curr.1 as isize + dir.1) as usize,
                );
                matrix[curr.0][curr.1] = '@';
            }
            _ => {}
        }
    }

    for line in matrix.iter() {
        for val in line.iter() {
            print!("{}", val);
        }

        println!();
    }

    let result = matrix
        .iter()
        .enumerate()
        .flat_map(|(i, line)| {
            line.iter()
                .enumerate()
                .filter_map(move |(j, &val)| if val != '[' { None } else { Some(i * 100 + j) })
        })
        .sum::<usize>();

    println!("{}", result);
}
