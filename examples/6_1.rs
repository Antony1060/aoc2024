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

fn walk(matrix: &mut [Vec<char>], (x, y): (usize, usize)) -> usize {
    if x == usize::MAX || y == usize::MAX {
        return 0;
    }

    let mut dir_char = matrix[x][y];

    let (mut fx, mut fy);

    loop {
        let (dx, dy) = dir_from_char(dir_char);

        (fx, fy) = (x as isize + dx, y as isize + dy);

        if fx < 0 || fx >= matrix.len() as isize || fy < 0 || fy >= matrix[0].len() as isize {
            return 1;
        }

        if matrix[fx as usize][fy as usize] != '#' {
            break;
        }

        dir_char = match dir_char {
            '^' => '>',
            '>' => 'v',
            '<' => '^',
            'v' => '<',
            _ => unreachable!(),
        };

        if dir_char == matrix[x][y] {
            return 0;
        }
    }

    let (fx, fy) = (fx as usize, fy as usize);

    let add = match matrix[fx][fy] {
        'X' => 0,
        _ => 1,
    };

    matrix[x][y] = 'X';
    matrix[fx][fy] = dir_char;

    add + walk(matrix, (fx, fy))
}

fn main() {
    let mut matrix = INPUT
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

    let result = walk(&mut matrix, (guard_x, guard_y));

    println!("{}", result);
}
