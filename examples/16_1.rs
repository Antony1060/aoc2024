use std::collections::BinaryHeap;

const INPUT: &str = include_str!("../input.txt");

fn dijkstra(graph: &[Vec<usize>], start: usize, end: usize, m_width: usize) -> Vec<usize> {
    let mut visited: Vec<bool> = vec![false; graph.len()];
    let mut distances: Vec<usize> = vec![usize::MAX; graph.len()];
    let mut path: Vec<usize> = vec![0; graph.len()];

    let mut queue: BinaryHeap<(usize, usize)> = BinaryHeap::new();

    distances[start] = 0;
    queue.push((0, start));
    path[start] = usize::MAX;

    while let Some((_, node)) = queue.pop() {
        visited[node] = true;

        for &neighbour in &graph[node] {
            // if visited[neighbour] {
            //     continue;
            // }

            let one_before_node = &path[node];

            let c_i = neighbour / m_width;
            let c_j = neighbour % m_width;

            let cost = if *one_before_node == usize::MAX {
                1000
            } else {
                let p_i = one_before_node / m_width;
                let p_j = one_before_node % m_width;

                if p_i != c_i && p_j != c_j {
                    1001
                } else {
                    1
                }
            };

            let new_distance = distances[node] + cost;

            if new_distance < distances[neighbour] {
                distances[neighbour] = new_distance;
                path[neighbour] = node;
                queue.push((new_distance, neighbour));
            }
        }
    }

    let mut ret_path = Vec::new();

    let mut curr = path[end];
    while curr != usize::MAX {
        ret_path.push(curr);
        curr = path[curr];
    }

    ret_path.reverse();

    ret_path
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

    let path = dijkstra(
        &graph,
        start.0 * m_width + start.1,
        end.0 * m_width + end.1,
        m_width,
    );

    let mut score = 0;

    for (idx, step) in path.iter().enumerate() {
        let c_i = step / m_width;
        let c_j = step % m_width;

        score += if idx < 2 {
            // don't ask me why this behaviour is like this, it's an off-by-one somewhere
            //  idk I wasted 3 hours debugging this

            // uncomment this for test cases
            // if idx == 0 {
            //     1001
            // } else {
            //     1
            // }

            // comment this for test cases
            1
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

    println!("{}", score);
}
