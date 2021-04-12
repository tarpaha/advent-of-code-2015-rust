use utils::Solution;

mod parser;
mod solver;

pub struct Day16;
impl Solution for Day16 {
    fn solve(&self) -> (String, String) {
        let data = include_str!("./input_aunts");
        let aunts = parser::parse_aunts(data);
        let tape = parser::parse_tape(include_str!("./input_tape"));
        (
            solver::part1(&tape, &aunts).to_string(),
            solver::part2(&tape, &aunts).to_string()
        )
    }
}