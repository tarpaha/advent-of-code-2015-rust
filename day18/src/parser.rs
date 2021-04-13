pub fn parse(data: &str) -> Vec<Vec<bool>> {
    let lines: Vec<&str> = data.lines().filter(|line| line.len() > 0).collect();
    let mut result = vec![vec![false; lines.len()]; lines.len()];
    for (row_id, line) in lines.iter().enumerate() {
        let row = &mut result[row_id];
        for (column_id, ch) in line.chars().enumerate() {
            row[column_id] = match ch {
                '.' => false,
                '#' => true,
                _ => panic!()
            };
        }
    }
    result
}

#[test]
fn parser_test() {
    let field = parse(r#"
.#.#.#
...##.
#....#
..#...
#.#..#
####.."#);
    assert_eq!(field.len(), 6);
    assert_eq!(field.iter().all(|row| row.len() == 6), true);
}