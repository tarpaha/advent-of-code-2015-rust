pub fn part1(field: &Vec<Vec<bool>>, steps: u32) -> u32 {
    let mut field = field.clone();
    for _ in 0..steps {
        field = step(&field);
    }
    field.iter().flatten().filter(|v| **v).count() as u32
}

pub fn part2(field: &Vec<Vec<bool>>, steps: u32) -> u32 {
    let mut field = field.clone();
    turn_on_corner_lights(&mut field);
    for _ in 0..steps {
        field = step(&field);
        turn_on_corner_lights(&mut field);
    }
    field.iter().flatten().filter(|v| **v).count() as u32
}

fn turn_on_corner_lights(field: &mut Vec<Vec<bool>>) {
    let border = field.len() - 1;
    field[0][0] = true;
    field[border][0] = true;
    field[0][border] = true;
    field[border][border] = true;
}

fn step(field: &Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    let mut result = field.clone();
    for y in 0..field.len() {
        for x in 0..field.len() {
            let neighbors = enabled_neighbors(field, x, y);
            result[y][x] = if result[y][x] {
                match neighbors {
                    2 | 3 => true,
                    _ => false
                }
            } else {
                match neighbors {
                    3 => true,
                    _ => false
                }
            }
        }
    }
    result
}

fn enabled_neighbors(field: &Vec<Vec<bool>>, x: usize, y: usize) -> u32 {
    let mut result = 0;
    let size = field.len() as isize;
    for dy in -1..=1 {
        let py = y as isize + dy;
        if py < 0 || py >= size {
            continue;
        }
        for dx in -1..=1 {
            if dy == 0 && dx == 0 {
                continue;
            }
            let px = x as isize + dx;
            if px < 0 || px >= size {
                continue;
            }
            if field[py as usize][px as usize] {
                result += 1;
            }
        }
    }
    result
}

#[test]
fn enabled_neighbors_test() {
    use crate::parser;
    let field = parser::parse(r#"
.#.#.#
...##.
#....#
..#...
#.#..#
####.."#);
    assert_eq!(enabled_neighbors(&field, 0, 0), 1);
    assert_eq!(enabled_neighbors(&field, 1, 0), 0);
    assert_eq!(enabled_neighbors(&field, 2, 0), 3);
    assert_eq!(enabled_neighbors(&field, 3, 0), 2);
    assert_eq!(enabled_neighbors(&field, 4, 0), 4);
    assert_eq!(enabled_neighbors(&field, 5, 0), 1);
    assert_eq!(enabled_neighbors(&field, 0, 5), 2);
    assert_eq!(enabled_neighbors(&field, 5, 5), 1);
}

#[test]
fn step_test() {
    use crate::parser;
    let initial = parser::parse(r#"
.#.#.#
...##.
#....#
..#...
#.#..#
####.."#);
    
    let after_1 = parser::parse(r#"
..##..
..##.#
...##.
......
#.....
#.##.."#);
    assert_eq!(step(&initial), after_1);
    
    let after_2 = parser::parse(r#"
..###.
......
..###.
......
.#....
.#...."#);
    assert_eq!(step(&after_1), after_2);

    let after_3 = parser::parse(r#"
...#..
......
...#..
..##..
......
......"#);
    assert_eq!(step(&after_2), after_3);

    let after_4 = parser::parse(r#"
......
......
..##..
..##..
......
......"#);
    assert_eq!(step(&after_3), after_4);
}

#[test]
fn part1_test() {
    use crate::parser;
    let field = parser::parse(r#"
.#.#.#
...##.
#....#
..#...
#.#..#
####.."#);
    assert_eq!(part1(&field, 4), 4);
}

#[test]
fn part2_test() {
    use crate::parser;
    let field = parser::parse(r#"
.#.#.#
...##.
#....#
..#...
#.#..#
####.."#);
    assert_eq!(part2(&field, 5), 17);
}