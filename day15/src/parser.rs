#[derive(Debug, PartialEq)]
pub struct Ingredient {
    pub name: String,
    pub capacity: i32,
    pub durability: i32,
    pub flavor: i32,
    pub texture: i32,
    pub calories: i32
}

pub fn parse(data: &str) -> Vec<Ingredient> {
    let mut ingredients = vec![];
    for line in data.lines() {
        ingredients.push(parse_line(line));
    }
    ingredients
}

fn parse_line(line: &str) -> Ingredient {
    let parts: Vec<&str> = line.split(&[':', ',', ' '][..]).collect();
    Ingredient {
        name:        parts[0].to_string(),
        capacity :   parts[3].parse().unwrap(),
        durability : parts[6].parse().unwrap(),
        flavor :     parts[9].parse().unwrap(),
        texture :    parts[12].parse().unwrap(),
        calories :   parts[15].parse().unwrap()
    }
}

#[test]
fn parse_line_test() {
    assert_eq!(
        parse_line("Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3"),
        Ingredient{ name: "Cinnamon".to_string(), capacity: 2, durability: 3, flavor: -2, texture: -1, calories: 3 }
    );
}