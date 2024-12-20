use std::collections::{HashMap, HashSet, LinkedList};

const INPUT: &str = include_str!("../input.txt");

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

fn bfs(grid: &[char], start: usize, end: usize, m_width: usize) -> Option<(usize, Vec<usize>)> {
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
            return Some((distances[node] + 1, path));
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

    let (distance, path) = bfs(&grid, start, end, m_width).unwrap();

    println!("Distance: {}", distance);

    let mut full_path: Vec<usize> = Vec::new();
    let mut curr = path[end];
    while curr != usize::MAX {
        full_path.push(curr);
        curr = path[curr];
    }

    let cheats = full_path
        .iter()
        .flat_map(|it| {
            let x = it % m_width;
            let y = it / m_width;

            let mut result = Vec::new();

            if x > 0 && grid[it - 1] == '#' {
                result.push(it - 1);
            }

            if x < m_width - 1 && grid[it + 1] == '#' {
                result.push(it + 1);
            }

            if y > 0 && grid[it - m_width] == '#' {
                result.push(it - m_width);
            }

            if y < (grid.len() / m_width) - 1 && grid[it + m_width] == '#' {
                result.push(it + m_width);
            }

            result
        })
        .collect::<HashSet<_>>();

    let mut histogram = HashMap::new();

    for cheat in cheats {
        let mut grid_iter = grid.clone();
        grid_iter[cheat] = '.';

        let Some((cheat_distance, _)) = bfs(&grid_iter, start, end, m_width) else {
            continue;
        };

        let saved = distance - cheat_distance;
        if saved == 0 {
            continue;
        }

        let Some(val) = histogram.get_mut(&saved) else {
            histogram.insert(saved, 1);
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
