use std::collections::{HashMap, HashSet};
use std::str::FromStr;

const INPUT: &str = include_str!("../input.txt");

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
enum Condition {
    Xor,
    And,
    Or,
}

impl Condition {
    fn eval(&self, lhs: bool, rhs: bool) -> bool {
        match self {
            Condition::Xor => lhs ^ rhs,
            Condition::And => lhs & rhs,
            Condition::Or => lhs | rhs,
        }
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct Expression {
    condition: Condition,
    lhs_name: String,
    rhs_name: String,
    output_var: String,
}

impl FromStr for Expression {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut s = s.split(" ");
        let lhs_name = s.next().unwrap().to_string();
        let condition = match s.next().unwrap() {
            "AND" => Condition::And,
            "OR" => Condition::Or,
            "XOR" => Condition::Xor,
            _ => unreachable!(),
        };
        let rhs_name = s.next().unwrap().to_string();
        s.next().unwrap();
        let output_var = s.next().unwrap().to_string();

        Ok(Expression {
            condition,
            lhs_name,
            rhs_name,
            output_var,
        })
    }
}

fn main() {
    let [vars, logic] = INPUT.split("\n\n").collect::<Vec<_>>()[..] else {
        unreachable!()
    };

    let mut vars: HashMap<String, bool> = vars
        .lines()
        .map(|l| l.split(": "))
        .map(|mut s| {
            (
                s.next().unwrap().to_string(),
                s.next().unwrap().parse::<u8>().unwrap() == 1,
            )
        })
        .collect();

    let logic = logic
        .lines()
        .map(|l| l.parse::<Expression>().unwrap())
        .collect::<Vec<_>>();

    let mut logic: HashSet<Expression> = HashSet::from_iter(logic);

    while !logic.is_empty() {
        let to_process = logic
            .iter()
            .find(|exp| vars.contains_key(&exp.lhs_name) && vars.contains_key(&exp.rhs_name))
            .unwrap();

        let lhs = vars[&to_process.lhs_name];
        let rhs = vars[&to_process.rhs_name];
        let output = to_process.condition.eval(lhs, rhs);
        vars.insert(to_process.output_var.clone(), output);

        logic.remove(&to_process.clone());
    }

    let zs = vars
        .iter()
        .filter(|(key, _)| key.starts_with("z"))
        .map(|(key, value)| (key[1..].parse::<u8>().unwrap(), value));

    let mut res: u64 = 0;

    for (idx, &value) in zs {
        res |= (value as u64) << idx as u64;
    }

    println!("{}", res);
}
