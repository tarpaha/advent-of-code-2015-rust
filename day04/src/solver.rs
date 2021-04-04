use md5::{Md5, Digest};
use std::sync::{Arc, Mutex};
use std::thread;

pub fn part1(key: &'static str) -> u32 {
    find(key, five_leading_zeroes)
}

pub fn part2(key: &'static str) -> u32 {
    find(key, six_leading_zeroes)
}

fn find(key: &'static str, acceptable: fn(&[u8]) -> bool) -> u32 {
    let chunk_size = 50000;
    let num_cpus = num_cpus::get();
    let result = Arc::new(Mutex::new(Vec::new()));
    let mut id = 0;
    loop {
        let mut handles = vec![];
        for i in id .. id + num_cpus {
            let start = i * chunk_size;
            let result = Arc::clone(&result);
            let handle = thread::spawn(move || {
                let mut hasher = Md5::new();
                for n in start .. start + chunk_size {
                    hasher.update(key.as_bytes());
                    hasher.update(n.to_string().as_bytes());
                    let md5 = hasher.finalize_reset();
                    if acceptable(&md5) {
                        result.lock().unwrap().push(n);
                        break;
                    }
                }
            });
            handles.push(handle);
        }
        for handle in handles {
            handle.join().unwrap();
        }
        if result.lock().unwrap().len() > 0 {
            break;
        }
        id += num_cpus;
    }

    let result = *match result.lock().unwrap().iter().min() {
        Some(min) => min,
        None      => panic!(),
    };
    result as u32
}

// first straight realization
fn _find_straight(key: &str, acceptable: fn(&[u8]) -> bool) -> u32 {
    let mut hasher = Md5::new();
    let mut n = 1;
    loop {
        hasher.update(key.as_bytes());
        hasher.update(n.to_string().as_bytes());
        let md5 = hasher.finalize_reset();
        if acceptable(&md5) {
            break;
        }
        n += 1;
    }
    n
}

fn five_leading_zeroes(data: &[u8]) -> bool {
    data[0] == 0 && data[1] == 0 && data[2] < 0x10
}

fn six_leading_zeroes(data: &[u8]) -> bool {
    data[0] == 0 && data[1] == 0 && data[2] == 0
}

#[test]
fn test_part1() {
    assert_eq!(part1("abcdef"), 609043);
    assert_eq!(part1("pqrstuv"), 1048970);
}

#[test]
fn test_five_leading_zeroes() {
    assert_eq!(five_leading_zeroes(&[0u8, 0, 0]), true);
    assert_eq!(five_leading_zeroes(&[0u8, 0, 1]), true);
    assert_eq!(five_leading_zeroes(&[0u8, 0, 15]), true);
    assert_eq!(five_leading_zeroes(&[0u8, 0, 16]), false);
}

#[test]
fn test_six_leading_zeroes() {
    assert_eq!(six_leading_zeroes(&[0u8, 0, 0]), true);
    assert_eq!(six_leading_zeroes(&[0u8, 0, 1]), false);
}

#[test]
fn test_md5() {
    assert_eq!(format!("{:x}", Md5::digest(b"abcdefghijklmnopqrstuvwxyz")), "c3fcd3d76192e4007dfb496cca67e13b");
    assert_eq!(format!("{:x}", Md5::digest(b"The quick brown fox jumps over the lazy dog")), "9e107d9d372bb6826bd81d3542a419d6");
}