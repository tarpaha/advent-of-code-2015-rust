use std::collections::HashMap;

pub fn part1(data: &Vec<&str>) -> u32 {
    data.iter().filter(|s| string_is_nice_part1(s)).count() as u32
}

pub fn part2(data: &Vec<&str>) -> u32 {
    data.iter().filter(|s| string_is_nice_part2(s)).count() as u32
}

fn string_is_nice_part1(str: &str) -> bool {
    fn is_vowel(ch: char) -> bool {
        match ch {
            'a' => true,
            'e' => true,
            'i' => true,
            'o' => true,
            'u' => true,
            _ => false
        }
    }
    fn bad_sequence(ch1: char, ch2: char) -> bool {
        let ch1_is_bad = match ch1 {
            'a' => true,
            'c' => true,
            'p' => true,
            'x' => true,
            _ => false
        };
        ch1_is_bad && ch2 as u32 == ch1 as u32 + 1
    }
    let mut vowels_count = 0;
    let mut have_double = false;
    let mut prev: char = 0 as char;
    for ch in str.chars() {
        if is_vowel(ch) {
            vowels_count += 1;
        }
        if ch == prev {
            have_double = true;
        }
        if bad_sequence(prev, ch) {
            return false;
        }
        prev = ch;
    }
    vowels_count >= 3 && have_double
}

pub fn string_is_nice_part2(str: &str) -> bool {
    let mut doubles = HashMap::new();
    let mut ignore_next_same = false;
    let mut one_with_between = false;
    let mut prevprev: char = 0 as char;
    let mut prev: char = 0 as char;
    for ch in str.chars() {
        if prev == ch {
            if ignore_next_same {
                ignore_next_same = false;
            }
            else
            {
                *doubles.entry((prev, ch)).or_insert(0) += 1;
                ignore_next_same = true;
            }
        }
        else {
            *doubles.entry((prev, ch)).or_insert(0) += 1;
            ignore_next_same = false;
        }
        if !one_with_between && prevprev == ch {
            one_with_between = true;
        }
        prevprev = prev;
        prev = ch;
    }
    one_with_between && doubles.values().any(|&v| v >= 2)
}

#[test]
fn string_is_nice_part1_test() {
    assert_eq!(string_is_nice_part1("ugknbfddgicrmopn"), true);
    assert_eq!(string_is_nice_part1("aaa"), true);
    assert_eq!(string_is_nice_part1("jchzalrnumimnmhp"), false);
    assert_eq!(string_is_nice_part1("haegwjzuvuyypxyu"), false);
    assert_eq!(string_is_nice_part1("dvszwmarrgswjxmb"), false);
}

#[test]
fn string_is_nice_part2_test() {
    assert_eq!(string_is_nice_part2("xyxy"), true);
    assert_eq!(string_is_nice_part2("aaa"), false);
    assert_eq!(string_is_nice_part2("qjhvhtzxzqqjkmpb"), true);
    assert_eq!(string_is_nice_part2("xxyxx"), true);
    assert_eq!(string_is_nice_part2("uurcxstgmygtbstg"), false);
    assert_eq!(string_is_nice_part2("ieodomkazucvgmuy"), false);
}