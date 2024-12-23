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

    let mut sets: HashSet<Vec<String>> = HashSet::new();
    let mut largest = 0;

    // dbg!(&connections);

    let ics: Vec<Vec<String>> = connections
        .par_iter()
        .map(|(key, values)| {
            println!("doing: {key}");
            let ic = make_ic(&[key.clone()], values, &connections);
            println!("done: {key}");

            ic
        })
        .collect();

    for ic in ics {
        if ic.len() < largest {
            continue;
        }

        largest = ic.len();

        sets.insert(ic);

        sets.retain(|it| it.len() == largest);
        println!("sets: {:?}", sets);
    }

    // dbg!(&sets);

    let mut largest = sets.iter().max_by_key(|it| it.len()).unwrap().clone();
    largest.sort();

    let result = largest.join(",");

    println!("{}", result);
}
