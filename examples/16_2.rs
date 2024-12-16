use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet, LinkedList};

const INPUT: &str = include_str!("../input.txt");

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct DijkstraNode {
    node: usize,
    direction: isize,
}

#[derive(Clone, Eq, PartialEq, Hash)]
struct DijsktraState {
    node: DijkstraNode,
    distance: usize,
}

impl Ord for DijsktraState {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .distance
            .cmp(&self.distance)
            .then_with(|| self.node.node.cmp(&other.node.node))
    }
}

impl PartialOrd for DijsktraState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn dijkstra(
    graph: &[Vec<usize>],
    start: usize,
    end: usize,
    history: &mut HashMap<DijkstraNode, Vec<DijkstraNode>>,
) -> usize {
    let mut distances: HashMap<DijkstraNode, usize> = HashMap::new();

    let mut queue: BinaryHeap<DijsktraState> = BinaryHeap::new();

    let start_node = DijkstraNode {
        node: start,
        direction: 1,
    };

    distances.insert(start_node, 0);

    queue.push(DijsktraState {
        distance: 0,
        node: start_node,
    });

    while let Some(DijsktraState { node, distance }) = queue.pop() {
        if node.node == end {
            return distance;
        }

        for &neighbour in &graph[node.node] {
            let new_direction = neighbour as isize - node.node as isize;
            let new_cost = if node.direction == new_direction {
                1
            } else {
                1001
            };
            let new_distance = distance + new_cost;

            let neighbour_node = DijkstraNode {
                node: neighbour,
                direction: new_direction,
            };

            match distances.get(&neighbour_node) {
                Some(&dist) if new_distance > dist => {}
                Some(&dist) if dist == new_distance => {
                    history.entry(neighbour_node).or_default().push(node);
                }
                _ => {
                    history.insert(neighbour_node, vec![node]);
                    distances.insert(neighbour_node, new_distance);
                    queue.push(DijsktraState {
                        distance: new_distance,
                        node: neighbour_node,
                    });
                }
            }
        }
    }

    *distances.iter().find(|(key, _)| key.node == end).unwrap().1
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

    let start = start.0 * m_width + start.1;
    let end = end.0 * m_width + end.1;

    let mut history = HashMap::new();

    let score = dijkstra(&graph, start, end, &mut history);
    let mut visited: HashSet<DijkstraNode> = HashSet::new();
    let mut unique: HashSet<usize> = HashSet::new();

    let mut stack = LinkedList::new();

    for end_dir in [-1, 1, -(m_width as isize), m_width as isize] {
        stack.push_back(DijkstraNode {
            node: end,
            direction: end_dir,
        });
    }

    while let Some(node) = stack.pop_front() {
        if visited.contains(&node) {
            continue;
        }

        visited.insert(node);
        unique.insert(node.node);

        matrix[node.node / m_width][node.node % m_width] = 'O';

        if let Some(path) = history.get(&node) {
            stack.extend(path.iter().cloned());
        }
    }

    for line in matrix {
        println!("{}", line.iter().collect::<String>());
    }

    println!("{}", unique.len());
    println!("{}", score);
}
