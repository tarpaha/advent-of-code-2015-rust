pub fn parse(str: &str) -> Vec<(u32, u32, u32)> {
    str.lines()
        .map(|line| parse_dimensions(&line))
        .collect()
}

fn parse_dimensions(s: &str) -> (u32, u32, u32) {
    let dims: Vec<u32> = s
        .split('x')
        .map(|s| s.parse().unwrap())
        .collect();
    (dims[0], dims[1], dims[2])
}

#[test]
fn test_parse_dimensions() {
    assert_eq!(parse_dimensions("2x3x4"), (2, 3, 4));
    assert_eq!(parse_dimensions("1x1x10"), (1, 1, 10));
}