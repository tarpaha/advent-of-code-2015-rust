use utils::Solution;

mod solver;
mod parser;

pub struct Day09;
impl Solution for Day09 {
    fn solve(&self) -> (String, String) {
        let data = include_str!("./input");
        let distances = parser::parse(data);
        (
            solver::part1(&distances).to_string(),
            solver::part2(&distances).to_string()
        )
    }
}