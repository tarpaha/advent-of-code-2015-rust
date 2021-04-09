use crate::parser::Reindeer;

pub fn part1(reindeers: &Vec<Reindeer>, time: u32) -> u32 {
    reindeers
        .iter()
        .map(|reindeer| distance_traveled(reindeer, time))
        .max()
        .unwrap()
}

pub fn part2(reindeers: &Vec<Reindeer>, time: u32) -> u32 {
    let mut scores = vec![0; reindeers.len()];
    let mut distances = vec![0; reindeers.len()];
    let mut fly_time_left = vec![0; reindeers.len()];
    let mut rest_time_left = vec![1; reindeers.len()];
    for _ in 1..=time {
        let mut lead_distance = u32::MIN;
        for (id, reindeer) in reindeers.iter().enumerate() {
            if rest_time_left[id] > 0 {
                rest_time_left[id] -= 1;
                if rest_time_left[id] == 0 {
                    fly_time_left[id] = reindeer.fly_duration;
                } else {
                    if distances[id] > lead_distance {
                        lead_distance = distances[id];
                    }
                    continue;
                }
            }
            if fly_time_left[id] > 0 {
                fly_time_left[id] -= 1;
                if fly_time_left[id] == 0 {
                    rest_time_left[id] = reindeer.rest_duration + 1;
                }
            }
            distances[id] += reindeer.speed;
            if distances[id] > lead_distance {
                lead_distance = distances[id];
            }
        }
        for (id, &position) in distances.iter().enumerate() {
            if position == lead_distance {
                scores[id] += 1;
            }
        }
    }
    *scores.iter().max().unwrap()
}

fn distance_traveled(reindeer: &Reindeer, time: u32) -> u32 {
    let part_duration = reindeer.fly_duration + reindeer.rest_duration;
    let parts = time / part_duration;
    let mut time_left = time % part_duration;
    if time_left > reindeer.fly_duration {
        time_left = reindeer.fly_duration
    }
    parts * reindeer.fly_duration * reindeer.speed + time_left * reindeer.speed
}

#[test]
fn distance_traveled_test() {
    let comet = Reindeer { name: "Comet".to_string(), speed: 14, fly_duration: 10, rest_duration: 127 };
    let dancer = Reindeer { name: "Dancer".to_string(), speed: 16, fly_duration: 11, rest_duration: 162 };
    assert_eq!(distance_traveled(&comet, 1), 14);
    assert_eq!(distance_traveled(&dancer, 1), 16);
    assert_eq!(distance_traveled(&comet, 10), 140);
    assert_eq!(distance_traveled(&dancer, 10), 160);
    assert_eq!(distance_traveled(&comet, 11), 140);
    assert_eq!(distance_traveled(&dancer, 11), 176);
    assert_eq!(distance_traveled(&comet, 12), 140);
    assert_eq!(distance_traveled(&dancer, 12), 176);
    assert_eq!(distance_traveled(&comet, 1000), 1120);
    assert_eq!(distance_traveled(&dancer, 1000), 1056);
}

#[test]
fn part2_test() {
    let comet = Reindeer { name: "Comet".to_string(), speed: 14, fly_duration: 10, rest_duration: 127 };
    let dancer = Reindeer { name: "Dancer".to_string(), speed: 16, fly_duration: 11, rest_duration: 162 };
    let reindeers = vec![comet, dancer];
    assert_eq!(part2(&reindeers, 140), 139);
    assert_eq!(part2(&reindeers, 1000), 689);
}
