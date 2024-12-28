use std::io::BufRead;

fn cost(adx: i32, ady: i32, bdx: i32, bdy: i32, tx: i32, ty: i32) -> i32 {
    for a in 0..100 {
        let dx = tx - a * adx;
        let dy = ty - a * ady;
        if dx < 0 || dy < 0 {
            break;
        }

        if dx % bdx != 0 || dy % bdy != 0 {
            continue;
        }

        let b = dx / bdx;
        if b != dy / bdy {
            continue;
        }

        if b > 100 {
            continue;
        }
        println!("{}x{}", a, b);
        return a * 3 + b;
    }
    return 0;
}

fn positive(a: i32, b: i32, c: i64) -> Option<(i64, i64)> {
    let a = a as i64;
    let b = b as i64;
    let (gcd, n, m) = gcd_extended(a, b);
    if c % gcd != 0 {
        return None;
    }

    let k = c / gcd;

    let n_base = n * k;
    let m_base = m * k;

    let n_step = b / gcd;
    let m_step = a / gcd;

    // n = n_base + n_step * t >= 0  t >= -n_base / n_step
    // m = m_base - m_step * t >= 0  t <= m_base / m_step

    let left = -n_base / n_step;
    let right = m_base / m_step;

    //println!("lr: {} {}", left, right);
    if left > right {
        return None
    }

    let mut n = n_base + n_step * left;
    if n < 0 {
        n += n_step;
    }

    Some((n, n_step))
}

fn slow(mut v1: i64, s1: i64, mut v2: i64, s2: i64) -> Option<i64> {
    if (v1 - v2).abs() % gcd(s1, s2) != 0 {
        return None
    }
    while v1 != v2 {
        if v1 > v2 {
            v2 = v2 + s2;
        } else {
            v1 = v1 + s1;
        }
    }
    Some(v1)
}

fn cost2(adx: i32, ady: i32, bdx: i32, bdy: i32, tx: i32, ty: i32) -> i64 {
    let tx = tx as i64 + 10000000000000;
    let ty = ty as i64 + 10000000000000;
    let xs = positive(adx, bdx, tx as i64);
    let ys = positive(ady, bdy, ty as i64);

    if !xs.and(ys).is_some() {
        println!("---");
        return 0;
    }
    let (x, sx) = xs.unwrap();
    let (y, sy) = ys.unwrap();

    if let Some(n) = slow(x, sx, y, sy) {
        let adx = adx as i64;
        let ady = ady as i64;
        let bdx = bdx as i64;
        let bdy = bdy as i64;
        let tx = tx as i64;
        let ty = ty as i64;
        
        if (tx * bdy - ty * bdx) % (adx * bdy - ady * bdx) != 0 {
            return 0;
        }

        let mut n = n;
        let step = sx * sy / gcd(sx, sy);
        loop {
            let mut mx = (tx as i64 - (n * adx as i64)) / bdx as i64;
            let mut my = (ty as i64 - (n * ady as i64)) / bdy as i64;
            let mut d = (mx - my).abs();
            if d == 0 {
                break;
            }
            let mut mx = (tx as i64 - ((n + step) * adx as i64)) / bdx as i64;
            let mut my = (ty as i64 - ((n + step) * ady as i64)) / bdy as i64;
            let d = d - (mx - my).abs();

            let mut d = (mx - my).abs() / d;
            if d < 1 {
                d = 1;
            }
            println!("ss {} {} {} {}", (mx - my).abs(), d, n, step);
            if mx < 0 || my < 0 {
                return 0;
            }
            

            n += step * d;
        }
        let mut mx = (tx as i64 - (n * adx as i64)) / bdx as i64;
        let mut my = (ty as i64 - (n * ady as i64)) / bdy as i64;
        println!("_ {}x{}:{} : {} {}", n, mx, my, (tx as i64 - (n * adx as i64)) % bdx as i64, (ty as i64 - (n * ady as i64)) % bdy as i64);
        //println!("{} = {}", n * adx as i64 + mx * bdx as i64, tx);
        //println!("{} = {}", n * ady as i64 + my * bdy as i64, ty);

        return 3 * n + mx;
    }
    0 
}

fn gcd(a: i64, b: i64) -> i64 {
    if a == 0 {
        b
    } else {
        gcd(b % a, a)
    }
}

fn gcd_extended(a: i64, b: i64) -> (i64, i64, i64) {
    if a == 0 {
        (b, 0, 1)
    } else {
        let (gcd, x1, y1) = gcd_extended(b % a, a);
        (gcd, y1 - (b / a) * x1, x1)
    }
}

fn main() {
    let re_numbers = regex::Regex::new(r"\d+").unwrap();
    let input: Vec<Vec<i32>> =
        std::io::BufReader::new(std::fs::File::open("input.txt").unwrap())
            .lines()
            .filter_map(|line| line.ok())
            .filter(|line| !line.is_empty())
            .map(|line| {
                re_numbers
                    .captures_iter(line.as_str())
                    .map(|m| m.get(0).unwrap().as_str().parse::<i32>().unwrap())
                    .collect()
            })
            //.take(15)
            .collect();


    let mut part1 = 0u32;
    let mut part2 = 0i64;
    for i in (0..input.len()).step_by(3) {
        part1 += cost(
            input[i][0],
            input[i][1],
            input[i + 1][0],
            input[i + 1][1],
            input[i + 2][0],
            input[i + 2][1],
        ) as u32;

        part2 += cost2(
            input[i][0],
            input[i][1],
            input[i + 1][0],
            input[i + 1][1],
            input[i + 2][0],
            input[i + 2][1],
        );
    }

    println!("Day 13.1: {}", part1);
    println!("Day 13.1: {}", part2);
    println!("{:?}", gcd_extended(15, 21));
}
