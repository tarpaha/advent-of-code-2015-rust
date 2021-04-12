use std::collections::HashMap;

pub fn parse_aunts(data: &str) -> Vec<Vec<(&str, u8)>> {
    let mut result = vec![];
    for line in data.lines() {
        result.push(parse_aunt(line));
    }
    result
}

pub fn parse_tape(data: &str) -> HashMap<&str, u8> {
    let mut result = HashMap::new();
    for line in data.lines() {
        if line.len() == 0 {
            continue;
        }
        let parts: Vec<&str> = line.split(&[' ', ':'][..]).collect();
        result.insert(parts[0], parts[2].parse().unwrap());
    }
    result
}

fn parse_aunt(line: &str) -> Vec<(&str, u8)> {
    let mut result = vec![];
    let parts: Vec<&str> = line.split(&[' ', ':', ','][..]).collect();
    let mut id = 3;
    while id < parts.len() {
        result.push((parts[id], parts[id + 2].parse().unwrap()));
        id += 4;
    }
    result
}

#[test]
fn parse_aunt_test() {
    assert_eq!(
        parse_aunt("Sue 1: goldfish: 6, trees: 9, akitas: 0"),
        vec![("goldfish", 6), ("trees", 9), ("akitas", 0)]);
}

#[test]
fn parse_tape_test() {
    let tape = parse_tape(r#"
cats: 7
samoyeds: 2"#);
    assert_eq!(tape.len(), 2);
    assert_eq!(*tape.get("cats").unwrap(), 7);
    assert_eq!(*tape.get("samoyeds").unwrap(), 2);
}