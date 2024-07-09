use serde_json::Value;

fn main() {
    let input = include_str!("../input.txt").trim();
    let json: Value = serde_json::from_str(&input).unwrap();

    let p1 = json_sum(&json, false);
    println!("Part 1. Sum = {p1}");

    let p2 = json_sum(&json, true);
    println!("Part 2. Sum = {p2}");
}


fn json_sum(expr: &Value, exclude_red: bool) -> i64 {
    match expr {
        Value::Number(n) => n.as_i64().unwrap(),
        Value::Object(map) => {
            if exclude_red && map.values().any(|v| v == "red") {
                0
            } else {
                map.values().map(|v| json_sum(v, exclude_red)).sum::<i64>()
            }
        }
        Value::Array(a) => {
            a.iter().map(|v| json_sum(v, exclude_red)).sum::<i64>()
        }
        _ => 0,
    }
}