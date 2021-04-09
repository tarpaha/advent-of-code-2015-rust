use utils::Solution;

mod parser;
mod solver;

pub struct Day14;
impl Solution for Day14 {
    fn solve(&self) -> (String, String) {
        let data = include_str!("./input");
        let reindeers = parser::parse(data);
        (
            solver::part1(&reindeers, 2503).to_string(),
            solver::part2(&reindeers, 2503).to_string()
        )
    }
}