pub fn part1(str: &str) -> String {
    let mut bytes: Vec<u8> = str.bytes().collect();
    while !acceptable(&bytes) {
        take_next(&mut bytes);
    }
    String::from_utf8(bytes).unwrap()
}

pub fn part2(str: &str) -> String {
    let mut bytes: Vec<u8> = part1(str).bytes().collect();
    take_next(&mut bytes);
    while !acceptable(&bytes) {
        take_next(&mut bytes);
    }
    String::from_utf8(bytes).unwrap()
}

fn acceptable(bytes: &Vec<u8>) -> bool {
    let mut increasing_count = 0;
    let mut first_dup = 0;
    let mut second_dup = 0;
    let mut prev = 0;
    
    for &current in bytes {
        
        match current {
            b'i' | b'o' | b'l' => return false,
            _ => ()
        }
        
        if increasing_count < 2 {
            if prev == current - 1 {
                increasing_count += 1;
            }
            else {
                increasing_count = 0;
            }
        }
        
        if prev == current {
            if first_dup == 0 {
                first_dup = current;
            } else {
                if second_dup == 0 && current != first_dup {
                    second_dup = current;
                }
            }
        }
        
        prev = current;
    }
    increasing_count >= 2 && first_dup != 0 && second_dup != 0
}

fn take_next(bytes: &mut Vec<u8>) {
    let mut pos = bytes.len() - 1;
    bytes[pos] += 1;
    while bytes[pos] > b'z' {
        bytes[pos] = b'a';
        bytes[pos - 1] += 1;
        pos -= 1;
    }
}

#[test]
fn take_next_test() {
    fn take_next_str(str: &str) -> String {
        let mut bytes: Vec<u8> = str.bytes().collect();
        take_next(&mut bytes);
        String::from_utf8(bytes).unwrap()
    }
    assert_eq!(take_next_str("aaaaaaaa"), "aaaaaaab");
    assert_eq!(take_next_str("azzzzzzz"), "baaaaaaa");
}

#[test]
fn acceptable_test() {
    fn acceptable_str(str: &str) -> bool {
        let mut bytes: Vec<u8> = str.bytes().collect();
        acceptable(&mut bytes)
    }
    assert_eq!(acceptable_str("hijklmmn"), false);
    assert_eq!(acceptable_str("abbceffg"), false);
    assert_eq!(acceptable_str("abbcegjk"), false);
    assert_eq!(acceptable_str("abcdefgh"), false);
    assert_eq!(acceptable_str("abcdffaa"), true);
    assert_eq!(acceptable_str("ghijklmn"), false);
    assert_eq!(acceptable_str("ghjaabcc"), true);
}

#[test]
fn part1_test() {
    assert_eq!(part1("abcdefgh"), "abcdffaa");
    assert_eq!(part1("ghijklmn"), "ghjaabcc");
}