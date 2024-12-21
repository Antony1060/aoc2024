use std::collections::LinkedList;

const INPUT: &str = include_str!("../input.txt");

const CHEATS_ALLOWED: usize = 20;
const DIST_LIMIT: usize = 100;

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

    distances[start] = 0;

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
    while curr != usize::MAX {
        full_path.push(curr);
        curr = path[curr];
    }

    full_path.reverse();

    let pairs = full_path.iter().enumerate().flat_map(|(idx, i)| {
        let (i_x, i_y) = (i / m_width, i % m_width);

        full_path[idx + 1..].iter().map(move |j| {
            let (j_x, j_y) = (j / m_width, j % m_width);

            (*i, j, i_x, i_y, j_x, j_y)
        })
    });

    let mut iter = 0;
    let result: usize = pairs
        .filter(|&(i, j, i_x, i_y, j_x, j_y)| {
            iter += 1;
            let manhattan = (i_x.abs_diff(j_x)) + (i_y.abs_diff(j_y));

            if manhattan > CHEATS_ALLOWED {
                return false;
            }

            let dist = distances[i] + manhattan + (distances[end] - distances[*j]);

            if dist > distance {
                return false;
            }

            let saved = distance - dist;

            saved >= DIST_LIMIT
        })
        .count();

    println!("Result: {}", result);
}
