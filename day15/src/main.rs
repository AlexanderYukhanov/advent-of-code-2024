use std::collections::HashSet;

struct Warehouse {
    state: Vec<Vec<char>>,
    x: i32,
    y: i32,
    moves: Vec<char>,
    mv: usize,
}

impl Warehouse {
    fn from_input(input: &str) -> Self {
        let mut state = vec![];
        let mut moves = vec![];
        for line in input.lines() {
            if line.is_empty() {
                continue;
            } else if line.starts_with('#') {
                state.push(line.chars().collect::<Vec<char>>());
            } else {
                moves.extend(line.chars());
            }
        }

        let mut x = 0;
        let mut y = 0;
        for j in 0..state.len() {
            for i in 0..state[0].len() {
                if state[j][i] == '@' {
                    state[j][i] = '.';
                    x = i as i32;
                    y = j as i32;
                }
            }
        }

        Self {
            state,
            x,
            y,
            moves,
            mv: 0,
        }
    }

    fn state(&self) -> &Vec<Vec<char>> {
        &self.state
    }

    fn step(&mut self) -> bool {
        if self.mv == self.moves.len() {
            return false;
        }

        let (dx, dy) = match self.moves[self.mv] {
            '<' => (-1, 0),
            '>' => (1, 0),
            '^' => (0, -1),
            'v' => (0, 1),
            _ => panic!("Unrecognized move: {} at {}", self.moves[self.mv], self.mv),
        };

        self.mv += 1;
        let nx = self.x + dx;
        let ny = self.y + dy;
        match self.state[ny as usize][nx as usize] {
            'O' => {
                let mut tx = nx;
                let mut ty = ny;
                let mut pushed = false;
                loop {
                    match self.state[ty as usize][tx as usize] {
                        '#' => break,
                        'O' => {}
                        _ => {
                            self.state[ty as usize][tx as usize] = 'O';
                            self.state[ny as usize][nx as usize] = '.';
                            pushed = true;
                            break;
                        }
                    }
                    tx += dx;
                    ty += dy;
                }
                if pushed {
                    self.x = nx;
                    self.y = ny;
                }
            }
            '#' => {}
            _ => {
                self.x = nx;
                self.y = ny;
            }
        }
        true
    }

    fn plot(&self) {
        for j in 0..self.state.len() {
            for i in 0..self.state[0].len() {
                ncurses::mv(j as i32, i as i32);
                ncurses::addch(self.state[j][i] as u32);
            }
        }
        ncurses::mv(self.y, self.x);
        if self.mv != self.moves.len() {
            ncurses::addch(self.moves[self.mv] as u32);
        } else {
            ncurses::addch('@' as u32);
        }
    }

    fn score(&self) -> u64 {
        let mut result = 0;
        for j in 0..self.state.len() {
            for i in 0..self.state[0].len() {
                if self.state[j][i] == 'O' {
                    result += (j * 100 + i) as u64;
                }
            }
        }
        result
    }
}

struct SecondWarehouse {
    state: Vec<Vec<char>>,
    x: i32,
    y: i32,
    moves: Vec<char>,
    mv: usize,
}

impl SecondWarehouse {
    fn from_input(input: &str) -> Self {
        let mut state = vec![];
        let mut moves = vec![];
        for line in input.lines() {
            if line.is_empty() {
                continue;
            } else if line.starts_with('#') {
                let line = line
                    .replace("#", "##")
                    .replace("O", "[]")
                    .replace(".", "..")
                    .replace("@", "@.");
                state.push(line.chars().collect::<Vec<char>>());
            } else {
                moves.extend(line.chars());
            }
        }

        let mut x = 0;
        let mut y = 0;
        for j in 0..state.len() {
            for i in 0..state[j].len() {
                if state[j][i] == '@' {
                    state[j][i] = '.';
                    x = i as i32;
                    y = j as i32;
                }
            }
        }

        Self {
            state,
            x,
            y,
            moves,
            mv: 0,
        }
    }

