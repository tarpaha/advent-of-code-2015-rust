use std::collections::HashSet;
use itertools::Itertools;

pub fn part1(data: &str) -> usize {
    let mut houses = HashSet::new();
    let (mut x, mut y) = (0, 0);
    houses.insert((x, y));
    for ch in data.chars() {
        let (dx, dy) = get_direction(ch);
        x += dx; y += dy;
        houses.insert((x, y));
    }
    houses.len()
}

pub fn part2(data: &str) -> usize {
    let mut houses = HashSet::new();
    let (mut x1, mut y1) = (0, 0);
    let (mut x2, mut y2) = (0, 0);
    houses.insert((x1, y1));
    houses.insert((x2, y2));
    for (ch1, ch2) in data.chars().tuples() {
        let (dx1, dy1) = get_direction(ch1);
        let (dx2, dy2) = get_direction(ch2);
        x1 += dx1; y1 += dy1;
        x2 += dx2; y2 += dy2;
        houses.insert((x1, y1));
        houses.insert((x2, y2));
    }
    houses.len()
}

fn get_direction(ch: char) -> (i32, i32) {
    match ch {
        '^' => ( 0, -1),
        'v' => ( 0,  1),
        '<' => (-1,  0),
        '>' => ( 1,  0),
        _ => panic!()
    }
}

#[test]
fn test_part1() {
    assert_eq!(part1(">"), 2);
    assert_eq!(part1("^>v<"), 4);
    assert_eq!(part1("^v^v^v^v^v"), 2);
}

#[test]
fn test_part2() {
    assert_eq!(part2("^v"), 3);
    assert_eq!(part2("^>v<"), 3);
    assert_eq!(part2("^v^v^v^v^v"), 11);
}