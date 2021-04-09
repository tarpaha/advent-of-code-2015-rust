use std::collections::HashMap;

pub fn parse(data: &str) -> HashMap<(&str, &str), i32>{
    let mut hashmap = HashMap::new();
    for line in data.lines() {
        let (name1, name2, happiness) = parse_line(line);
        hashmap.insert((name1, name2), happiness);
    }
    hashmap
}

fn parse_line(line: &str) -> (&str, &str, i32) {
    let line = &line[0 .. line.len()-1];
    let parts: Vec<&str> = line.split(' ').collect();
    let mut happiness = parts[3].parse::<i32>().unwrap();
    if parts[2] == "lose" {
        happiness = -happiness;
    }
    (parts[0], parts[10], happiness)
}

#[test]
fn parse_line_test() {
    assert_eq!(parse_line("Alice would lose 2 happiness units by sitting next to David."), ("Alice", "David", -2));
    assert_eq!(parse_line("David would gain 46 happiness units by sitting next to Alice."), ("David", "Alice", 46));
}