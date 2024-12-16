use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet, LinkedList};

const INPUT: &str = include_str!("../input.txt");

#[derive(Clone, Eq, PartialEq)]
struct DijsktraNode {
    distance: usize,
    node: usize,
    direction: isize,
    cost: usize,
}

impl Ord for DijsktraNode {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.node.cmp(&other.node))
    }
}

impl PartialOrd for DijsktraNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn dijkstra(
    graph: &[Vec<usize>],
    start: usize,
    end: usize,
    history: &mut [HashSet<usize>],
) -> usize {
    let mut distances: Vec<usize> = vec![usize::MAX; graph.len()];
    let mut path: Vec<usize> = vec![0; graph.len()];

    let mut queue: BinaryHeap<DijsktraNode> = BinaryHeap::new();

    distances[start] = 0;
    queue.push(DijsktraNode {
        distance: 0,
        node: start,
        direction: 1,
        cost: 1,
    });
    path[start] = usize::MAX;

    while let Some(DijsktraNode {
        node,
        direction,
        cost,
        ..
    }) = queue.pop()
    {
        for &neighbour in &graph[node] {
            let new_distance = distances[node] + cost;

            let new_direction = neighbour as isize - node as isize;
            let new_cost = if direction == new_direction { 1 } else { 1001 };

            if new_distance > distances[neighbour] {
                continue;
            };

            history[neighbour].insert(node);

            if new_distance < distances[neighbour] {
                distances[neighbour] = new_distance;
                path[neighbour] = node;
                queue.push(DijsktraNode {
                    distance: new_distance,
                    node: neighbour,
                    cost: new_cost,
                    direction: new_direction,
                });
            }
        }
    }

    distances[end]
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

    let start = matrix
        .iter()
        .enumerate()
        .find_map(|(idx, line)| line.iter().position(|it| *it == 'S').map(|it| (idx, it)))
        .unwrap();

    let end = matrix
        .iter()
        .enumerate()
        .find_map(|(idx, line)| line.iter().position(|it| *it == 'E').map(|it| (idx, it)))
        .unwrap();

    let mut history: Vec<HashSet<usize>> = Vec::with_capacity(graph.len());
    for _ in 0..history.capacity() {
        history.push(HashSet::new());
    }

    let score = dijkstra(
        &graph,
        start.0 * m_width + start.1,
        end.0 * m_width + end.1,
        &mut history,
    );

    let mut total = HashSet::new();
    let mut stack: LinkedList<usize> = LinkedList::new();
    stack.push_back(end.0 * m_width + end.1);

    while let Some(curr) = stack.pop_front() {
        total.insert(curr);
        matrix[curr / m_width][curr % m_width] = 'O';

        for &it in history[curr].iter() {
            stack.push_back(it);
        }
    }

    for line in matrix {
        println!("{}", line.iter().collect::<String>());
    }

    println!("{}", score);
    println!("{}", total.len());
}
