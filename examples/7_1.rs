use rayon::iter::ParallelIterator;
use rayon::iter::{IntoParallelIterator, IntoParallelRefIterator};

const INPUT: &str = include_str!("../input.txt");

#[derive(Debug, Clone)]
struct EquationDefinition {
    result: u64,
    operands: Vec<u64>,
}

#[derive(Debug, Clone)]
enum Operation {
    Add,
    Multiply,
}

impl Operation {
    fn apply(&self, a: u64, b: u64) -> u64 {
        match self {
            Operation::Add => a + b,
            Operation::Multiply => a * b,
        }
    }
}

#[derive(Debug)]
struct Equation {
    definition: EquationDefinition,
    operations: Vec<Operation>,
}

impl Equation {
    // returns none if solution isn't result
    fn solve(&self) -> Option<u64> {
        if self.operations.len() != self.definition.operands.len() - 1 {
            return None;
        }

        let mut result = 0;

        for (i, operand) in self.definition.operands.iter().enumerate() {
            if i == 0 {
                result = *operand;
                continue;
            }

            result = self.operations[i - 1].apply(result, *operand);
        }

        if result != self.definition.result {
            return None;
        }

        Some(result)
    }
}

impl EquationDefinition {
    fn get_possible_equations(&self) -> Vec<Equation> {
        self.get_possible_equations_inner(vec![])
    }

    fn get_possible_equations_inner(&self, operations: Vec<Operation>) -> Vec<Equation> {
        if operations.len() == self.operands.len() - 1 {
            return vec![Equation {
                definition: self.clone(),
                operations,
            }];
        }

        let mut result = vec![];

        for operation in [Operation::Add, Operation::Multiply].into_iter() {
            let mut new_operations = operations.clone();
            new_operations.push(operation);

            result.extend(self.get_possible_equations_inner(new_operations));
        }

        result
    }
}

fn main() {
    let definitions: Vec<EquationDefinition> = INPUT
        .split('\n')
        .map(|line| {
            let mut parts = line.split(": ");
            let result = parts.next().unwrap().parse::<u64>().unwrap();
            let operands = parts
                .next()
                .unwrap()
                .split(' ')
                .map(|it| it.parse::<u64>().unwrap())
                .collect();

            EquationDefinition { result, operands }
        })
        .collect();

    let possible_equations: Vec<Vec<Equation>> = definitions
        .par_iter()
        .map(|definition| definition.get_possible_equations())
        .collect();

    let result = possible_equations
        .into_par_iter()
        .filter_map(|eqs| {
            eqs.into_par_iter()
                .filter_map(|eq| eq.solve())
                .find_any(|_| true)
        })
        .sum::<u64>();

    println!("{}", result);
}
