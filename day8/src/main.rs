use std::{collections::HashSet, io::BufRead};
use ncurses;

fn main() {
    ncurses::initscr();
    let reader = std::io::BufReader::new(std::fs::File::open("input.txt").unwrap());
    let map: Vec<Vec<char>> = reader
        .lines()
        .map(|line| line.unwrap())
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();
    let width = map.len();
    let height = map[0].len();
    for y in 0..height {
        for x in 0..width {
            ncurses::mv(y as i32, x as i32);
            ncurses::addch(map[y][x] as u32);
        }
    }
    ncurses::refresh();
    let mut visited: HashSet<i32> = HashSet::new();
    let key = |x: i32, y: i32| x + y * 1024;
    for y in 0..height {
        for x in 0..width {
            for yo in 0..height {
                for xo in 0..width {
                    if x == xo && y == yo {
                        continue;
                    }
                    if map[y][x] != map[yo][xo] || map[y][x] == '.' {
                        continue;
                    }
                    let dx = x as i32 - xo as i32;
                    let dy = y as i32 - yo as i32;
                    let mut hrm = 0;
                    loop {
                        let nx = x as i32 + hrm * dx;
                        let ny = y as i32 + hrm * dy;
                        hrm += 1;
                        if nx < 0 || ny < 0 || nx >= width as i32 || ny >= height as i32 {
                            break;
                        }
                        ncurses::mv(ny, nx);
                        ncurses::addch('#' as u32);
                        ncurses::refresh();
                        visited.insert(key(nx, ny));
                    }
                }
            }
        }
    }
    ncurses::getch();
    ncurses::endwin();
    println!("Day 8.1: {}", visited.len());
}
