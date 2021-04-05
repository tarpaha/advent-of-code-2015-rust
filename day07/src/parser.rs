use std::collections::HashMap;

pub fn parse(data: &str) -> HashMap<String, Action> {
    let mut hashmap = HashMap::new();
    for line in data.lines() {
        let (action, out) = parse_line(line);
        hashmap.insert(out, action);
    }
    hashmap
}
#[derive(Debug, PartialEq)]
pub enum Action {
    Set(u16),
    Move(String),
    And(String, String),
    AndN(u16, String),
    Or(String, String),
    LShift(String, u16),
    RShift(String, u16),
    Not(String)
}

fn parse_line(s: &str) -> (Action, String) {
    let parts: Vec<&str> = s.split(' ').collect();
    let action = match parts.len() {
        3 => match parts[0].parse::<u16>() {
            Ok(n) => Action::Set(n),
            Err(_) => Action::Move(parts[0].to_string())
        }
        4 => Action::Not(parts[1].to_string()),
        5 => match parts[1] {
            "AND"    => match parts[0].parse::<u16>() {
                Ok(n) => Action::AndN(n, parts[2].to_string()),
                Err(_) => Action::And(parts[0].to_string(), parts[2].to_string()),
            }
            "OR"     => Action::Or(parts[0].to_string(), parts[2].to_string()),
            "LSHIFT" => Action::LShift(parts[0].to_string(), parts[2].parse::<u16>().unwrap()),
            "RSHIFT" => Action::RShift(parts[0].to_string(), parts[2].parse::<u16>().unwrap()),
            _ => panic!()
        },
        _ => panic!()
    };
    (action, parts[parts.len() - 1].to_string())
}

#[test]
fn instruction_from_str_test() {
    assert_eq!(parse_line("123 -> x"), (Action::Set(123), "x".to_string()));
    assert_eq!(parse_line("a -> x"), (Action::Move("a".to_string()), "x".to_string()));
    assert_eq!(parse_line("NOT x -> h"), (Action::Not("x".to_string()), "h".to_string()));
    assert_eq!(parse_line("x AND y -> d"), (Action::And("x".to_string(), "y".to_string()), "d".to_string()));
    assert_eq!(parse_line("1 AND y -> d"), (Action::AndN(1, "y".to_string()), "d".to_string()));
    assert_eq!(parse_line("x OR y -> e"), (Action::Or("x".to_string(), "y".to_string()), "e".to_string()));
    assert_eq!(parse_line("x LSHIFT 2 -> f"), (Action::LShift("x".to_string(), 2), "f".to_string()));
    assert_eq!(parse_line("y RSHIFT 3 -> g"), (Action::RShift("y".to_string(), 3), "g".to_string()));
}