fn fit(lock: &[i32], key: &[i32]) -> bool {
    for ind in 0..lock.len() {
        if lock[ind] + key[ind] > 7 {
            return false;
        }
    }
    true
}

fn main() {
    let mut keys: Vec<Vec<i32>> = vec![];
    let mut locks: Vec<Vec<i32>> = vec![];
    let mut key: bool = false;
    let mut inside = false;
    let mut current: Vec<i32> = std::iter::repeat(0).take(5).collect();
    std::fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .for_each(|line| {
            if line == "" {
                if inside {
                    if key {
                        keys.push(current.clone());
                    } else {
                        locks.push(current.clone());
                    }
                }
                inside = false;
                current.fill(0);
            }
            if line != "" && !inside {
                inside = true;
                key = line == ".....";
            }
            if inside {
                for (ind, ch) in line.chars().enumerate() {
                    if ch == '#' {
                        current[ind] += 1;
                    }
                }
            }
        });

    let mut part1 = 0;
    for key in keys.iter() {
        for lock in locks.iter() {
            if fit(lock, key) {
                part1 += 1;
            }
        }
    }

    println!("Day 25.1: {}", part1);
}
