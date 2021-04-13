use utils::Solution;

mod solver;

pub struct Day17;
impl Solution for Day17 {
    fn solve(&self) -> (String, String) {
        let values = include_str!("./input")
            .lines()
            .map(|line| line.parse().unwrap())
            .collect();
        (
            solver::part1(150, &values).to_string(),
            solver::part2(150, &values).to_string()
        )
    }
}