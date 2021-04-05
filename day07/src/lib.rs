use utils::Solution;

pub mod parser;
pub mod solver;

pub struct Day07;
impl Solution for Day07 {
    fn solve(&self) -> (String, String) {
        let data = include_str!("./input");
        let instructions = parser::parse(data);
        (
            solver::part1(&instructions, "a").to_string(),
            solver::part2(&instructions, "a", "b").to_string(),
        )
    }
}