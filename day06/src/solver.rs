use crate::parser::{Instruction, Action};

pub fn part1(instructions: &Vec<Instruction>) -> u32 {
    let mut lights = vec![false; 1000000];
    for instruction in instructions {
        let action = &instruction.action;
        for y in instruction.y1 .. instruction.y2 + 1 {
            let offset = y * 1000;
            for x in instruction.x1 .. instruction.x2 + 1 {
                match action {
                    Action::TurnOn  => lights[offset + x] = true,
                    Action::TurnOff => lights[offset + x] = false,
                    Action::Toggle  => lights[offset + x] = !lights[offset + x]
                }
            }
        }
    }
    lights.iter().filter(|light| **light).count() as u32
}

pub fn part2(instructions: &Vec<Instruction>) -> u32 {
    let mut lights = vec![0u32; 1000000];
    for instruction in instructions {
        let action = &instruction.action;
        for y in instruction.y1 .. instruction.y2 + 1 {
            let offset = y * 1000;
            for x in instruction.x1 .. instruction.x2 + 1 {
                let br = &mut lights[offset + x];
                *br = match action {
                    Action::TurnOn  => *br + 1,
                    Action::TurnOff => if *br > 0 { *br - 1 } else { *br },
                    Action::Toggle  => *br + 2
                }
            }
        }
    }
    lights.iter().sum()
}

#[test]
pub fn part1_test() {
    use crate::parser;
    let instructions = parser::parse("turn on 0,0 through 999,999");
    assert_eq!(part1(&instructions), 1000000);
    let instructions = parser::parse("toggle 0,0 through 999,0");
    assert_eq!(part1(&instructions), 1000);
    let instructions = parser::parse("turn on 0,0 through 999,999\nturn off 499,499 through 500,500");
    assert_eq!(part1(&instructions), 1000000 - 4);
}

#[test]
pub fn part2_test() {
    use crate::parser;
    let instructions = parser::parse("turn on 0,0 through 0,0");
    assert_eq!(part2(&instructions), 1);
    let instructions = parser::parse("toggle 0,0 through 999,999");
    assert_eq!(part2(&instructions), 2000000);
}