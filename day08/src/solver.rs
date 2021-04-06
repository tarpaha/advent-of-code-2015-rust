pub fn part1(data: &Vec<&str>) -> u32 {
    data
        .iter()
        .map(|line| memory_length_diff(line))
        .sum()
}

pub fn part2(data: &Vec<&str>) -> u32 {
    data
        .iter()
        .map(|line| encode_length_diff(line))
        .sum()
}

fn memory_length_diff(str: &str) -> u32 {
    str.len() as u32 - in_memory_length(str)
}

fn in_memory_length(str: &str) -> u32 {
    let bytes = &str.as_bytes()[1..str.len()-1];
    let mut id = 0;
    let mut actual_bytes = 0;
    while id < bytes.len() {
        match bytes[id] {
            b'\\' => match bytes[id + 1] {
                b'x' => id += 3,
                _ => id += 1
            },
            _ => {}
        }
        id += 1;
        actual_bytes += 1
    }
    actual_bytes
}

fn encode_length_diff(str: &str) -> u32 {
    return encode_length(str) - str.len() as u32;
}

fn encode_length(str: &str) -> u32 {
    let mut length = 0;
    for ch in str.chars() {
        length += match ch {
            '"' => 2,
            '\\' => 2,
            _ => 1
        };
    }
    length + 2
}

#[test]
fn in_memory_length_test() {
    assert_eq!(in_memory_length(r#""\\""#), 1);
    assert_eq!(in_memory_length(r#""\"""#), 1);
    assert_eq!(in_memory_length(r#""\x27""#), 1);
}

#[test]
fn memory_length_diff_test() {
    assert_eq!(memory_length_diff(r#""""#), 2);
    assert_eq!(memory_length_diff(r#""aaa\"aaa""#), 3);
    assert_eq!(memory_length_diff(r#""\x27""#), 5);
}

#[test]
fn encode_length_test() {
    assert_eq!(encode_length(r#""""#), 6);
    assert_eq!(encode_length(r#""abc""#), 9);
    assert_eq!(encode_length(r#""aaa\"aaa""#), 16);
    assert_eq!(encode_length(r#""\x27""#), 11);
}

#[test]
fn encode_length_diff_test() {
    assert_eq!(encode_length_diff(r#""""#), 4);
    assert_eq!(encode_length_diff(r#""abc""#), 4);
    assert_eq!(encode_length_diff(r#""aaa\"aaa""#), 6);
    assert_eq!(encode_length_diff(r#""\x27""#), 5);

}