use std::{collections::HashSet, io::BufRead};

fn score(y: usize, x: usize, map: &Vec<Vec<i32>>) -> usize {
    let mut total = 0;
    let mut discovered: HashSet<(usize, usize)> = HashSet::new();
    let mut pending: Vec<(usize, usize)> = vec![(y, x)];
    while !pending.is_empty() {
        let (y, x) = pending.pop().unwrap();
        let v = map[y][x];
        for (dy, dx) in vec![(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let ny = y as i32 + dy;
            let nx = x as i32 + dx;
            if ny < 0 || nx < 0 || ny as usize >= map.len() || nx as usize >= map[0].len() {
                continue;
            }
            let ny = ny as usize;
            let nx = nx as usize;
            let nv = map[ny][nx];
            if nv != v + 1 {
                continue;
            }
            if !discovered.insert((ny, nx)) {
                continue;
            }
            if nv == 9 {
                total += 1;
            }
            pending.push((ny, nx));
        }
    }
    total
}

fn score2(y: usize, x: usize, map: &Vec<Vec<i32>>) -> usize {
    let v = map[y][x];
    if v == 9 {
        return 1;
    }
    let mut total = 0;
    for (dy, dx) in vec![(-1, 0), (1, 0), (0, -1), (0, 1)] {
        let ny = y as i32 + dy;
        let nx = x as i32 + dx;
        if ny < 0 || nx < 0 || ny as usize >= map.len() || nx as usize >= map[0].len() {
            continue;
        }
        let ny = ny as usize;
        let nx = nx as usize;
        let nv = map[ny][nx];
        if nv != v + 1 {
            continue;
        }
        total += score2(ny, nx, map);
    }
    total
}

fn main() {
    let map: Vec<Vec<i32>> =
        std::io::BufReader::new(std::fs::File::open("input.txt").unwrap())
            .lines()
            .map(|line| {
                line.unwrap()
                    .chars()
                    .map(|ch| ch as i32 - '0' as i32)
                    .collect::<Vec<i32>>()
            })
            .collect();

    let mut part1 = 0;
    let mut part2 = 0;
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if map[y][x] == 0 {
                part1 += score(y, x, &map);
                part2 += score2(y, x, &map);
            }
        }
    }
    println!("Day 10.1: {}", part1);
    println!("Day 10.2: {}", part2);
}
