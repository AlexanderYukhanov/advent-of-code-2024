use std::{collections::HashMap, result};

fn prune(secret: u64) -> u64 {
    secret % 16777216
}

fn mix(value: u64, secret: u64) -> u64 {
    value ^ secret
}

fn next_value(secret: u64) -> u64 {
    let secret = prune(mix(secret, secret * 64));
    let secret = prune(mix(secret, secret / 32));
    prune(mix(secret, secret * 2048))
}

fn prices(mut secret: u64) -> HashMap<Vec<i32>, i32> {
    let mut result = HashMap::new();
    let mut seq = vec![];
    let mut price = (secret % 10) as i32;
    for _ in 0..2000 {
        let next_secret = next_value(secret);
        let next_price = (next_secret % 10) as i32;
        seq.push(next_price - price);
        if seq.len() > 4 {
            seq = seq[1..].to_vec();
        }
        if seq.len() == 4 {
            if !result.contains_key(&seq) {
                result.insert(seq.clone(), next_price);
            }
        }
        secret = next_secret;
        price = next_price;
    }
    result
}

fn nth_value(mut secret: u64, mut steps: usize) -> u64 {
    while steps > 0 {
        secret = next_value(secret);
        steps -= 1;
    }
    secret
}

fn test_next_value() {
    let mut secret = 123;
    for _ in 0..10 {
        secret = next_value(secret);
        println!("{}", secret);
    }
}

fn test_nth_value() {
    println!("{}: {}", 2024, nth_value(2024, 2000));
}

fn combinations() -> Vec<Vec<i32>> {
    let mut combinations = vec![];
    let mut comb = vec![];
    for a in -9..=9 {
        comb.push(a);
        for b in -9..=9 {
            comb.push(b);
            for c in -9..=9 {
                comb.push(c);
                for d in -9..=9 {
                    comb.push(d);
                    combinations.push(comb.clone());
                    comb.pop();
                }
                comb.pop();
            }
            comb.pop();
        }
        comb.pop();
    }
    combinations
}

fn main() {
    let secrets = std::fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|line| line.parse::<u64>().unwrap())
        .collect::<Vec<_>>();
    let part1 = secrets.iter().map(|&v| nth_value(v, 2000)).sum::<u64>();
    println!("Day22.1: {}", part1);

    let sequences = secrets.iter().map(|&s| prices(s)).collect::<Vec<_>>();
    
    let combs = combinations();
    let mut part2 = 0;
    let mut best = vec![];
    ncurses::initscr();
    for (ind, comb) in combs.iter().enumerate() {
        ncurses::clear();
        ncurses::mv(0, 0);
        let _ = ncurses::addstr(&format!("Processed {} of {}: {} %", ind, combs.len(), 100 * ind / combs.len()));
        ncurses::refresh();
        let mut res = 0;
        for seq in sequences.iter() {
            if let Some(bannanas) = seq.get(comb) {
                res += bannanas;
            }
        }
        if res > part2 {
            part2 = res;
            best = comb.clone();
        }
    }
    ncurses::endwin();
    println!("Day22.2: {}, {:?}", part2, best);
}
