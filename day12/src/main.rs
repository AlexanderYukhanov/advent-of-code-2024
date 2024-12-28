use std::io::BufRead;
use std::iter;

fn count_vert(outline: &Vec<Vec<u8>>, mask: u8) -> u64 {
    let mut result = 0;
    for x in 0..outline[0].len() {
        let mut inside = false;
        for y in 0..outline.len() {
            if outline[y][x] & mask == mask {
                if !inside {
                    result += 1;
                    inside = true;
                }
            } else {
                inside = false;
            }
        }
    }
    result
}

fn count_horiz(outline: &Vec<Vec<u8>>, mask: u8) -> u64 {
    let mut result = 0;
    for y in 0..outline.len() {
        let mut inside = false;
        for x in 0..outline[0].len() {
            if outline[y][x] & mask == mask {
                if !inside {
                    result += 1;
                    inside = true;
                }
            } else {
                inside = false;
            }
        }
    }
    result
}

fn main() {
    let map: Vec<Vec<char>> =
        std::io::BufReader::new(std::fs::File::open("input.txt").unwrap())
            .lines()
            .filter_map(|v| v.ok())
            .filter(|v| !v.is_empty())
            .map(|v| v.chars().collect())
            .collect();
    let height = map.len();
    let width = map[0].len();
    let mut part1 = 0u64;
    let mut part2 = 0u64;
    let mut handled: Vec<Vec<bool>> = iter::repeat(iter::repeat(false).take(width).collect())
        .take(height)
        .collect();

    for y in 0..width {
        for x in 0..width {
            if handled[y][x] {
                continue;
            }
            let mut outline: Vec<Vec<u8>> = iter::repeat(iter::repeat(0).take(width).collect())
                .take(height)
                .collect();
            handled[y][x] = true;
            let plant = map[y][x];
            let mut square = 0;
            let mut joins = 0;
            let mut pending = vec![(x, y)];
            while !pending.is_empty() {
                let (x, y) = pending.pop().unwrap();
                square += 1;
                for (ind, (dx, dy)) in [(-1, 0), (1, 0), (0, 1), (0, -1)].iter().enumerate() {
                    let nx = x as i32 + dx;
                    let ny = y as i32 + dy;
                    if nx < 0 || ny < 0 || nx as usize >= width || ny as usize >= height {
                        outline[y][x] |= 1 << ind;
                        continue;
                    }
                    let nx = nx as usize;
                    let ny = ny as usize;
                    if map[ny][nx] != plant {
                        outline[y][x] |= 1 << ind;
                        continue;
                    }
                    joins += 1;
                    if handled[ny][nx] {
                        continue;
                    }
                    handled[ny][nx] = true;
                    pending.push((nx, ny));
                }
            }
            println!(
                "Plant: {}, Square: {}, Perimeter: {}",
                plant,
                square,
                square * 4 - joins
            );
            let sides = count_vert(&outline, 1)
                + count_vert(&outline, 2)
                + count_horiz(&outline, 4)
                + count_horiz(&outline, 8);
            println!("Plant: {}, Square: {}, Sides: {}", plant, square, sides);
            part1 += square * (square * 4 - joins) as u64;
            part2 += square * sides;
        }
    }

    println!("Day 12.1: {}", part1);
    println!("Day 12.2: {}", part2);
}
