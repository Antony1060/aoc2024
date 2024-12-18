use std::collections::LinkedList;

const INPUT: &str = include_str!("../input.txt");

const GRID_W: usize = 71;
const GRID_H: usize = 71;

const FALLEN: usize = 1024;

fn neighbours(grid: &[char], curr: usize) -> Vec<usize> {
    let mut result = Vec::new();

    let x = curr % GRID_W;
    let y = curr / GRID_W;

    if x > 0 && grid[curr - 1] != '#' {
        result.push(curr - 1);
    }

    if x < GRID_W - 1 && grid[curr + 1] != '#' {
        result.push(curr + 1);
    }

    if y > 0 && grid[curr - GRID_W] != '#' {
        result.push(curr - GRID_W);
    }

    if y < GRID_H - 1 && grid[curr + GRID_W] != '#' {
        result.push(curr + GRID_W);
    }

    result
}

fn bfs(grid: &[char], start: usize, end: usize) -> Option<usize> {
    let mut visited = vec![false; GRID_W * GRID_H];
    let mut distances = vec![usize::MAX; GRID_W * GRID_H];
    let mut queue = LinkedList::new();
    queue.push_back(start);

    while let Some(node) = queue.pop_front() {
        if visited[node] {
            continue;
        }

        visited[node] = true;

        if node == end {
            return Some(distances[node] + 1);
        }

        for neighbour in neighbours(grid, node) {
            distances[neighbour] = distances[node] + 1;
            queue.push_back(neighbour);
        }
    }

    None
}

fn main() {
    let obstacles: Vec<(usize, usize)> = INPUT
        .lines()
        .map(|x| {
            let mut split = x.split(",").map(|val| val.parse::<usize>().unwrap());

            (split.next().unwrap(), split.next().unwrap())
        })
        .collect();

    let obstacles = obstacles
        .iter()
        .map(|it| it.1 * GRID_W + it.0)
        .collect::<Vec<_>>();

    let start = 0;
    let end = GRID_W * GRID_H - 1;

    let mut grid = vec!['.'; GRID_W * GRID_H];

    let obstacles = &obstacles[..FALLEN];
    for obstacle in obstacles {
        grid[*obstacle] = '#';
    }

    let result = bfs(&grid, start, end);

    match result {
        Some(val) => println!("{}", val),
        None => println!("No path found"),
    }
}
