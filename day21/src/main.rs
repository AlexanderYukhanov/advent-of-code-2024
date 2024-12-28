use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
    hash::{Hash, Hasher},
    result,
};

fn dir(dx: i32, dy: i32) -> String {
    match (dx, dy) {
        (-1, 0) => "<",
        (1, 0) => ">",
        (0, 1) => "v",
        (0, -1) => "^",
        _ => panic!("Unexpected move {}:{}", dx, dy),
    }
    .to_owned()
}

fn locate_digital(ch: char) -> (i32, i32) {
    match ch {
        'A' => (2, 3),
        '0' => (1, 3),
        '1' => (0, 2),
        '2' => (1, 2),
        '3' => (2, 2),
        '4' => (0, 1),
        '5' => (1, 1),
        '6' => (2, 1),
        '7' => (0, 0),
        '8' => (1, 0),
        '9' => (2, 0),
        '#' => (0, 3),
        _ => panic!("Unexpected button: {}", ch),
    }
}

fn locate_dir(ch: char) -> (i32, i32) {
    match ch {
        'A' => (2, 0),
        '^' => (1, 0),
        '>' => (2, 1),
        'v' => (1, 1),
        '<' => (0, 1),
        '#' => (0, 0),
        _ => panic!("Unexpected button: {}", ch),
    }
}

fn dist2(start: (i32, i32), finish: (i32, i32), block: (i32, i32)) -> Vec<String> {
    let dx = finish.0 - start.0;
    let dy = finish.1 - start.1;
    if dx == 0 && dy == 0 {
        return vec![String::new()];
    }
    let mut result = vec![];
    if dx != 0 {
        let nxt = (start.0 + dx / dx.abs(), start.1);
        if nxt != block {
            result.extend(
                dist(nxt, finish, block)
                    .iter()
                    .map(|path| dir(dx / dx.abs(), 0) + path),
            );
        }
    }
    if dy != 0 {
        let nxt = (start.0, start.1 + dy / dy.abs());
        if nxt != block {
            result.extend(
                dist(nxt, finish, block)
                    .iter()
                    .map(|path| dir(0, dy / dy.abs()) + path),
            );
        }
    }
    result
}

fn dist(start: (i32, i32), finish: (i32, i32), block: (i32, i32)) -> Vec<String> {
    let dx = (finish.0 - start.0).signum();
    let dy = (finish.1 - start.1).signum();
    if dx == 0 && dy == 0 {
        return vec![String::new()];
    }
    let mut result = vec![];
    let mut path = vec![];
    let mut good = true;
    let mut current = start;
    while current.0 != finish.0 {
        current.0 += dx;
        if current == block {
            good = false;
            break;
        }
        path.push(dir(dx, 0));
    }
    while current.1 != finish.1 {
        current.1 += dy;
        if current == block {
            good = false;
            break;
        }
        path.push(dir(0, dy));
    }
    if good && dx != 0 {
        result.push(path.join(""));
    }
    let mut path = vec![];
    let mut good = true;
    let mut current = start;
    while current.1 != finish.1 {
        current.1 += dy;
        if current == block {
            good = false;
            break;
        }
        path.push(dir(0, dy));
    }
    while current.0 != finish.0 {
        current.0 += dx;
        if current == block {
            good = false;
            break;
        }
        path.push(dir(dx, 0));
    }
    if good && dy != 0 {
        result.push(path.join(""));
    }
    result
}

fn translate<MapFn>(seq: &[char], keypad: MapFn) -> Vec<String>
where
    MapFn: Fn(char) -> (i32, i32),
{
    let mut result = vec![String::new()];
    let mut current = keypad('A');
    for nxt in seq {
        let mut tmp = vec![];
        for path in dist(current, keypad(*nxt), keypad('#')) {
            for before in result.iter() {
                tmp.push(before.to_owned() + &path + "A");
            }
        }
        result = tmp;
        current = keypad(*nxt);
    }
    result
}

fn sort_letters(s: &str) -> String {
    let mut letters = s.split('A').map(|s| s.to_owned() + "A").collect::<Vec<_>>();
    letters.sort_by(|a, b| b.cmp(a));
    return letters.join("");
}

#[derive(PartialEq, Eq)]
struct Solution {
    paths: HashMap<String, usize>,
}

impl Display for Solution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut sorted_paths: Vec<_> = self.paths.iter().collect();
        sorted_paths.sort_by(|a, b| a.0.cmp(b.0));
        let s = sorted_paths
            .into_iter()
            .fold(String::new(), |acc, (k, &v)| {
                acc + "."
                    + &std::iter::repeat(k)
                        .take(v)
                        .fold(String::new(), |acc2: String, s| acc2 + s)
            });
        f.write_str(&s)
    }
}

impl Hash for Solution {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // Create a sorted vector of the key-value pairs to ensure deterministic order
        let mut sorted_paths: Vec<_> = self.paths.iter().collect();
        sorted_paths.sort_by(|a, b| a.0.cmp(b.0));

