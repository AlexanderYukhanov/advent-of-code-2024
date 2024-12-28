use ncurses;
use std::collections::{HashMap, HashSet};

#[derive(Clone)]
struct Dir {
    current: usize,
}

impl Dir {
    fn from_char(ch: char) -> Self {
        Self {
            current: match ch {
                '^' => 0,
                '>' => 1,
                'v' => 2,
                '<' => 3,
                _ => panic!("Uknown initial dir: '{}'", ch),
            },
        }
    }

    fn turn(&mut self) {
        self.current = (self.current + 1) % 4;
    }

    fn go(&self, x: usize, y: usize, w: usize, h: usize) -> Option<(usize, usize)> {
        let bounded_increase = |v: usize, limit: usize| {
            if v + 1 < limit {
                Some(v + 1)
            } else {
                None
            }
        };
        let nx = match self.current {
            1 => bounded_increase(x, w),
            3 => x.checked_sub(1),
            _ => Some(x),
        };
        let ny = match self.current {
            0 => y.checked_sub(1),
            2 => bounded_increase(y, h),
            _ => Some(y),
        };
        match (nx, ny) {
            (Some(x), Some(y)) => Some((x, y)),
            _ => None,
        }
    }

    fn value(&self) -> usize {
        1 << self.current
    }
}

fn travel(
    map: &Vec<Vec<char>>,
    mut x: usize,
    mut y: usize,
    mut dir: Dir,
    path: &mut HashSet<(usize, usize)>,
) -> bool {
    let key = |x: usize, y: usize| x + y * 255;
    let mut visited_pos: HashMap<usize, usize> = HashMap::new();
    let height = map.len();
    let width = map[0].len();
    loop {
        let visited = visited_pos
            .get(&key(x, y))
            .is_some_and(|v| (*v & dir.value()) == dir.value());
        if visited {
            return false;
        }
        path.insert((x, y));
        //ncurses::mv(y as i32, x as i32);
        //ncurses::refresh();
        //sleep(time::Duration::from_secs(1));
        //ncurses::addch(65);
        *visited_pos.entry(key(x, y)).or_insert(0) |= dir.value();
        match dir.go(x, y, width, height) {
            Some((nx, ny)) => {
                if map[ny][nx] != '#' {
                    x = nx;
                    y = ny;
                } else {
                    dir.turn();
                }
            }
            _ => {
                return true;
            }
        }
    }
}

fn main() {
    ncurses::initscr();
    let mut map: Vec<Vec<char>> = std::fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|line| line.chars().into_iter().collect())
        .collect();

    let height = map.len();
    let width = map[0].len();
    let (dir, x, y) = (|| {
        for line in 0..height {
            for col in 0..width {
                let ch = map[line][col];
                if ch != '.' && ch != '#' {
                    return (ch, col, line);
                }
            }
        }
        panic!("Starting pos not found!")
    })();
    let dir = Dir::from_char(dir);

    for line in 0..height {
        for col in 0..width {
            ncurses::mv(line as i32, col as i32);
            ncurses::addch(map[line][col] as u32);
        }
    }

    let mut initial_path: HashSet<(usize, usize)> = HashSet::new();
    assert!(travel(&map, x, y, dir.clone(), &mut initial_path));
    initial_path.remove(&(x, y));
    let part2 = initial_path
        .iter()
        .filter(|(cx, cy)| {
            let mut unused: HashSet<(usize, usize)> = HashSet::new();
            map[*cy][*cx] = '#';
            let escaped = travel(&map, x, y, dir.clone(), &mut unused);
            map[*cy][*cx] = '.';
            !escaped
        })
        .count();
    ncurses::mv(0, 0);
    ncurses::endwin();
    println!("Day 6.1: {}", initial_path.len() + 1);
    println!("Day 6.2: {}", part2);
}
