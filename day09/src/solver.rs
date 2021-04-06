use std::collections::HashMap;
use itertools::Itertools;

pub fn part1(distances: &HashMap<(&str, &str), u32>) -> u32 {
    minmax(distances).0
}

pub fn part2(distances: &HashMap<(&str, &str), u32>) -> u32 {
    minmax(distances).1
}

pub fn minmax(distances: &HashMap<(&str, &str), u32>) -> (u32, u32) {
    let cities: Vec<&str> = distances
        .keys()
        .map(|pair| pair.0)
        .unique()
        .collect();
    let mut min = u32::MAX;
    let mut max = u32::MIN;
    for path in cities.iter().permutations(cities.len()) {
        let mut distance = 0;
        for i in 1..path.len() {
            distance += distances.get(&(*path[i-1], *path[i])).unwrap();
        }
        if distance < min {
            min = distance;
        }
        if distance > max {
            max = distance
        }
    }
    (min, max)
}