    fn step(&mut self) -> bool {
        if self.mv == self.moves.len() {
            return false;
        }

        let (dx, dy) = match self.moves[self.mv] {
            '<' => (-1, 0),
            '>' => (1, 0),
            '^' => (0, -1),
            'v' => (0, 1),
            _ => panic!("Unrecognized move: {} at {}", self.moves[self.mv], self.mv),
        };

        self.mv += 1;
        let nx = self.x + dx;
        let ny = self.y + dy;
        match self.state[ny as usize][nx as usize] {
            '#' => {}
            '.' => {
                self.x = nx;
                self.y = ny;
            }
            ch => {
                if dy == 0 {
                    if self.push_horizontal(nx, dx) {
                        self.x = nx;
                    }
                } else {
                    if self.push_vertical(
                        ny,
                        if ch == '[' {
                            vec![nx, nx + 1]
                        } else {
                            vec![nx - 1, nx]
                        },
                        dy,
                    ) {
                        self.y = ny;
                    }
                }
            }
        }
        true
    }

    fn push_horizontal(&mut self, x: i32, dx: i32) -> bool {
        let mut t = x;
        loop {
            match self.state[self.y as usize][t as usize] {
                '.' => break,
                '#' => return false,
                _ => t += dx,
            }
        }
        while t != x {
            let nt = t - dx;
            self.state[self.y as usize][t as usize] = self.state[self.y as usize][nt as usize];
            t = nt;
        }

        self.state[self.y as usize][x as usize] = '.';
        true
    }

    fn push_vertical(&mut self, y: i32, affected: Vec<i32>, dy: i32) -> bool {
        if affected.is_empty() {
            return true;
        }
        let mut dep = vec![];
        let ny = y + dy;
        for &x in affected.iter() {
            match self.state[ny as usize][x as usize] {
                '[' => {
                    dep.push(x);
                    dep.push(x + 1);
                }
                ']' => {
                    dep.push(x - 1);
                    dep.push(x);
                }
                '#' => return false,
                _ => {}
            }
        }
        let dep = dep
            .into_iter()
            .collect::<HashSet<i32>>()
            .into_iter()
            .collect::<Vec<i32>>();
        if !self.push_vertical(ny, dep, dy) {
            return false;
        }

        for x in affected {
            self.state[ny as usize][x as usize] = self.state[y as usize][x as usize];
            self.state[y as usize][x as usize] = '.';
        }
        true
    }

    fn plot(&self) {
        for j in 0..self.state.len() {
            for i in 0..self.state[j].len() {
                ncurses::mv(j as i32, i as i32);
                ncurses::addch(self.state[j][i] as u32);
            }
        }
        ncurses::mv(self.y, self.x);
        ncurses::mv(self.y, self.x);
        if self.mv != self.moves.len() {
            ncurses::addch(self.moves[self.mv] as u32);
        } else {
            ncurses::addch('@' as u32);
        }
    }

    fn score(&self) -> u64 {
        let mut result = 0;
        for j in 0..self.state.len() {
            for i in 0..self.state[0].len() {
                if self.state[j][i] == '[' {
                    result += (j * 100 + i) as u64;
                }
            }
        }
        result
    }
}

fn main() {
    ncurses::initscr();
    let mut warehouse = Warehouse::from_input(&std::fs::read_to_string("input.txt").unwrap());
    warehouse.plot();

    while warehouse.step() {
        //warehouse.plot();
        //ncurses::refresh();
    }
    //ncurses::getch();

    let mut warehouse2 =
        SecondWarehouse::from_input(&std::fs::read_to_string("input.txt").unwrap());
    warehouse2.plot();
    while warehouse2.step() {
        //ncurses::getch();
        warehouse2.plot();
        ncurses::refresh();
    }
    ncurses::getch();
    ncurses::endwin();

    println!("Day 15.1: {}", warehouse.score());
    println!("Day 15.2: {}", warehouse2.score());
}
