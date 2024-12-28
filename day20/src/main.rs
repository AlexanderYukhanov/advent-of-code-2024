use std::collections::{HashSet, VecDeque};

fn find_start_finish(map: &[Vec<char>]) -> ((usize, usize), (usize, usize)) {
    let mut start = (0, 0);
    let mut finish = (0, 0);
    for j in 0..map.len() {
        for i in 0..map[0].len() {
            match map[j][i] {
                'S' => start = (i, j),
                'E' => finish = (i, j),
                _ => {}
            }
        }
    }
    (start, finish)
}

fn bfs(map: &[Vec<char>], start: (usize, usize)) -> Vec<Vec<i32>> {
    let mut result: Vec<Vec<i32>> =
        std::iter::repeat(std::iter::repeat(-1).take(map[0].len()).collect())
            .take(map.len())
            .collect();
    let (x, y) = start;
    result[y][x] = 0;
    let mut discovered: HashSet<(usize, usize)> = HashSet::new();
    let mut pending: VecDeque<(usize, usize, i32)> = VecDeque::new();
    pending.push_back((x, y, 0));
    discovered.insert((x, y));
    while let Some((x, y, dist)) = pending.pop_front() {
        for (dx, dy) in [(-1, 0), (1, 0), (0, 1), (0, -1)] {
            let nx = x as i32 + dx;
            let ny = y as i32 + dy;
            if nx < 0 || ny < 0 {
                continue;
            }
            let nx = nx as usize;
            let ny = ny as usize;
            if nx >= map[0].len() || ny >= map.len() {
                continue;
            }
            if map[ny][nx] == '#' {
                continue;
            }
            if !discovered.insert((nx, ny)) {
                continue;
            }
            result[ny][nx] = dist + 1;
            pending.push_back((nx, ny, dist + 1));
        }
    }
    result
}

fn cheat(
    map: &[Vec<char>],
    back: &[Vec<i32>],
    start: (usize, usize),
    cheat_size: i32,
    win: i32,
) -> usize {
    let (x, y) = start;
    let nocheat = back[y][x];
    let mut discovered: HashSet<(usize, usize)> = HashSet::new();
    let mut pending: VecDeque<(usize, usize, i32)> = VecDeque::new();
    pending.push_back((x, y, 0));
    discovered.insert((x, y));
    let mut result = 0;
    while let Some((x, y, dist)) = pending.pop_front() {
        for (dx, dy) in [(-1, 0), (1, 0), (0, 1), (0, -1)] {
            let nx = x as i32 + dx;
            let ny = y as i32 + dy;
            if nx < 0 || ny < 0 {
                continue;
            }
            let nx = nx as usize;
            let ny = ny as usize;
            if nx >= map[0].len() || ny >= map.len() {
                continue;
            }
            if map[ny][nx] == '#' {
                continue;
            }
            if !discovered.insert((nx, ny)) {
                continue;
            }
            pending.push_back((nx, ny, dist + 1));
        }
        for csx in -cheat_size..=cheat_size {
            for csy in -cheat_size..=cheat_size {
                if csx.abs() + csy.abs() > cheat_size {
                    continue;
                }
                let nx = x as i32 + csx;
                let ny = y as i32 + csy;
                if nx < 0 || ny < 0 {
                    continue;
                }
                let nx = nx as usize;
                let ny = ny as usize;
                if nx >= map[0].len() || ny >= map.len() {
                    continue;
                }
                if map[ny][nx] == '#' {
                    continue;
                }
                if back[ny][nx] == -1 {
                    continue;
                }
                let ndist = dist + back[ny][nx] + csy.abs() + csx.abs();
                if nocheat - ndist >= win {
                    result += 1;
                }
            }
        }
    }
    result
}

fn main() {
    let map: Vec<Vec<char>> = std::fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|l| l.chars().collect())
        .collect();

    let (start, finish) = find_start_finish(&map);
    let direct = bfs(&map, finish);
    let part1 = cheat(&map, &direct, start, 2, 100);
    let part2 = cheat(&map, &direct, start, 20, 100);
    println!("Day 20.1: {}", part1);
    println!("Day 20.2: {}", part2);
}
