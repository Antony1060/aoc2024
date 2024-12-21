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

fn directional_on_numeric(code: &str) -> String {
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
    let mut sequence = String::new();
    for digit_char in code.chars() {
        let next = match digit_char {
            'A' => COORDS[10],
            _ => COORDS[(digit_char as u8 - b'0') as usize],
        };

        let is_last = curr.0 == 3;
        let is_next_last = next.0 == 3;

        println!("curr: {:?}, next: {:?}", curr, next);
        println!("is_last: {}, is_next_last: {}", is_last, is_next_last);

        if !is_last && next.1 < curr.1 {
            for _ in 0..(curr.1 - next.1) {
                sequence.push('<');
            }
        }

        if !is_next_last && next.0 > curr.0 {
            for _ in 0..(next.0 - curr.0) {
                sequence.push('v');
            }
        }

        if next.0 < curr.0 {
            for _ in 0..(curr.0 - next.0) {
                sequence.push('^');
            }
        }

        if is_next_last && next.1 > curr.1 {
            for _ in 0..(next.1 - curr.1) {
                sequence.push('>');
            }
        }

        if is_next_last && next.0 > curr.0 {
            for _ in 0..(next.0 - curr.0) {
                sequence.push('v');
            }
        }

        if !is_next_last && next.1 > curr.1 {
            for _ in 0..(next.1 - curr.1) {
                sequence.push('>');
            }
        }

        if is_last && next.1 <= curr.1 {
            for _ in 0..(curr.1 - next.1) {
                sequence.push('<');
            }
        }

        // if !is_next_last && next.0 > curr.0 {
        //     for _ in 0..(next.0 - curr.0) {
        //         sequence.push('v');
        //     }
        // }

        sequence.push('A');

        curr = next;
    }

    sequence
}

fn robot_code(code: &str) -> String {
    let first = directional_on_numeric(code);
    dbg!(&first);
    let result = directional_on_directional(&first);
    dbg!(&result);

    directional_on_directional(&result)
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
