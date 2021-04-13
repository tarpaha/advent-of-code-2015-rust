use utils::Solution;

mod parser;
mod solver;

pub struct Day18;
impl Solution for Day18 {
    fn solve(&self) -> (String, String) {
        let data = include_str!("./input");
        let field = parser::parse(data);
        (
            solver::part1(&field, 100).to_string(),
            solver::part2(&field, 100).to_string()
        )
    }
}