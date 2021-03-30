use utils::Solution;

mod parser;
mod solver;

pub struct Day02;
impl Solution for Day02 {
    fn solve(&self) -> (String, String) {
        let data = include_str!("./input");
        let dimensions = parser::parse(data);
        (
            solver::part1(&dimensions).to_string(),
            solver::part2(&dimensions).to_string()
        )
    }
}