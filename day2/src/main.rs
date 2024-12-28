use std::io::BufRead;

fn increasing(report: &Vec<i32>) -> bool {
    for i in 1..report.len() {
        let d = report[i] - report[i - 1];
        if d <= 0 || d > 3 {
            return false;
        }
    }
    return true;
}

fn decreasing(report: &Vec<i32>) -> bool {
    for i in 1..report.len() {
        let d = report[i] - report[i - 1];
        if d >= 0 || d < -3 {
            return false;
        }
    }
    return true;
}

fn safe(report: Vec<i32>, ex: bool) -> bool {
    if ex {
        for exclude in 0..report.len() {
            if safe(
                report
                    .iter()
                    .enumerate()
                    .filter_map(|(ind, &v)| if ind != exclude { Some(v) } else { None })
                    .collect(),
                false,
            ) {
                return true;
            }
        }
        return false;
    } else {
        return increasing(&report) || decreasing(&report);
    };
}

fn main() {
    let cnt = std::io::BufReader::new(std::fs::File::open("intput.txt").unwrap())
        .lines()
        .filter_map(|v| v.ok())
        .filter(|s| {
            safe(
                s.split_whitespace()
                    .filter_map(|v| v.parse::<i32>().ok())
                    .collect::<Vec<i32>>(), false
            )
        })
        .count();

    println!("Day2.1: {}", cnt);

    let cnt = std::io::BufReader::new(std::fs::File::open("intput.txt").unwrap())
        .lines()
        .filter_map(|v| v.ok())
        .filter(|s| {
            safe(
                s.split_whitespace()
                    .filter_map(|v| v.parse::<i32>().ok())
                    .collect::<Vec<i32>>(), true
            )
        })
        .count();

    println!("Day2.2: {}", cnt);
}
