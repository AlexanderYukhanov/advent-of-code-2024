use std::{collections::HashSet, io::BufRead};

fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcd(a: i64, b: i64) -> i64 {
    a / gcd(a, b) * b
}

fn period(d: i32, size: i32) -> i64 {
    size as i64 / gcd(d.abs() as i64, size as i64)
}

fn main() {
    let width = 101;
    let height = 103;

    let part2 = std::io::BufReader::new(std::fs::File::open("input.txt").unwrap())
        .lines()
        .filter_map(|v| v.ok())
        .filter(|line| !line.is_empty())
        .map(|line| {
            line.replace("p=", "")
                .replace("v=", "")
                .replace(",", " ")
                .split_ascii_whitespace()
                .map(|s| s.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .map(|input| {
            let [_, _, dx, dy] = input[..].try_into().unwrap();
            lcd(period(dx, width), period(dy, height))
        })
        .fold(1, |acc, p| lcd(acc, p));
    println!("Day 14.2: {}", part2);

    let mut pos: Vec<(i32, i32, i32, i32)> =
        std::io::BufReader::new(std::fs::File::open("input.txt").unwrap())
            .lines()
            .filter_map(|v| v.ok())
            .filter(|line| !line.is_empty())
            .map(|line| {
                line.replace("p=", "")
                    .replace("v=", "")
                    .replace(",", " ")
                    .split_ascii_whitespace()
                    .map(|s| s.parse::<i32>().unwrap())
                    .collect::<Vec<_>>()
            })
            .map(|input| {
                let [x, y, dx, dy] = input[..].try_into().unwrap();
                (x, y, dx, dy)
            })
            .collect();

    ncurses::initscr();
    for step in 0..part2 {
        pos = pos
            .into_iter()
            .map(|(x, y, dx, dy)| ((x + dx + width) % width, (y + dy + height) % height, dx, dy))
            .collect();
        let uniq: HashSet<(i32, i32)> = pos.iter().map(|&(x, y, _, _)| (x, y)).collect();

        if uniq.len() != pos.len() {
              continue;
        }
        ncurses::clear();
        for &(x, y, _, _) in pos.iter() {
            ncurses::mv(y, x);
            ncurses::addch('*' as u32);
        }
        ncurses::mv(0, 0);
        let _ = ncurses::addstr(format!("{}", step).as_str());
        ncurses::refresh();
        ncurses::getch();
    }
    ncurses::endwin();

    let part1 = std::io::BufReader::new(std::fs::File::open("input.txt").unwrap())
        .lines()
        .filter_map(|v| v.ok())
        .filter(|line| !line.is_empty())
        .map(|line| {
            line.replace("p=", "")
                .replace("v=", "")
                .replace(",", " ")
                .split_ascii_whitespace()
                .map(|s| s.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .map(|input| {
            let [x, y, dx, dy] = input[..].try_into().unwrap();
            (
                ((x + dx * 100) % width + width) % width,
                ((y + dy * 100) % height + height) % height,
            )
        })
        .map(|(x, y)| match (x, y) {
            (x, y) if x < width / 2 && y < height / 2 => Some(0),
            (x, y) if x > width / 2 && y < height / 2 => Some(1),
            (x, y) if x < width / 2 && y > height / 2 => Some(2),
            (x, y) if x > width / 2 && y > height / 2 => Some(3),
            _ => None,
        })
        .filter_map(|r| r)
        .fold(vec![0, 0, 0, 0], |mut acc, n| {
            acc[n] += 1;
            acc
        })
        .into_iter()
        .fold(1, |acc, v| acc * v);

    println!("Day 14.1: {}", part1);
}
