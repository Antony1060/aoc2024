use std::collections::{HashMap, HashSet};

const INPUT: &str = include_str!("../input.txt");

type CoordPair = ((isize, isize), (isize, isize));

fn main() {
    let mut antennas: HashMap<char, Vec<(isize, isize)>> = HashMap::new();

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
        .flat_map(|(char, antennas)| {
            let mut pairs = vec![];

            for (i, a) in antennas.iter().enumerate() {
                for b in antennas.iter().skip(i + 1) {
                    pairs.push((*a, *b));
                }
            }

            pairs
        })
        .collect();

    let antinodes: HashSet<(isize, isize)> = pairs
        .into_iter()
        .flat_map(|((ax, ay), (bx, by))| {
            let dx = ax - bx;
            let dy = ay - by;

            vec![(ax + dx, ay + dy), (bx - dx, by - dy)]
        })
        .collect();

    let bound_x = INPUT.lines().count() as isize;
    let bound_y = INPUT.lines().next().unwrap().len() as isize;

    let result: Vec<(isize, isize)> = antinodes
        .into_iter()
        .filter(|(x, y)| *x >= 0 && *y >= 0 && *x < bound_x && *y < bound_y)
        .collect();

    println!("{}", result.len());
}
