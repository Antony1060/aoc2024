use rayon::iter::IndexedParallelIterator;
use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;
use std::collections::{HashMap, LinkedList};
use std::io::stdin;

const INPUT: &str = include_str!("../input.txt");

const CHEATS_ALLOWED: usize = 20;

fn neighbours(grid: &[char], curr: usize, m_width: usize) -> Vec<usize> {
    let mut result = Vec::new();

    let x = curr % m_width;
    let y = curr / m_width;

    if x > 0 && grid[curr - 1] != '#' {
        result.push(curr - 1);
    }

    if x < m_width - 1 && grid[curr + 1] != '#' {
        result.push(curr + 1);
    }

    if y > 0 && grid[curr - m_width] != '#' {
        result.push(curr - m_width);
    }

    if y < (grid.len() / m_width) - 1 && grid[curr + m_width] != '#' {
        result.push(curr + m_width);
    }

    result
}

fn bfs(
    grid: &[char],
    start: usize,
    end: usize,
    m_width: usize,
) -> Option<(Vec<usize>, Vec<usize>)> {
    let mut visited = vec![false; grid.len()];
    let mut distances = vec![usize::MAX; grid.len()];
    let mut path = vec![usize::MAX; grid.len()];
    let mut queue = LinkedList::new();
    queue.push_back(start);

    while let Some(node) = queue.pop_front() {
        if visited[node] {
            continue;
        }

        visited[node] = true;

        if node == end {
            return Some((distances, path));
        }

        for neighbour in neighbours(grid, node, m_width) {
            if visited[neighbour] {
                continue;
            }

            distances[neighbour] = distances[node] + 1;
            path[neighbour] = node;
            queue.push_back(neighbour);
        }
    }

    None
}

fn main() {
    let m_width = INPUT.lines().next().unwrap().len();

    let grid: Vec<char> = INPUT
        .lines()
        .collect::<Vec<_>>()
        .join("")
        .chars()
        .collect::<Vec<_>>();

    let start = grid.iter().position(|it| *it == 'S').unwrap();

    let end = grid.iter().position(|it| *it == 'E').unwrap();

    let (distances, path) = bfs(&grid, start, end, m_width).unwrap();

    let distance = distances[end];

    println!("Distance: {}", distance);

    let mut full_path: Vec<usize> = Vec::new();
    full_path.push(end);
    let mut curr = path[end];
    let mut grid_draw = grid.clone();
    while curr != usize::MAX {
        full_path.push(curr);
        curr = path[curr];
    }

    full_path.reverse();

    for &idx in &full_path {
        grid_draw[idx] = 'O';
    }

    let saves: Vec<usize> = full_path
        .par_iter()
        .enumerate()
        .flat_map(|(idx, i)| {
            let (i_x, i_y) = (i / m_width, i % m_width);

            if idx % 100 == 0 {
                println!("{}/{}", idx + 1, full_path.len());
            }

            let mut res = Vec::new();

            for j_idx in (idx + 1)..full_path.len() {
                let mut grid_cleaned = grid.clone();

                let j = full_path[j_idx];
                let (j_x, j_y) = (j / m_width, j % m_width);

                let (l_x, l_y) = (i_x.min(j_x), i_y.min(j_y));
                let (h_x, h_y) = (i_x.max(j_x), i_y.max(j_y));
                let manhattan = (h_x - l_x) + (h_y - l_y);

                if manhattan > CHEATS_ALLOWED {
                    continue;
                }

                for iter_i in l_x..=h_x {
                    grid_cleaned[iter_i * m_width + (if i_y < j_y { l_y } else { h_y })] = '.';
                }

                for iter_j in l_y..=h_y {
                    grid_cleaned[(if i_x < j_x { h_x } else { l_x }) * m_width + iter_j] = '.';
                }

                let a = i_x * m_width + i_y;
                let b = j_x * m_width + j_y;

                let dist = distances[a] + manhattan + (distances[end] - distances[b]);

                if dist > distance {
                    continue;
                }

                let saved = distance - dist;

                if saved == 0 {
                    continue;
                }

                // grid_cleaned[i_x * m_width + i_y] = 's';
                // grid_cleaned[j_x * m_width + j_y] = 'e';

                // for (idx, val) in grid_cleaned.iter().enumerate() {
                //     print!("{val}");
                //     if (idx + 1) % m_width == 0 {
                //         println!()
                //     }
                // }

                // stdin().read_line(&mut String::new());

                res.push(saved);
            }

            res
        })
        .collect();

    let mut histogram = HashMap::new();
    for save in saves {
        let Some(val) = histogram.get_mut(&save) else {
            histogram.insert(save, 1);
            continue;
        };

        *val += 1;
    }

    let result = histogram
        .into_iter()
        .filter(|(key, _)| *key >= 100)
        .map(|(_, value)| value)
        .sum::<usize>();

    println!("Result: {}", result);
}
