fn execute(mut a: u64, mut b: u64, mut c: u64, mem: &[u8]) -> String {
    let mut pc = 0;
    let mut result = String::new();
    loop {
        if pc > mem.len() - 2 {
            return result;
        }
        let cmd = mem[pc];
        let op = mem[pc + 1] as u64;
        pc += 2;
        let combo_op = match op {
            0..=3 => op,
            4 => a,
            5 => b,
            6 => c,
            _ => panic!("Unexpected combo {} at {}", op, pc - 1),
        };

        match cmd {
            0 => {
                a = a / (1 << combo_op);
            },
            1 => {
                b = b ^ op;
            },
            2 => {
                b = combo_op & 7;
            },
            3 => {
                if a != 0 {
                    pc = op as usize;
                }
            },
            4 => {
                b = b ^ c;
            },
            5 => {
                result += &format!("{},", combo_op & 7);
            },
            6 => {
                b = a / (1 << combo_op);
            },
            7 => {
                c = a / (1 << combo_op);
            },
            _ => panic!("Unexpected {} at {}", cmd, pc - 2),
        };
    }
}

fn hardcoded(a: u64) -> u8 {
    let b = a & 7;
    let b = b ^ 5;
    let c = a >> b;
    let b = b ^ 6;
    let b = b ^ c;
    return (b & 7) as u8;
}

fn fix(sofar: u64, mem: &[u8]) -> Option<u64> {
    if mem.len() == 0 {
        return Some(sofar);
    }

    for c in 0..=7 {
        let candidate = (sofar << 3) + c;
        if hardcoded(candidate) == *mem.last().unwrap() {
            if let Some(found) = fix(candidate, &mem[0..mem.len()-1]) {
                return Some(found);
            }
        }
    }
    None
}

fn main() {
    println!("Day 17.1 (test): {}", execute(729, 0, 0, &[0,1,5,4,3,0]));
    println!("Day 17.1: {}", execute(1140200324802857, 0, 0, &[2,4,1,5,7,5,1,6,0,3,4,3,5,5,3,0]));
    for a in 0..=7 {
        if let Some(found) = fix(a, &[2,4,1,5,7,5,1,6,0,3,4,3,5,5,3,0]) {
            println!("{}: {}", found, execute(found, 0, 0, &[2,4,1,5,7,5,1,6,0,3,4,3,5,5,3,0]));
        }
    }
}
