use std::collections::{HashSet, VecDeque};

fn bfs(size: usize, blocks: &[(usize, usize)]) -> Option<usize> {
    let dirs = vec![(-1, 0), (1, 0), (0, 1), (0, -1)];
    let mut occupied: HashSet<(usize, usize)> = HashSet::new();
    occupied.extend(blocks.iter());

    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut pending: VecDeque<(usize, usize, usize)> = VecDeque::new();
    pending.push_back((0, 0, 0));
    
    while !pending.is_empty() {
        let (x, y, path) = pending.pop_front().unwrap();
        if x == size - 1 && y == size - 1 {
            return Some(path);
        }
        for &(dx, dy) in dirs.iter() {
            let nx = x as i32 + dx;
            let ny = y as i32 + dy;
            if nx < 0 || ny < 0 {
                continue;
            }
            let nx = nx as usize;
            let ny = ny as usize;
            if nx == size || ny == size {
                continue;
            }
            if occupied.contains(&(nx, ny)) || !visited.insert((nx, ny)) {
                continue;
            }
            pending.push_back((nx, ny, path + 1));
        }
    }
    None
}

fn main() {    
    let size = 71;
    let fallen = 1024;
    let path = "input.txt";
    
    let blocks: Vec<(usize, usize)> = std::fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(|line| {
            let (x, y) = line.split_once(",").unwrap();
            (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap())
        })
        .collect();

    println!("Day 18.1: {:?}", bfs(size, &blocks[..fallen]));

    let mut left = fallen;
    let mut right = blocks.len();
    while left + 1 != right {
        let mid = (left + right) >> 1;
        if bfs(size, &blocks[..mid]).is_none() {
            right = mid;
        } else {
            left = mid;
        }
    }

    println!("Day 18.2: {:?}", blocks[right - 1]);
}
