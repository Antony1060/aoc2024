use std::cmp::min;
use std::collections::VecDeque;

const INPUT: &str = include_str!("../input.txt");

fn all_paths_bfs(graph: &[Vec<usize>], start: usize, end: usize) -> Vec<Vec<usize>> {
    let mut paths: VecDeque<Vec<usize>> = VecDeque::new();
    let mut path: Vec<usize> = Vec::new();

    let mut ret_paths: Vec<Vec<usize>> = Vec::new();

    path.push(start);
    paths.push_back(path);

    while let Some(curr_path) = paths.pop_front() {
        let last = *curr_path.last().unwrap();

        if last == end {
            ret_paths.push(curr_path.clone());
        }

        for neighbour in &graph[last] {
            if curr_path.contains(neighbour) {
                continue;
            }

            let mut new_path = curr_path.clone();
            new_path.push(*neighbour);
            paths.push_back(new_path);
        }
    }

    ret_paths
}

fn main() {
    let mut matrix: Vec<Vec<char>> = INPUT.lines().map(|line| line.chars().collect()).collect();

    let m_width = matrix[0].len();

    let mut graph: Vec<Vec<usize>> = Vec::with_capacity(matrix.len() * m_width);
    for _ in 0..graph.capacity() {
        graph.push(Vec::new());
    }

    for (i, line) in matrix.iter().enumerate() {
        for (j, val) in line.iter().enumerate() {
            if *val == '#' {
                continue;
            }

            if i != 0 && matrix[i - 1][j] != '#' {
                graph[i * m_width + j].push((i - 1) * m_width + j);
            }

            if i != matrix.len() - 1 && matrix[i + 1][j] != '#' {
                graph[i * m_width + j].push((i + 1) * m_width + j);
            }

            if j != 0 && matrix[i][j - 1] != '#' {
                graph[i * m_width + j].push(i * m_width + (j - 1));
            }

            if j != m_width - 1 && matrix[i][j + 1] != '#' {
                graph[i * m_width + j].push(i * m_width + (j + 1));
            }
        }
    }

    let mut start = (0, 0);

    'outer: for (i, line) in matrix.iter().enumerate() {
        for (j, val) in line.iter().enumerate() {
            if *val == 'S' {
                start = (i, j);
                break 'outer;
            }
        }
    }

    let mut end = (0, 0);

    'outer: for (i, line) in matrix.iter().enumerate() {
        for (j, val) in line.iter().enumerate() {
            if *val == 'E' {
                end = (i, j);
                break 'outer;
            }
        }
    }

    let paths = all_paths_bfs(&graph, start.0 * m_width + start.1, end.0 * m_width + end.1);

    let mut min_score = usize::MAX;

    for path in paths {
        let mut mx = matrix.clone();

        let mut score = 0;

        for (idx, step) in path.iter().enumerate() {
            let c_i = step / m_width;
            let c_j = step % m_width;
            mx[c_i][c_j] = '+';

            score += if idx < 2 {
                if idx == 0 {
                    1000
                } else {
                    1
                }
            } else {
                let p_i = path[idx - 2] / m_width;
                let p_j = path[idx - 2] % m_width;

                if p_i != c_i && p_j != c_j {
                    1001
                } else {
                    1
                }
            };
        }

        // for line in mx {
        //     println!("{}", line.iter().collect::<String>());
        // }
        //
        // println!("{}", score);

        min_score = min(min_score, score);
    }

    println!("{}", min_score);
}
