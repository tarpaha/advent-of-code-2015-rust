use utils::Solution;
use day01::Day01;
use day02::Day02;
use day03::Day03;
use day04::Day04;
use day05::Day05;
use day06::Day06;
use day07::Day07;
use day08::Day08;
use day09::Day09;
use day10::Day10;
use day11::Day11;
use day12::Day12;
use day13::Day13;
use day14::Day14;
use day15::Day15;
use day16::Day16;
use day17::Day17;
use day18::Day18;

fn main() {
    let solutions: Vec<Box<dyn Solution>> = vec![
        Box::new(Day01),
        Box::new(Day02),
        Box::new(Day03),
        Box::new(Day04),
        Box::new(Day05),
        Box::new(Day06),
        Box::new(Day07),
        Box::new(Day08),
        Box::new(Day09),
        Box::new(Day10),
        Box::new(Day11),
        Box::new(Day12),
        Box::new(Day13),
        Box::new(Day14),
        Box::new(Day15),
        Box::new(Day16),
        Box::new(Day17),
        Box::new(Day18)
    ];
    for (day, solution) in solutions.iter().enumerate() {
        println!("{}", format!("Day{:02}:", day + 1));
        let (part1, part2) = solution.solve();
        println!("  Part1: {}", part1);
        println!("  Part2: {}", part2);
    }
}