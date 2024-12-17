use std::collections::HashSet;

const INPUT: &str = include_str!("../input.txt");

const UP: u8 = 0;
const DOWN: u8 = 1;
const LEFT: u8 = 2;
const RIGHT: u8 = 3;

fn dfs(graph: &Vec<Vec<usize>>, node: usize, visited: &mut HashSet<usize>) {
    visited.insert(node);

    for &node in &graph[node] {
        if !visited.contains(&node) {
            dfs(graph, node, visited);
        }
    }
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
            if i != 0 && matrix[i - 1][j] == *val {
                graph[i * m_width + j].push((i - 1) * m_width + j);
            }

            if i != matrix.len() - 1 && matrix[i + 1][j] == *val {
                graph[i * m_width + j].push((i + 1) * m_width + j);
            }

            if j != 0 && matrix[i][j - 1] == *val {
                graph[i * m_width + j].push(i * m_width + (j - 1));
            }

            if j != m_width - 1 && matrix[i][j + 1] == *val {
                graph[i * m_width + j].push(i * m_width + (j + 1));
            }
        }
    }

    let mut visited: HashSet<usize> = HashSet::new();
    let mut result = 0;

    while visited.len() < (matrix.len() * m_width) {
        let mut region = HashSet::new();
        dfs(
            &graph,
            graph
                .iter()
                .enumerate()
                .find(|(idx, _)| !visited.contains(idx))
                .unwrap()
                .0,
            &mut region,
        );

        visited.extend(region.iter());

        // println!(
        //     "region {}: {}",
        //     matrix[region.iter().next().unwrap() / m_width]
        //         [region.iter().next().unwrap() % m_width],
        //     region.len()
        // );

        let area = region.len();
        let perimeter_nodes: HashSet<(usize, u8)> = region
            .iter()
            .flat_map(|&node| {
                let (i, j) = (node / m_width, node % m_width);

                let mut curr = Vec::new();

                if i == 0 || !region.contains(&((i - 1) * m_width + j)) {
                    curr.push((node, UP));
                }
                if i == matrix.len() - 1 || !region.contains(&((i + 1) * m_width + j)) {
                    curr.push((node, DOWN));
                }
                if j == 0 || !region.contains(&(i * m_width + (j - 1))) {
                    curr.push((node, LEFT));
                }
                if j == m_width - 1 || !region.contains(&(i * m_width + (j + 1))) {
                    curr.push((node, RIGHT));
                }

                curr
            })
            .collect();

        let mut perimeter_visited: HashSet<(usize, u8)> = HashSet::new();
        let mut sides = 0;

        for &(perimeter_node, direction) in perimeter_nodes.iter() {
            if perimeter_visited.contains(&(perimeter_node, direction)) {
                continue;
            }

            sides += 1;

            for (i_shift, j_shift) in {
                if direction == UP || direction == DOWN {
                    [(0, -1), (0, 1)]
                } else {
                    [(-1, 0), (1, 0)]
                }
            } {
                let (mut i, mut j) = (perimeter_node / m_width, perimeter_node % m_width);

                let mut first = true;

                while perimeter_nodes.contains(&(i * m_width + j, direction))
                    && (first || !perimeter_visited.contains(&(i * m_width + j, direction)))
                {
                    first = false;
                    perimeter_visited.insert((i * m_width + j, direction));

                    if i == 0 && i_shift == -1 || j == 0 && j_shift == -1 {
                        break;
                    }

                    i = (i as isize + i_shift) as usize;
                    j = (j as isize + j_shift) as usize;
                }
            }
        }

        result += area * sides;
    }

    println!("{}", result);
}
