use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;
use std::collections::{HashMap, HashSet};

const INPUT: &str = include_str!("../input.txt");

fn make_ic(
    locked: &[String],
    left: &[String],
    connections: &HashMap<String, Vec<String>>,
) -> Vec<String> {
    let mut res = locked.to_vec();

    for it in left {
        let good = locked.iter().all(|locked| {
            let values = connections.get(locked).unwrap();
            values.contains(it)
        });

        if good {
            let mut locked_clone = locked.to_vec();
            locked_clone.push(it.clone());
            locked_clone.sort();

            let ic = make_ic(&locked_clone, &left[1..], connections);
            if ic.len() > res.len() {
                res = ic;
            }
        }
    }

    res
}

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

    println!("computing...");
    let ics: HashSet<Vec<String>> = connections
        .par_iter()
        .map(|(key, values)| make_ic(&[key.clone()], values, &connections))
        .collect();

    println!("processing...");

    let largest = ics.iter().max_by_key(|it| it.len()).unwrap().clone();

    let result = largest.join(",");

    println!("{}", result);
}
