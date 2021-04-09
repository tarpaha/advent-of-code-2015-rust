use std::collections::HashMap;
use itertools::Itertools;

pub fn part1(relationship: &HashMap<(&str, &str), i32>) -> i32 {
    let persons: Vec<&str> = relationship
        .keys()
        .map(|pair| pair.0)
        .unique()
        .collect();
    let mut max = i32::MIN;
    for seated in persons.iter().permutations(persons.len()) {
        let mut happiness = 0;
        for i in 1..seated.len() {
            happiness += relationship.get(&(*seated[i - 1], *seated[i])).unwrap();
            happiness += relationship.get(&(*seated[i], *seated[i - 1])).unwrap();
        }
        happiness += relationship.get(&(*seated[seated.len() - 1], *seated[0])).unwrap();
        happiness += relationship.get(&(*seated[0], *seated[seated.len() - 1])).unwrap();
        if happiness > max {
            max = happiness;
        }
    }
    max
}

pub fn part2(relationship: &HashMap<(&str, &str), i32>) -> i32 {
    let mut relationship = relationship.clone();
    let persons: Vec<&str> = relationship
        .keys()
        .map(|pair| pair.0)
        .unique()
        .collect();
    for person in persons {
        relationship.insert(("Me", person), 0);
        relationship.insert((person, "Me"), 0);
    }
    part1(&relationship)
}