const INPUT: &str = include_str!("../input.txt");

fn main() {
    let [board, moves] = INPUT.split("\n\n").collect::<Vec<_>>()[..] else {
        unreachable!()
    };

    let mut matrix = board
        .lines()
        .map(|it| it.chars().collect::<Vec<_>>())
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
        while matrix[lookup.0 as usize][lookup.1 as usize] == 'O' {
            let new_lookup = (lookup.0 + dir.0, lookup.1 + dir.1);
            if new_lookup.0 < 0
                || new_lookup.1 < 0
                || new_lookup.0 >= matrix.len() as isize
                || new_lookup.1 >= matrix[0].len() as isize
            {
                break;
            }

            lookup = new_lookup;
        }

        let lookup = (lookup.0 as usize, lookup.1 as usize);

        if matrix[lookup.0][lookup.1] == '#' {
            continue;
        }

        matrix[lookup.0][lookup.1] = 'O';
        matrix[curr.0][curr.1] = '.';
        curr = (
            (curr.0 as isize + dir.0) as usize,
            (curr.1 as isize + dir.1) as usize,
        );
        matrix[curr.0][curr.1] = '@';
    }

    let result = matrix
        .iter()
        .enumerate()
        .flat_map(|(i, line)| {
            line.iter()
                .enumerate()
                .filter_map(move |(j, &val)| if val != 'O' { None } else { Some(i * 100 + j) })
        })
        .sum::<usize>();

    println!("{}", result);
}
