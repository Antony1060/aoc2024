use std::collections::HashSet;

const INPUT: &str = include_str!("../input.txt");

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

        let area = region.len();
        let perimeter = region
            .iter()
            .map(|&node| {
                let (i, j) = (node / m_width, node % m_width);

                let mut count = 0;

                if i == 0 || !region.contains(&((i - 1) * m_width + j)) {
                    count += 1;
                }

                if i == matrix.len() - 1 || !region.contains(&((i + 1) * m_width + j)) {
                    count += 1;
                }

                if j == 0 || !region.contains(&(i * m_width + (j - 1))) {
                    count += 1;
                }

                if j == m_width - 1 || !region.contains(&(i * m_width + (j + 1))) {
                    count += 1;
                }

                count
            })
            .sum::<usize>();

        result += area * perimeter;
    }

    println!("{}", result);
}
