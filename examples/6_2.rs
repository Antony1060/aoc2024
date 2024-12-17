const INPUT: &str = include_str!("../input.txt");

fn dir_from_char(c: char) -> (isize, isize) {
    match c {
        '^' => (-1, 0),
        '>' => (0, 1),
        '<' => (0, -1),
        'v' => (1, 0),
        _ => unreachable!(),
    }
}

fn walk(matrix: &mut [Vec<char>], (mut x, mut y): (usize, usize)) -> Option<usize> {
    let mut result = 0;

    let mut iter: usize = 0;

    'outer: loop {
        iter += 1;

        // sometimes, my check if we're in a loop will not work, so let's just stop after 20k iterations
        if iter >= 20000 {
            return None;
        }

        if x == usize::MAX || y == usize::MAX {
            result += 0;
            break 'outer;
        }

        let dir_char_original = matrix[x][y];
        let mut dir_char = matrix[x][y];

        let (mut fx, mut fy);

        let mut is_turn = false;

        loop {
            let (dx, dy) = dir_from_char(dir_char);

            (fx, fy) = (x as isize + dx, y as isize + dy);

            if fx < 0 || fx >= matrix.len() as isize || fy < 0 || fy >= matrix[0].len() as isize {
                result += 1;
                break 'outer;
            }

            if matrix[fx as usize][fy as usize] != '#' {
                break;
            }

            is_turn = true;

            dir_char = match dir_char {
                '^' => '>',
                '>' => 'v',
                '<' => '^',
                'v' => '<',
                _ => unreachable!(),
            };

            if dir_char == matrix[x][y] {
                result += 0;
                break 'outer;
            }
        }

        let (fx, fy) = (fx as usize, fy as usize);

        let add = match matrix[fx][fy] {
            'X' => 0,
            x if x == dir_char_original => return None,
            _ => 1,
        };

        if is_turn {
            matrix[x][y] = dir_char_original;
        } else {
            matrix[x][y] = 'X';
        }

        matrix[fx][fy] = dir_char;

        (x, y) = (fx, fy);
        result += add;
    }

    Some(result)
}

// this is probably the most botched solution this aoc lol
fn main() {
    let matrix = INPUT
        .split("\n")
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let (mut guard_x, mut guard_y) = (usize::MAX, usize::MAX);

    for (i, line) in matrix.iter().enumerate() {
        for (j, val) in line.iter().enumerate() {
            if "^><v".contains(*val) {
                guard_x = i;
                guard_y = j;
                break;
            }
        }
    }

    // super inefficient
    let mut possible_matrices: Vec<Vec<Vec<char>>> = vec![];

    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            let mut dup = matrix.clone();

            if dup[i][j] != '.' {
                continue;
            }
            dup[i][j] = '#';

            possible_matrices.push(dup);
        }
    }

    let mut result = 0;

    for mut possible_matrix in possible_matrices.into_iter().rev() {
        let walkable = walk(&mut possible_matrix, (guard_x, guard_y));

        if walkable.is_none() {
            result += 1;
        }
    }

    println!("{}", result);
}
