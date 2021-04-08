use serde_json::Value; 

pub fn part1(data: &str) -> i64 {
    let v: Value = serde_json::from_str(data).unwrap();
    sum_of_numbers_in_json(&v)
}

pub fn part2(data: &str) -> i64 {
    let v: Value = serde_json::from_str(data).unwrap();
    sum_of_numbers_in_json_except(&v, "red")
}

fn sum_of_numbers_in_json(v: &Value) -> i64 {
    sum_of_numbers_in_json_except(v, "")
}

fn sum_of_numbers_in_json_except(v: &Value, filter: &str) -> i64 {
    match v {
        Value::Number(number) => number.as_i64().unwrap(),
        Value::Array(array) => array.iter().map(|v| sum_of_numbers_in_json_except(v, filter)).sum(),
        Value::Object(map) => {
            if !filter.is_empty() && map.values().any(|v| v == filter) {
                0
            } else {
                map.values().map(|v| sum_of_numbers_in_json_except(v, filter)).sum()
            }
        },
        _ => 0
    }
}

#[test]
fn sum_of_numbers_in_json_test() {
    use serde_json::json;
    assert_eq!(sum_of_numbers_in_json(&json!([1,2,3])), 6);
    assert_eq!(sum_of_numbers_in_json(&json!({"a":2,"b":4})), 6);
    assert_eq!(sum_of_numbers_in_json(&json!([[[3]]])), 3);
    assert_eq!(sum_of_numbers_in_json(&json!({"a":{"b":4},"c":-1})), 3);
    assert_eq!(sum_of_numbers_in_json(&json!({"a":[-1,1]})), 0);
    assert_eq!(sum_of_numbers_in_json(&json!([-1,{"a":1}])), 0);
}

#[test]
fn sum_of_numbers_in_json_except_test() {
    use serde_json::json;
    assert_eq!(sum_of_numbers_in_json_except(&json!([1,2,3]), "red"), 6);
    assert_eq!(sum_of_numbers_in_json_except(&json!([1,{"c":"red","b":2},3]), "red"), 4);
    assert_eq!(sum_of_numbers_in_json_except(&json!({"d":"red","e":[1,2,3,4],"f":5}), "red"), 0);
    assert_eq!(sum_of_numbers_in_json_except(&json!([1,"red",5]), "red"), 6);
}