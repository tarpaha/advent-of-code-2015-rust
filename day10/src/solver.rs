pub fn part1(str: &str) -> u32 {
    convert_times(str, 40).len() as u32
}

pub fn part2(str: &str) -> u32 {
    convert_times(str, 50).len() as u32
}

fn convert_times(str: &str, times: u32) -> String {
    let mut s = str.to_string();
    for _ in 0..times {
        s = convert(&s);
    }
    s
}

fn convert(str: &str) -> String {
    let mut str: Vec<char> = str.chars().collect();
    let mut current = str[0];
    let mut count = 1;
    let mut result = Vec::with_capacity(2 * str.len());
    str.push(0 as char);
    for id in 1..str.len() {
        let ch = str[id];
        if ch != current {
            result.push((b'0' + count) as char);
            result.push(current);
            current = ch;
            count = 1;
        }
        else {
            count += 1;
        }
    }
    result.into_iter().collect()
}

#[test]
fn convert_test() {
    assert_eq!(convert("1"), "11");
    assert_eq!(convert("11"), "21");
    assert_eq!(convert("21"), "1211");
    assert_eq!(convert("1211"), "111221");
    assert_eq!(convert("111221"), "312211");
}