use std::io::BufRead;

fn concat(mut lhs: i64, rhs: i64) -> i64 {
    let mut left = rhs;
    while left != 0 {
        lhs *= 10;
        left /= 10;
    }
    return lhs + rhs;
}

fn reachable(target: i64, value: i64, ops: &[i64], second: bool) -> bool {
    if ops.is_empty() && target == value {
        return true;
    }
    if ops.is_empty() || value > target {
        return false;
    }
    return reachable(target, value + ops[0], &ops[1..], second) ||
        reachable(target, value * ops[0], &ops[1..], second) ||
        (second && reachable(target, concat(value, ops[0]), &ops[1..], second));
}

fn main() {
    let reader = std::io::BufReader::new(std::fs::File::open("input.txt").unwrap());
    let result: i64 = reader
        .lines()
        .into_iter()
        .map(|v| v.unwrap())
        .filter_map(|line| {
            let mut parts = line.split(" ");
            let target = parts.next().unwrap();
            let target = target[..target.len()-1].parse::<i64>().unwrap();
            let operands: Vec<i64> = parts.map(|v| v.parse::<i64>().unwrap()).collect();
            if reachable(target, operands[0], &operands[1..], true) {
                Some(target)
            } else {
                None
            }
        }
        )
        .sum();
    println!("Day 1.1: {}", result);
}
