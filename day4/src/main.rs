
fn day_1_1(map: &[Vec<char>]) {
    let dirs = &[
        (0, 1),
        (0, -1),
        (1, 0),
        (-1, 0),
        (1, 1),
        (1, -1),
        (-1, 1),
        (-1, -1),
    ];
    const TARGET: &str = "XMAS";
    let width = map[0].len() as i32;
    let height = map.len() as i32;

    let at_pos = |x: usize, y: usize, i: usize, dx: i32, dy: i32, t: char| -> bool {
        let nx = x as i32 + i as i32 * dx;
        let ny = y as i32 + i as i32 * dy;
        nx >= 0 && nx < width && ny >= 0 && ny < height && map[ny as usize][nx as usize] == t
    };

    let mut cnt = 0;
    for line in 0..map.len() {
        for col in 0..map[line].len() {
            for &(dx, dy) in dirs.iter() {
                if !TARGET.chars().enumerate().any(|(ind, ch)| !at_pos(col, line, ind, dx, dy, ch)) {
                    cnt +=1;
                }
            }
        }
    }

    println!("Day 1.1: {}", cnt);
}

fn day_1_2(map: &Vec<Vec<char>>) {
    let mut cnt = 0;
    for line in 1..map.len() - 1 {
        for col in 1..map[line].len() - 1 {
            if map[line][col] != 'A' {
                continue;
            }
            let w1 = (map[line-1][col-1], map[line+1][col+1]);
            let w2 = (map[line-1][col+1], map[line+1][col-1]);
            if (w1 == ('M','S') || w1 == ('S', 'M')) && (w2 == ('M','S') || w2 == ('S', 'M')) {
                cnt += 1;
            } 
        }
    }

    println!("Day 1.2: {}", cnt);
}

fn main() {
    let content = std::fs::read_to_string("input.txt").unwrap();
    let map: Vec<Vec<char>> = content
        .split("\n")
        .filter(|s| s.len() > 0)
        .map(|s| s.chars().collect::<Vec<char>>())
        .collect();
    day_1_1(&map);
    day_1_2(&map);
}
