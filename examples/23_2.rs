use std::collections::{HashMap, HashSet};
use std::hash::Hash;

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

    let mut sets: HashSet<Vec<String>> = HashSet::new();

    dbg!(&connections);

    'out: for (key, values) in connections.iter() {
        for i in values.iter() {
            for j in values.iter() {
                if i == j {
                    continue;
                }

                let j_values = connections.get(j).unwrap();

                if !j_values.contains(i) {
                    continue 'out;
                }
            }
        }

        let mut set = vec![key.to_string()];
        set.extend(values.iter().cloned());
        set.sort();
        sets.insert(set);
    }

    dbg!(&sets);

    let largest = sets.iter().max_by_key(|it| it.len()).unwrap();

    let result = largest.join(",");

    println!("{}", result);
}
