use md5::{Md5, Digest};
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;

pub fn part1(key: &'static str) -> u32 {
    find(key, five_leading_zeroes)
}

pub fn part2(key: &'static str) -> u32 {
    find(key, six_leading_zeroes)
}

fn find(key: &'static str, acceptable: fn(&[u8]) -> bool) -> u32 {
    let chunk_size = 50000;

    let semaphore = Arc::new(Mutex::new(num_cpus::get()));
    let result = Arc::new(AtomicUsize::new(0));

    for id in 0.. {
        // wait for free threads (also break when have result)
        while result.load(Ordering::SeqCst) == 0 {
            let mut count = semaphore.lock().unwrap();
            if *count > 0
            {
                // "acquire" semaphore
                *count -= 1;
                drop(count);
                break;
            }
            drop(count);
        }

        // do not start new thread if already have result
        if result.load(Ordering::SeqCst) != 0 {
            break;
        }

        // new calculation thread
        let semaphore = Arc::clone(&semaphore);
        let result = Arc::clone(&result);
        thread::spawn(move || {
            let start = id * chunk_size;
            let mut hasher = Md5::new();
            for n in start .. start + chunk_size {
                hasher.update(key.as_bytes());
                hasher.update(n.to_string().as_bytes());
                let md5 = hasher.finalize_reset();
                if acceptable(&md5) {
                    result.store(n, Ordering::SeqCst);
                    break;
                }
            }
            let mut count = semaphore.lock().unwrap();
            // "release" semaphore
            *count += 1;
            drop(count);
        });
    }

    result.load(Ordering::SeqCst) as u32
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