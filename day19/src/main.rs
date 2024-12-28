use std::collections::{HashMap, HashSet};

fn possible(pattern: &str, towels: &[&str], mem: &mut HashMap<String, usize>) -> usize {
    if pattern.is_empty() {
        return 1;
    }
    if let Some(ways) = mem.get(pattern) {
        return *ways;
    }
    let mut ways = 0;
    for t in towels {
        if pattern.starts_with(t) {
            ways += possible(&pattern[t.len()..], towels, mem) ;
        }
    }
    mem.insert(pattern.to_string(), ways);
    return ways;
}

fn main() {
    let content = std::fs::read_to_string("input.txt").unwrap();
    let towels: Vec<&str> = content.lines().next().unwrap().split(", ").collect();
    let sol = content
        .lines()
        .skip(2)
        .filter_map(|pattern| {
            let ways = possible(pattern, &towels, &mut HashMap::new());
            if ways != 0 {
                Some(ways)
            } else {
                None
            }
        }).fold((0, 0), |acc, v| (acc.0 + 1, acc.1 + v));
    
    println!("Day 19.1: {}", sol.0);
    println!("Day 19.2: {}", sol.1);
}
