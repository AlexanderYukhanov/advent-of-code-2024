use regex::Regex;

fn main() {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|do\(\)|don't\(\)").unwrap();
    let content = std::fs::read_to_string("input.txt").unwrap();
    let result = re
        .captures_iter(&content)
        .fold((true, 0, 0), |(enabled, sum, cond_sum), m| match &m[0] {
            "do()" => (true, sum, cond_sum),
            "don't()" => (false, sum, cond_sum),
            _ => {
                let a: i64 = m[1].parse().unwrap();
                let b: i64 = m[2].parse().unwrap();
                (
                    enabled,
                    sum + a * b,
                    cond_sum + if enabled { a * b } else { 0 },
                )
            }
        });
    println!("Day3.1: {}", result.1);
    println!("Day3.2: {}", result.2);
}
