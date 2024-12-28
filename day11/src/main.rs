use std::collections::HashMap;

fn digits(mut n: u64) -> usize {
    let mut res = 1;
    while n >= 10 {
        res += 1;
        n /= 10;
    }
    res
}

fn count(start: u64, steps: u32, mem: &mut HashMap<(u64, u32), u64> ) -> u64 {
    if steps == 0 {
        return 1;
    }
    if let Some(&cached) = mem.get(&(start, steps)) {
        return cached;
    }
    let result;
    let digits = digits(start);
    if start == 0 {
        result = count(1, steps - 1, mem);
    } else if digits & 1 == 1 {
        result = count(start * 2024, steps - 1, mem);
    } else {
        let mut mask = 1;
        for _ in 0..digits/2 {
            mask *= 10;
        }
        let lhs = start / mask;
        let rhs = start % mask;
        //println!("Split {} -> {}:{}", start, lhs, rhs);
        result = count(lhs, steps - 1, mem) + count(rhs, steps - 1, mem);
    }
    mem.insert((start, steps), result);
    //println!("{}/{}: {} -> {}", start, digits, steps, result);
    result
}

fn main() {
    let input: Vec<u64> = std::fs::read_to_string("input.txt")
        .unwrap()
        .split_ascii_whitespace()
        .map(|v| v.parse().unwrap())
        .collect();
    let mut mem: HashMap<(u64, u32), u64> = HashMap::new();
    let part1: u64 = input.iter().map(|v| count(*v, 25, &mut mem)).sum();
    let part2: u64 = input.iter().map(|v| count(*v, 75, &mut mem)).sum();
    println!("Day 11.1 {}", part1);
    println!("Day 11.2 {}", part2);
    println!("Cached values: {}", mem.len());
}
