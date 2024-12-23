use std::collections::{HashMap, HashSet};

const INPUT: &str = include_str!("../input.txt");

fn make_ic(
    locked: &[String],
    left: &[String],
    connections: &HashMap<String, Vec<String>>,
) -> HashSet<Vec<String>> {
    let mut sets = HashSet::new();
    sets.insert(locked.to_vec());

    for it in left {
        let good = locked.iter().all(|locked| {
            let values = connections.get(locked).unwrap();
            values.contains(it)
        });

        if good {
            let mut locked_clone = locked.to_vec();
            locked_clone.push(it.clone());
            sets.extend(make_ic(&locked_clone, &left[1..], connections));
        }
    }

    sets
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

    for (key, values) in connections.iter() {
        println!("doing: {key}");
        let ic = make_ic(&[key.clone()], values, &connections);

        let ic = ic
            .into_iter()
            .filter(|it| it.len() >= largest)
            .collect::<Vec<_>>();

        if ic.is_empty() {
            continue;
        }

        let ic_largest = ic.iter().max_by_key(|it| it.len()).unwrap().len();

        if ic_largest > largest {
            largest = ic_largest;
            sets.extend(ic);
        }

        sets.retain(|it| it.len() == largest);
        println!("sets: {:?}", sets);
    }

    // dbg!(&sets);

    let mut largest = sets.iter().max_by_key(|it| it.len()).unwrap().clone();
    largest.sort();

    let result = largest.join(",");

    println!("{}", result);
}
