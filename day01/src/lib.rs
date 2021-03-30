use utils::Solution;

mod solver;

pub struct Day01;
impl Solution for Day01 {
    fn solve(&self) -> (String, String) {
        let data = include_str!("./input");
        (
            solver::part1(data).to_string(),
            solver::part2(data).to_string()
        )
    }
}