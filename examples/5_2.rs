const INPUT: &str = include_str!("../input.txt");

fn find_faults(page: &[i32], rules: &[(i32, i32)]) -> Vec<(i32, i32)> {
    let mut wrong: Vec<(i32, i32)> = vec![];
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
            wrong.push((*f, *s));
        }
    }

    wrong
}

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

    let bad_pages = pages
        .into_iter()
        .map(|page| {
            let wrong: Vec<(i32, i32)> = find_faults(&page, &rules);

            (page, wrong)
        })
        .filter(|(_, wrong)| !wrong.is_empty())
        .collect::<Vec<_>>();

    let fixed_pages = bad_pages
        .into_iter()
        .map(|(mut page, mut faults)| {
            while !faults.is_empty() {
                let (f, s) = faults[0];

                let Some(first) = page.iter().position(|it| *it == f) else {
                    continue;
                };

                let Some(second) = page.iter().position(|it| *it == s) else {
                    continue;
                };

                page.swap(first, second);
                faults = find_faults(&page, &rules);
            }

            page
        })
        .collect::<Vec<_>>();

    let result = fixed_pages
        .into_iter()
        .map(|it| it[it.len() / 2])
        .sum::<i32>();

    println!("{}", result);
}
