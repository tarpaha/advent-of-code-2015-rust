pub fn part1(str: &str) -> i32 {
    let mut floor = 0;
    for ch in str.chars() {
        match ch {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => ()
        }
    }
    floor
}

pub(crate) fn part2(str: &str) -> i32 {
    let mut floor = 0;
    let mut position = 1;
    for ch in str.chars() {
        match ch {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => ()
        }
        if floor < 0 {
            break;
        }
        position += 1;
    }
    position
}

#[test]
fn test_part1() {
    assert_eq!(part1("(())"), 0);
    assert_eq!(part1("()()"), 0);
    assert_eq!(part1("((("), 3);
    assert_eq!(part1("(()(()("), 3);
    assert_eq!(part1("))((((("), 3);
    assert_eq!(part1("())"), -1);
    assert_eq!(part1("))("), -1);
    assert_eq!(part1(")))"), -3);
    assert_eq!(part1(")())())"), -3);
}

#[test]
fn test_part2() {
    assert_eq!(part2(")"), 1);
    assert_eq!(part2("()())"), 5);
}