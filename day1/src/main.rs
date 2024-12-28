use std::io::BufRead;
use std::{collections::HashMap, io};

fn main() {
    let mut lhs = vec![];
    let mut rhs = vec![];
    io::BufReader::new(std::fs::File::open("input.txt").unwrap())
        .lines()
        .filter_map(|l| l.ok())
        .for_each(|line| {
            let parts: Vec<i32> = line
                .split_whitespace()
                .filter_map(|s| s.parse::<i32>().ok())
                .collect();
            if let [l, r] = parts.as_slice() {
                lhs.push(*l);
                rhs.push(*r);
            }
        });
    lhs.sort_unstable();
    rhs.sort_unstable();
    let dist: i64 = (0..rhs.len()).map(|i| (rhs[i] - lhs[i]).abs() as i64).sum();
    let freq = rhs.iter().fold(HashMap::new(), |mut acc, &v| {
        *acc.entry(v).or_insert(0) += 1;
        acc
    });
    let score: i64 = lhs
        .iter()
        .map(|&v| v as i64 * *freq.get(&v).unwrap_or(&0) as i64)
        .sum();
    println!("Day 1.1: {}", dist);
    println!("Day 1.2: {}", score);
}
