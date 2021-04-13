use std::collections::HashMap;

pub fn part1(total: u32, values: &Vec<u32>) -> u32 {
    let mut count = 0;
    for bitmask in 0..2u32.pow(values.len() as u32) {
        let mut sum = 0;
        for (id, value) in values.iter().enumerate() {
            if bitmask & (1 << id) != 0 {
                sum += value;
            }
        }
        if sum == total {
            count += 1;
        }
    }
    count
}

pub fn part2(total: u32, values: &Vec<u32>) -> u32 {
    let mut count = 0;
    let mut answers = HashMap::new();
    for bitmask in 0..2u32.pow(values.len() as u32) {
        let mut sum = 0;
        let mut length = 0;
        for (id, value) in values.iter().enumerate() {
            if bitmask & (1 << id) != 0 {
                sum += value;
                length += 1;
            }
        }
        if sum == total {
            count += 1;
            let answer = answers.entry(length).or_insert(0);
            *answer += 1;
        }
    }
    *answers.get(answers.keys().min().unwrap()).unwrap()
}

#[test]
fn part1_test() {
    assert_eq!(part1(25, &vec![20, 15, 10, 5, 5]), 4);
}

#[test]
fn part2_test() {
    assert_eq!(part2(25, &vec![20, 15, 10, 5, 5]), 3);
}