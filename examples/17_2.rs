const INPUT: &str = include_str!("../input.txt");

const REG_A: usize = 0;
const REG_B: usize = 1;
const REG_C: usize = 2;

const INST_ADV: u8 = 0;
const INST_BXL: u8 = 1;
const INST_BST: u8 = 2;
const INST_JNZ: u8 = 3;
const INST_BXC: u8 = 4;
const INST_OUT: u8 = 5;
const INST_BDV: u8 = 6;
const INST_CDV: u8 = 7;

fn parse_combo(reg: &mut [u64; 3], op: u8) -> u64 {
    match op {
        0..=3 => op as u64,
        4..=6 => reg[((op - 1) % 3) as usize],
        _ => unreachable!(),
    }
}

fn do_div(reg: &mut [u64; 3], op: u8) -> u64 {
    let num = reg[REG_A];
    let denom = 2u64.pow(parse_combo(reg, op) as u32);

    num / denom
}

fn process_adv(reg: &mut [u64; 3], ip: u64, op: u8) -> u64 {
    reg[REG_A] = do_div(reg, op);
    ip + 2
}

fn process_bxl(reg: &mut [u64; 3], ip: u64, op: u8) -> u64 {
    reg[REG_B] ^= op as u64;
    ip + 2
}

fn process_bst(reg: &mut [u64; 3], ip: u64, op: u8) -> u64 {
    let combo = parse_combo(reg, op);
    reg[REG_B] = combo % 8;
    ip + 2
}

fn process_jnz(reg: &mut [u64; 3], ip: u64, op: u8) -> u64 {
    if reg[REG_A] == 0 {
        return ip + 2;
    }

    op as u64
}

fn process_bxc(reg: &mut [u64; 3], ip: u64, _op: u8) -> u64 {
    reg[REG_B] ^= reg[REG_C];
    ip + 2
}

fn process_out(reg: &mut [u64; 3], ip: u64, op: u8, stdout: &mut Vec<u8>) -> u64 {
    let combo = parse_combo(reg, op);
    stdout.push((combo % 8) as u8);
    ip + 2
}

fn process_bdv(reg: &mut [u64; 3], ip: u64, op: u8) -> u64 {
    reg[REG_B] = do_div(reg, op);
    ip + 2
}

fn process_cdv(reg: &mut [u64; 3], ip: u64, op: u8) -> u64 {
    reg[REG_C] = do_div(reg, op);
    ip + 2
}

fn run_program(reg: &[u64; 3], program: &[u8]) -> Vec<u8> {
    let mut reg = reg.clone();

    let mut stdout: Vec<u8> = Vec::new();

    let mut ip: u64 = 0;
    while (ip as usize) < program.len() - 1 {
        let op = program[ip as usize + 1];
        match program[ip as usize] {
            INST_ADV => ip = process_adv(&mut reg, ip, op),
            INST_BXL => ip = process_bxl(&mut reg, ip, op),
            INST_BST => ip = process_bst(&mut reg, ip, op),
            INST_JNZ => ip = process_jnz(&mut reg, ip, op),
            INST_BXC => ip = process_bxc(&mut reg, ip, op),
            INST_OUT => ip = process_out(&mut reg, ip, op, &mut stdout),
            INST_BDV => ip = process_bdv(&mut reg, ip, op),
            INST_CDV => ip = process_cdv(&mut reg, ip, op),
            _ => unreachable!(),
        }
    }

    stdout
}

fn main() {
    let [_, program] = INPUT.split("\n\n").collect::<Vec<_>>()[..] else {
        unreachable!()
    };

    let program: Vec<u8> = program
        .split(" ")
        .last()
        .unwrap()
        .split(",")
        .map(|x| x.parse::<u8>().unwrap())
        .collect();

    // my test case had a property where it did "adv 3" right after outputting something
    //  update this with the testcase accordingly
    let last_instruction_denom = 2u64.pow(3);

    let mut success = 0;
    let mut result = 0;
    while success != program.len() {
        let current_lookup = &program[(program.len() - (success + 1))..];
        loop {
            let reg: [u64; 3] = [result, 0, 0];
            let stdout = run_program(&reg, &program);

            if stdout == current_lookup {
                result *= last_instruction_denom;
                success += 1;
                break;
            }

            result += 1;
        }
    }

    println!("{}", result / last_instruction_denom);
}
