use std::collections::HashMap;

pub fn parse(data: &str) -> HashMap<(&str, &str), u32> {
    let mut hashmap = HashMap::new();
    for line in data.lines() {
        let (city1, city2, distance) = parse_line(line);
        hashmap.insert((city1, city2), distance);
        hashmap.insert((city2, city1), distance);
    }
    hashmap
}

fn parse_line(line: &str) -> (&str, &str, u32) {
    let parts: Vec<&str> = line.split(' ').collect();
    (parts[0], parts[2], parts[4].parse().unwrap())
}

#[test]
fn parse_line_test() {
    assert_eq!(parse_line("London to Dublin = 464"), ("London", "Dublin", 464));
}

#[test]
fn parse_test() {
    let hashmap = parse("London to Dublin = 464");
    assert_eq!(*hashmap.get(&("London", "Dublin")).unwrap(), 464);
    assert_eq!(*hashmap.get(&("Dublin", "London")).unwrap(), 464);
}