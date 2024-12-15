const INPUT: &str = include_str!("../input.txt");

fn dfs(graph: &Vec<Vec<usize>>, visited: &mut Vec<usize>, node: usize) {
    visited.push(node);

    for &neighbour in &graph[node] {
        dfs(graph, visited, neighbour);
    }
}

fn main() {
    let matrix: Vec<Vec<u8>> = INPUT
        .lines()
        .map(|line| line.chars().map(|c| c as u8 - b'0').collect())
        .collect();

    let m_width = matrix[0].len();

    let mut graph: Vec<Vec<usize>> = Vec::with_capacity(matrix.len() * m_width);

    for _ in 0..graph.capacity() {
        graph.push(vec![]);
    }

    for (i, line) in matrix.iter().enumerate() {
        for (j, val) in line.iter().enumerate() {
            if i != 0 && matrix[i - 1][j] > *val && matrix[i - 1][j] - *val == 1 {
                graph[i * m_width + j].push((i - 1) * m_width + j);
            }

            if i != matrix.len() - 1 && matrix[i + 1][j] > *val && matrix[i + 1][j] - *val == 1 {
                graph[i * m_width + j].push((i + 1) * m_width + j);
            }

            if j != 0 && matrix[i][j - 1] > *val && matrix[i][j - 1] - *val == 1 {
                graph[i * m_width + j].push(i * m_width + j - 1);
            }

            if j != m_width - 1 && matrix[i][j + 1] > *val && matrix[i][j + 1] - *val == 1 {
                graph[i * m_width + j].push(i * m_width + j + 1);
            }
        }
    }

    let mut result = 0;

    for (i, line) in matrix.iter().enumerate() {
        for (j, val) in line.iter().enumerate() {
            if *val != 0 {
                continue;
            }

            let mut visited: Vec<usize> = vec![];

            dfs(&graph, &mut visited, i * m_width + j);

            let score = visited
                .into_iter()
                .filter(|value| matrix[value / m_width][value % m_width] == 9)
                .count();

            result += score;
        }
    }

    println!("{}", result);
}
