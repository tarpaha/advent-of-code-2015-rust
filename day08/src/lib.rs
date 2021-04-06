use utils::Solution;

pub mod solver;

pub struct Day08;
impl Solution for Day08 {
    fn solve(&self) -> (String, String) {
        let data = include_str!("./input").lines().collect();
        (
            solver::part1(&data).to_string(),
            solver::part2(&data).to_string()
        )
    }
}