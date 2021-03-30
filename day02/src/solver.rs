use std::cmp;

pub fn part1(dimensions: &Vec<(u32, u32, u32)>) -> u32 {
    dimensions
        .iter()
        .fold(0, |sum, (w, h, l)| sum + calculate_area(*w, *h, *l))
}

pub fn part2(dimensions: &Vec<(u32, u32, u32)>) -> u32 {
    dimensions
        .iter()
        .fold(0, |sum, (w, h, l)| sum + calculate_ribbon_length(*w, *h, *l))
}

fn calculate_area(w: u32, h: u32, l: u32) -> u32 {
    let lw = l * w;
    let wh = w * h;
    let hl = h * l;
    let min_area = cmp::min(lw, cmp::min(wh, hl));
    2*lw + 2*wh + 2*hl + min_area
}

fn calculate_ribbon_length(w: u32, h: u32, l: u32) -> u32 {
    let lw = l + w;
    let wh = w + h;
    let hl = h + l;
    let min_half_perimeter = cmp::min(lw, cmp::min(wh, hl));
    2 * min_half_perimeter + w * h * l
}

#[test]
fn test_calculate_area() {
    assert_eq!(calculate_area(2, 3, 4), 58);
    assert_eq!(calculate_area(1, 1, 10), 43);
}

#[test]
fn test_calculate_ribbon_length() {
    assert_eq!(calculate_ribbon_length(2, 3, 4), 34);
    assert_eq!(calculate_ribbon_length(1, 1, 10), 14);
}