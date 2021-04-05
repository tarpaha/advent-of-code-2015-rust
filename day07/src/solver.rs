use std::collections::HashMap;
use crate::parser::Action;

pub fn part1(instructions: &HashMap<String, Action>, wire: &str) -> u16 {
    let mut memo = HashMap::new();
    get_signal(instructions, &wire.to_string(), &mut memo)
}

pub fn part2(instructions: &HashMap<String, Action>, wire1: &str, wire2: &str) -> u16 {
    let part1 = part1(instructions, wire1);
    let mut memo = HashMap::new();
    memo.insert(wire2.to_string(), part1);
    get_signal(instructions, &wire1.to_string(), &mut memo)
}

fn get_signal(instructions: &HashMap<String, Action>, wire: &String, memo: &mut HashMap<String, u16>) -> u16 {
    match memo.get(wire) {
        Some(n) => return *n,
        None => ()
    }
    let current = instructions.get(wire).unwrap();
    let result = match current {
        Action::Set(n) => *n,
        Action::Move(op) => get_signal(instructions, op, memo),
        Action::Not(op) => !get_signal(instructions, op, memo),
        Action::And(op1, op2) => get_signal(instructions, op1, memo) & get_signal(instructions, op2, memo),
        Action::Or(op1, op2) => get_signal(instructions, op1, memo) | get_signal(instructions, op2, memo),
        Action::AndN(n, op) => n & get_signal(instructions, op, memo),
        Action::LShift(op, n) => get_signal(instructions, op, memo) << n,
        Action::RShift(op, n) => get_signal(instructions, op, memo) >> n
    };
    memo.insert(wire.to_string(), result);
    result
}