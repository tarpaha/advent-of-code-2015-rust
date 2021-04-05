﻿use utils::Solution;
use day01::Day01;
use day02::Day02;
use day03::Day03;
use day04::Day04;
use day05::Day05;
use day06::Day06;
use day07::Day07;

fn main() {
    let solutions: Vec<Box<dyn Solution>> = vec![
        Box::new(Day01),
        Box::new(Day02),
        Box::new(Day03),
        Box::new(Day04),
        Box::new(Day05),
        Box::new(Day06),
        Box::new(Day07)
    ];
    for (day, solution) in solutions.iter().enumerate() {
        println!("{}", format!("Day{:02}:", day + 1));
        let (part1, part2) = solution.solve();
        println!("  Part1: {}", part1);
        println!("  Part2: {}", part2);
    }
}