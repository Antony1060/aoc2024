const INPUT: &str = include_str!("../input.txt");

fn main() {
    let [rules, pages] = INPUT.split("\n\n").collect::<Vec<_>>()[..] else {
        unreachable!()
    };

    let rules = rules
        .split("\n")
        .map(|rule| {
            let [f, s] = rule
                .split("|")
                .map(|it| it.parse::<i32>().unwrap())
                .collect::<Vec<_>>()[..]
            else {
                unreachable!()
            };

            (f, s)
        })
        .collect::<Vec<_>>();

    let pages = pages
        .split("\n")
        .map(|line| {
            line.split(",")
                .map(|it| it.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let correct_pages = pages
        .into_iter()
        .filter(|page| {
            for (f, s) in rules.iter() {
                if !page.contains(f) && !page.contains(s) {
                    continue;
                }

                let Some(first) = page.iter().position(|it| it == f) else {
                    continue;
                };

                let Some(second) = page.iter().position(|it| it == s) else {
                    continue;
                };

                if second <= first {
                    return false;
                }
            }

            true
        })
        .collect::<Vec<_>>();

    let result = correct_pages
        .into_iter()
        .map(|it| it[it.len() / 2])
        .sum::<i32>();

    println!("{}", result);
}
