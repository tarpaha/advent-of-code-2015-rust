use utils::Solution;

pub mod solver;

pub struct Day04;
impl Solution for Day04 {
    fn solve(&self) -> (String, String) {
        let data = include_str!("./input");
        (
            solver::part1(data).to_string(),
            solver::part2(data).to_string()
        )
    }
}