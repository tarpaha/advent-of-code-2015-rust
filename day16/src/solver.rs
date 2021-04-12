use std::collections::HashMap;

pub fn part1(tape: &HashMap<&str, u8>, aunts: &Vec<Vec<(&str, u8)>>) -> usize {
    for (id, aunt) in aunts.iter().enumerate() {
        let mut success_count = 0;
        for (item, count) in aunt {
            match tape.get(item) {
                Some(required) =>
                    if count == required {
                        success_count += 1
                    },
                None => break
            }
        }
        if success_count == aunt.len() {
            return id + 1;
        }
    }
    panic!()
}

pub fn part2(tape: &HashMap<&str, u8>, aunts: &Vec<Vec<(&str, u8)>>) -> usize {
    for (id, aunt) in aunts.iter().enumerate() {
        let mut success_count = 0;
        for (item, count) in aunt {
            match tape.get(item) {
                Some(required) =>
                    match *item {
                        "cats" | "trees" => if count > required {
                            success_count += 1;
                        },
                        "pomeranians" | "goldfish" => if count < required {
                            success_count += 1;
                        },                        
                        _ => if count == required {
                            success_count += 1;
                        }
                    },
                None => break
            }
        }
        if success_count == aunt.len() {
            return id + 1;
        }
    }
    panic!()
}