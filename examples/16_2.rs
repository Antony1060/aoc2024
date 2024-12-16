use std::collections::{BinaryHeap, HashSet};

const INPUT: &str = include_str!("../input.txt");

fn dijkstra(graph: &[Vec<usize>], start: usize, end: usize, m_width: usize) -> usize {
    let mut visited: Vec<bool> = vec![false; graph.len()];
    let mut distances: Vec<usize> = vec![usize::MAX; graph.len()];
    let mut path: Vec<HashSet<usize>> = Vec::with_capacity(graph.len());
    for _ in 0..path.capacity() {
        path.push(HashSet::new());
    }

    let mut queue: BinaryHeap<(usize, usize)> = BinaryHeap::new();

    distances[start] = 0;
    queue.push((0, start));

    while let Some((_, node)) = queue.pop() {
        if node == end {
            break;
        }

        visited[node] = true;

        for &neighbour in &graph[node] {
            // if visited[neighbour] {
            //     continue;
            // }

            let ones_before_node = &path[node];

            let c_i = neighbour / m_width;
            let c_j = neighbour % m_width;

            let cost = ones_before_node
                .iter()
                .map(|one_before_node| {
                    let p_i = one_before_node / m_width;
                    let p_j = one_before_node % m_width;

                    if p_i != c_i && p_j != c_j {
                        1001
                    } else {
                        1
                    }
                })
                .min()
                .unwrap_or(1000);

            let new_distance = distances[node] + cost;

            if new_distance < distances[neighbour] {
                distances[neighbour] = new_distance;
                path[neighbour] = HashSet::from([node]);
                queue.push((new_distance, neighbour));
            }

            if new_distance == distances[neighbour] {
                path[neighbour].insert(node);
            }
        }
    }

    path.iter().map(|set| set.len()).sum()
}

fn main() {
    let matrix: Vec<Vec<char>> = INPUT.lines().map(|line| line.chars().collect()).collect();

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

    let total_best_path = dijkstra(
        &graph,
        start.0 * m_width + start.1,
        end.0 * m_width + end.1,
        m_width,
    );

    println!("{}", total_best_path);
}
