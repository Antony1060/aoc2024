use std::collections::{HashMap, HashSet};

const INPUT: &str = include_str!("../input.txt");

fn main() {
    let mut connections: HashMap<String, Vec<String>> = HashMap::new();

    for line in INPUT.lines() {
        let [f, s] = line.split("-").collect::<Vec<_>>()[..] else {
            unreachable!()
        };

        match connections.get_mut(f) {
            Some(conn) => conn.push(s.to_string()),
            None => {
                connections.insert(f.to_string(), vec![s.to_string()]);
            }
        }

        match connections.get_mut(s) {
            Some(conn) => conn.push(f.to_string()),
            None => {
                connections.insert(s.to_string(), vec![f.to_string()]);
            }
        }
    }

    let mut triples: HashSet<[String; 3]> = HashSet::new();

    for (key, values) in connections.iter() {
        for i in values.iter() {
            for j in values.iter() {
                if i == j {
                    continue;
                }

                let j_values = connections.get(j).unwrap();

                if j_values.contains(i) {
                    let mut sorted = [key, i, j];
                    sorted.sort();

                    triples.insert([sorted[0].clone(), sorted[1].clone(), sorted[2].clone()]);
                }
            }
        }
    }

    for triple in triples.iter() {
        println!("{:?}", triple);
    }
    println!("{}", triples.len());

    let result = triples
        .iter()
        .filter(|triple| triple.iter().any(|it| it.starts_with('t')))
        .count();

    println!("{}", result);
}
