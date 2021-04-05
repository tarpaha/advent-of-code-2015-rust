use std::str::FromStr;

pub fn parse(data: &str) -> Vec<Instruction> {
    data
        .lines()
        .map(|line| Instruction::from_str(line).unwrap())
        .collect()
}

#[derive(Debug, PartialEq)]
pub enum Action {
    TurnOn,
    TurnOff,
    Toggle
}

#[derive(Debug, PartialEq)]
pub struct Instruction {
    pub x1: usize, pub y1: usize,
    pub x2: usize, pub y2: usize,
    pub action: Action
}

impl FromStr for Instruction {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(&[' ', ','][..]).collect();
        let action =
            if parts[0] == "turn" { if parts[1] == "on" { Action::TurnOn } else { Action::TurnOff } }
            else { Action::Toggle };
        let shift = match action {
            Action::Toggle => 0,
            Action::TurnOff => 1,
            Action::TurnOn  => 1
        };
        let x1 = parts[shift + 1].parse().unwrap();
        let y1 = parts[shift + 2].parse().unwrap();
        let x2 = parts[shift + 4].parse().unwrap();
        let y2 = parts[shift + 5].parse().unwrap();
        Ok(Instruction{ x1, y1, x2, y2, action })
    }
}

#[test]
fn instruction_from_str_test() {
    let instruction = Instruction::from_str("turn on 0,1 through 2,3").unwrap();
    assert_eq!(instruction, Instruction { x1: 0, y1: 1, x2: 2, y2: 3, action: Action::TurnOn });
    let instruction = Instruction::from_str("turn off 4,5 through 6,7").unwrap();
    assert_eq!(instruction, Instruction { x1: 4, y1: 5, x2: 6, y2: 7, action: Action::TurnOff });
    let instruction = Instruction::from_str("toggle 8,9 through 10,11").unwrap();
    assert_eq!(instruction, Instruction { x1: 8, y1: 9, x2: 10, y2: 11, action: Action::Toggle });
}