use utils::Solution;

mod parser;
mod solver;

pub struct Day13;
impl Solution for Day13 {
    fn solve(&self) -> (String, String) {
        let data = include_str!("./input");
        let relationship = parser::parse(data);
        (
            solver::part1(&relationship).to_string(),
            solver::part2(&relationship).to_string()
        )
    }
}