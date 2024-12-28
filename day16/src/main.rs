use std::{collections::{HashMap, HashSet}, io::BufRead};

fn main() {
    let map: Vec<Vec<char>> =
        std::io::BufReader::new(std::fs::File::open("input.txt").unwrap())
            .lines()
            .map(|line| line.unwrap())
            .filter(|line| !line.is_empty())
            .map(|line| line.chars().collect())
            .collect();
    let x = 1;
    let y = map.len() - 2;
    let mut visited: HashMap<(usize, usize, usize), usize> = HashMap::new();
    visited.insert((x, y, 1), 0);
    let mut pending: Vec<(usize, usize, usize, Vec<(usize, usize)>)> = vec![(x, y, 1, vec![(x, y)])];
    let mut best = usize::max_value();
    let mut tiles: HashSet<(usize, usize)> = HashSet::new();

    while !pending.is_empty() {
        let (x, y, dir, path) = pending.pop().unwrap();
        let current = visited[&(x, y, dir)];
        if y == 1 && x == map[0].len() - 2 {
            if current == best {
                tiles.extend(path.iter());
            } 
            else if current < best {
                best = current;
                tiles.clear();
                tiles.extend(path.iter());
            }
            continue;
        }
        let forward = match dir {
            0 => (0, -1),
            1 => (1, 0),
            2 => (0, 1),
            _ => (-1, 0),
        };
        let backward = (forward.0 * -1, forward.1 * -1);
        for (dx, dy) in vec![(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let price = if (dx, dy) == forward {1} else if (dx, dy) == backward {2002} else {1001};
            let nx = (x as i32 + dx) as usize;
            let ny = (y as i32 + dy) as usize;
            if map[ny][nx] == '#' {
                continue;
            }
            let ndir = match (dx, dy) {
                (0, -1) => 0,
                (1, 0) => 1,
                (0, 1) => 2,
                _ => 3
            };
            if let Some(&best) = visited.get(&(nx, ny, ndir)) {
                if best < current + price {
                    continue;
                }
            }
            visited.insert((nx, ny, ndir), current + price);
            let mut newpath = path.clone();
            newpath.push((nx, ny));
            pending.push((nx, ny, ndir, newpath));
        }
    }
    println!("Day 16.1: {}", best);
    println!("Day 16.2: {}", tiles.len());
}
