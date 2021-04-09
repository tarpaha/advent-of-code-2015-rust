use crate::parser::Ingredient;

pub fn part1(ingredients: &Vec<Ingredient>, sum: u32) -> u32 {
    find_best_score(ingredients, sum, 0)
}

pub fn part2(ingredients: &Vec<Ingredient>, sum: u32, required_calories: u32) -> u32 {
    find_best_score(ingredients, sum, required_calories)
}

fn find_best_score(ingredients: &Vec<Ingredient>, sum: u32, required_calories: u32) -> u32 {
    let permutations = get_permutations(sum, ingredients.len() as u32);
    let mut max_score = 0;
    for permutation in permutations {
        let mut capacity = 0;
        let mut durability = 0;
        let mut flavor = 0;
        let mut texture = 0;
        let mut calories = 0;
        for (id, number) in permutation.iter().enumerate() {
            let mul = *number as i32;
            capacity += mul * ingredients[id].capacity;
            durability += mul * ingredients[id].durability;
            flavor += mul * ingredients[id].flavor;
            texture += mul * ingredients[id].texture;
            calories += mul * ingredients[id].calories;
        }
        if required_calories > 0 && calories != required_calories as i32 {
            continue;
        }
        let score  = if capacity < 0 || durability < 0 || flavor < 0 || texture < 0 {
            0
        }
        else {
            capacity * durability * flavor * texture
        };
        if score > max_score {
            max_score = score;
        }
    }
    max_score as u32
}

//  returns all combinations of <count> numbers that produce sum = <n>
fn get_permutations(n: u32, count: u32) -> Vec<Vec<u32>> {
    let mut out = vec![];
    permute(n, count, vec![], &mut out);
    out
}

fn permute(n: u32, left: u32, currently_collected: Vec<u32>, out: &mut Vec<Vec<u32>>) {
    if left == 1 {
        let mut vec = currently_collected.clone(); vec.push(n);
        out.push(vec);
    } else {
        for i in 0..=n {
            let mut vec = currently_collected.clone(); vec.push(i);
            permute(n - i, left - 1, vec, out);
        }
    }
}

#[test]
pub fn get_permutations_test() {
    assert_eq!(get_permutations(2, 2), [[0, 2], [1, 1], [2, 0]]);
}

#[test]
pub fn part1_test() {
    let butterscotch = Ingredient{ name: "Butterscotch".to_string(), capacity: -1, durability: -2, flavor: 6, texture: 3, calories: 8 };
    let cinnamon = Ingredient{ name: "Cinnamon".to_string(), capacity: 2, durability: 3, flavor: -2, texture: -1, calories: 3 };
    let ingredients = vec![butterscotch, cinnamon];
    assert_eq!(part1(&ingredients, 100), 62842880);
}

#[test]
pub fn part2_test() {
    let butterscotch = Ingredient{ name: "Butterscotch".to_string(), capacity: -1, durability: -2, flavor: 6, texture: 3, calories: 8 };
    let cinnamon = Ingredient{ name: "Cinnamon".to_string(), capacity: 2, durability: 3, flavor: -2, texture: -1, calories: 3 };
    let ingredients = vec![butterscotch, cinnamon];
    assert_eq!(part2(&ingredients, 100, 500), 57600000);
}