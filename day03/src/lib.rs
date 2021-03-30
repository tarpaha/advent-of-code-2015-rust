use utils::Solution;

pub mod solver;

pub struct Day03;
impl Solution for Day03 {
    fn solve(&self) -> (String, String) {
        let data = include_str!("./input");
        (
            solver::part1(data).to_string(),
            solver::part2(data).to_string()
        )
    }
}