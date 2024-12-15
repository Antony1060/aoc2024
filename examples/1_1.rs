const INPUT: &str = include_str!("../input.txt");

fn main() {
    let mut l1: Vec<i32> = vec![];
    let mut l2: Vec<i32> = vec![];

    for (n1, n2) in INPUT.split("\n").map(|l| l.split("   ")).map(|mut nums| {
        (
            nums.next().unwrap().parse::<i32>().unwrap(),
            nums.next().unwrap().parse::<i32>().unwrap(),
        )
    }) {
        l1.push(n1);
        l2.push(n2);
    }

    l1.sort();
    l2.sort();

    let res = l1
        .iter()
        .zip(l2.iter())
        .map(|(n1, n2)| (n1 - n2).abs())
        .sum::<i32>();

    println!("{}", res);
}
