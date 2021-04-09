use utils::Solution;

mod parser;
mod solver;

pub struct Day15;
impl Solution for Day15 {
    fn solve(&self) -> (String, String) {
        let data = include_str!("./input");
        let _ingredients = parser::parse(data);
        (
            solver::part1(&_ingredients, 100).to_string(),
            solver::part2(&_ingredients, 100, 500).to_string()
        )
    }
}