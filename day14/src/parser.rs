#[derive(Debug, PartialEq)]
pub struct Reindeer {
    pub name: String,
    pub speed: u32,
    pub fly_duration: u32,
    pub rest_duration: u32
}

pub fn parse(data: &str) -> Vec<Reindeer> {
    let mut reindeers = vec![];
    for line in data.lines() {
        reindeers.push(parse_line(line));
    }
    reindeers
}

fn parse_line(line: &str) -> Reindeer {
    let parts: Vec<&str> = line.split(' ').collect();
    let name = parts[0].to_string();
    let speed = parts[3].parse().unwrap();
    let fly_duration = parts[6].parse().unwrap();
    let rest_duration = parts[13].parse().unwrap();
    Reindeer{ name, speed, fly_duration, rest_duration }
}

#[test]
fn parse_line_test() {
    assert_eq!(
        parse_line("Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds."),
        Reindeer { name: "Comet".to_string(), speed: 14, fly_duration: 10, rest_duration: 127 } )
}