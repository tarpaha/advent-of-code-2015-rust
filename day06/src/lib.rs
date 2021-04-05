use utils::Solution;

pub mod parser;
pub mod solver;

pub struct Day06;
impl Solution for Day06 {
    fn solve(&self) -> (String, String) {
        let data = include_str!("./input");
        let instructions = parser::parse(data);
        (
            solver::part1(&instructions).to_string(),
            solver::part2(&instructions).to_string()
        )
    }
}