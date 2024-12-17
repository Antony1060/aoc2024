use std::collections::{HashMap, HashSet};

const INPUT: &str = include_str!("../input.txt");

type Coord = (isize, isize);
type CoordPair = ((isize, isize), (isize, isize));

fn walk_coords(src: Coord, step: Coord, low_bound: isize, high_bound: isize) -> Vec<Coord> {
    let mut res = vec![];

    let mut curr = src;
    while curr.0 >= low_bound && curr.1 >= low_bound && curr.0 <= high_bound && curr.1 <= high_bound
    {
        res.push(curr);

        curr = (curr.0 + step.0, curr.1 + step.1);
    }

    res
}

fn main() {
    let mut antennas: HashMap<char, Vec<Coord>> = HashMap::new();

    for (x, line) in INPUT.split('\n').enumerate() {
        for (y, char) in line.chars().enumerate() {
            if char != '.' {
                antennas
                    .entry(char)
                    .or_default()
                    .push((x as isize, y as isize));
            }
        }
    }

    let pairs: Vec<CoordPair> = antennas
        .into_iter()
        .flat_map(|(_, antennas)| {
            let mut pairs = vec![];

            for (i, a) in antennas.iter().enumerate() {
                for b in antennas.iter().skip(i + 1) {
                    pairs.push((*a, *b));
                }
            }

            pairs
        })
        .collect();

    let bound_x = INPUT.lines().count() as isize;
    let bound_y = INPUT.lines().next().unwrap().len() as isize;

    let antinodes: HashSet<Coord> = pairs
        .into_iter()
        .flat_map(|((ax, ay), (bx, by))| {
            let dx = ax - bx;
            let dy = ay - by;

            let mut coords = walk_coords((ax, ay), (dx, dy), 0, bound_x + bound_y);

            coords.extend(walk_coords((ax, ay), (-dx, -dy), 0, bound_x + bound_y));

            coords
        })
        .collect();

    let result: Vec<Coord> = antinodes
        .into_iter()
        .filter(|(x, y)| *x >= 0 && *y >= 0 && *x < bound_x && *y < bound_y)
        .collect();

    println!("{}", result.len());
}