        // Hash the sorted key-value pairs
        for (key, value) in sorted_paths {
            key.hash(state);
            value.hash(state);
        }
    }
}

impl Solution {
    fn from_path(path: &str) -> Self {
        Self {
            paths: Self::split_path(path),
        }
    }

    fn split_path(path: &str) -> HashMap<String, usize> {
        let mut result = HashMap::new();
        for subpath in path.split('A') {
            let s = subpath.to_owned() + "A";
            *result.entry(s).or_insert(0) += 1;
        }
        *result.entry("A".to_owned()).or_insert(1) -= 1;
        result
    }

    fn step(&self) -> Vec<Self> {
        let mut solutions: Vec<HashMap<String, usize>> = vec![HashMap::new()];
        for (k, appearences) in self.paths.iter() {
            let translated = translate(&k.chars().collect::<Vec<char>>(), locate_dir)
                .iter()
                .map(|s| Self::split_path(s))
                .collect::<Vec<_>>();
            let mut tmp = vec![];
            for sol in solutions.iter() {
                for add in translated.iter() {
                    let mut ns = sol.clone();
                    for (k, v) in add.iter() {
                        *ns.entry(k.to_owned()).or_insert(0) += v * appearences;
                    }
                    tmp.push(ns);
                }
            }
            solutions = tmp;
        }
        solutions
            .into_iter()
            .map(|p| Self { paths: p })
            .collect::<HashSet<_>>()
            .into_iter()
            .collect()
    }

    fn len(&self) -> u64 {
        self.paths
            .iter()
            .fold(0, |acc, (k, &v)| acc + k.len() as u64 * v as u64)
    }

    fn solve(s: &str, steps: usize, mem: &mut Vec<HashMap<String, u64>>) -> u64 {
        if steps == 0 {
            return s.len() as u64;
        }

        if let Some(&cached) = mem[steps].get(s) {
            return cached;
        }
     
        let translated = translate(&s.chars().collect::<Vec<char>>(), locate_dir)
            .iter()
            .map(|s| Self::split_path(s))
            .collect::<Vec<_>>();

        let result = translated
            .into_iter()
            .map(|variant| {
                variant.into_iter().fold(0u64, |acc, (k, v)| {
                    acc + Self::solve(&k, steps - 1, mem) * v as u64
                })
            })
            .min()
            .unwrap();

        mem[steps].insert(s.to_owned(), result);
        result
    }

    fn do_it(&self, steps: usize) -> u64 {
        let mut mem = std::iter::repeat(HashMap::new()).take(steps + 1).collect();
        let mut result = 0;
        for (k, &v) in self.paths.iter() {
            result += Self::solve(k, steps, &mut mem) * v as u64;
        }
        result
    }

}
fn prove_depends_on_letters() {
    let candidate = "<A^A>^^AvvvA";
    let sizes = translate(&candidate.chars().collect::<Vec<char>>(), locate_dir)
        .iter()
        .map(|s| s.len())
        .collect::<HashSet<usize>>();
    println!(
        "{}.{}: {}",
        candidate,
        candidate.len(),
        sizes.iter().min().unwrap()
    );
    for s in sizes {
        println!("   {}", s);
    }
    let solution = Solution::from_path(candidate);
    let solutions = solution.step();
    let solutions = solutions.iter().flat_map(|s| s.step()).collect::<Vec<_>>();
    let sizes = solutions.iter().map(|s| s.len()).collect::<Vec<_>>();
    println!(
        "{}.{}: {}",
        solution,
        solution.len(),
        sizes.iter().min().unwrap()
    );
    for s in sizes {
        println!("   {}", s);
    }
}

fn main() {
    //prove_depends_on_letters();
    
    let input = ["593A", "508A", "386A", "459A", "246A"];
    //let input = ["029A", "980A", "179A", "456A", "379A"];
    //let input = ["5", "4", "3", "4", "2"];

    let mut part1 = 0;
    let mut part2 = 0;
    for seq in input {
        let mut sub = translate(&seq.chars().collect::<Vec<char>>(), locate_digital);
        let mut solutions = sub
            .iter()
            .map(|path| Solution::from_path(path))
            .collect::<Vec<_>>();
/*
        println!("Day 21.2: {:?}", solutions.iter().map(|s| s.do_it(3)).min().unwrap());
        for i in 0..2 {
            let mut tmp = vec![];
            for sol in solutions.iter() {
                tmp.extend(sol.step());
            }
            solutions = tmp;
        }

        let shortest = solutions.iter().map(|s| s.len()).min().unwrap();
*/
        let val = seq[..seq.len() - 1].parse::<u64>().unwrap();
        let shortest1 = solutions.iter().map(|s| s.do_it(2)).min().unwrap();
        part1 += val * shortest1;
        let shortest2 = solutions.iter().map(|s| s.do_it(25)).min().unwrap();
        part2 += val * shortest2;;
    }

    println!("Day 21.1: {}", part1);
    println!("Day 21.2: {}", part2);

}
