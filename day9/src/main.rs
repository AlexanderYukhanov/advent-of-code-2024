use std::collections::BTreeMap;

#[derive(Clone, Copy)]
struct Block {
    id: i32,
    start: usize,
    length: usize,
}

impl Block {
    fn new(id: i32, start: usize, length: usize) -> Self {
        Self { id, start, length }
    }
    fn place(&self, length: usize, id: i32, start: usize) -> Option<(Self, Option<Self>)> {
        if self.id != -1 || self.start >= start || length > self.length {
            return None;
        }
        let placed = Self {
            id,
            start: self.start,
            length,
        };
        let remaining = if self.length > length {
            Some(Self {
                id: -1,
                start: self.start + length,
                length: self.length - length,
            })
        } else {
            None
        };
        Some((placed, remaining))
    }
}

fn main() {
    let map: Vec<i32> = std::fs::read_to_string("input.txt")
        .unwrap()
        .chars()
        .filter(|ch| ch.is_ascii_digit())
        .map(|ch| ch as i32 - '0' as i32)
        .collect();
    let mut expanded = Vec::<i32>::with_capacity(map.len() * 9);
    let mut id = 0;
    for (ind, &length) in map.iter().enumerate() {
        expanded
            .extend(std::iter::repeat(if ind % 2 == 0 { id } else { -1 }).take(length as usize));
        if ind % 2 == 0 {
            id += 1;
        }
    }

    let advance_left = |mut left: usize, expanded: &Vec<i32>| {
        while left < expanded.len() && expanded[left] != -1 {
            left += 1;
        }
        left
    };
    let advance_right = |mut right: usize, expanded: &Vec<i32>| {
        while right > 0 && expanded[right] == -1 {
            right -= 1;
        }
        right
    };
    let mut left = advance_left(0, &expanded);
    let mut right = advance_right(expanded.len() - 1, &expanded);
    while left < right {
        expanded[left] = expanded[right];
        expanded[right] = -1;
        left = advance_left(left, &expanded);
        right = advance_right(right, &expanded);
    }

    let mut checksum: u64 = 0;
    for (ind, value) in expanded.into_iter().enumerate() {
        if value == -1 {
            break;
        }
        checksum += ind as u64 * value as u64;
    }
    println!("Day 9.1: {}", checksum);

    let mut original = BTreeMap::<usize, Block>::new();
    let mut start = 0;
    let mut id = 0;
    for (ind, length) in map.iter().enumerate() {
        original.insert(
            start,
            Block::new(if ind % 2 == 1 { -1 } else { id }, start, *length as usize),
        );
        start += *length as usize;
        if ind % 2 == 0 {
            id += 1;
        }
    }

    let mut compressed = original.clone();
    for (start, block) in original.iter().rev() {
        if block.id == -1 {
            continue;
        }
        if let Some((placed, remaining)) = compressed.values().find_map(|candidate| candidate.place(block.length, block.id, block.start)) {
            compressed.remove(start);
            compressed.remove(&block.start);
            compressed.insert(placed.start, placed);
            if let Some(remaining) = remaining {
                compressed.insert(remaining.start, remaining);
            }
        }
    }

    let mut checksum: u64 = 0;
    for (start, block) in compressed {
        if block.id == -1 {
            continue;
        }
        checksum += (0..block.length)
            .map(|d| (start + d) as u64 * block.id as u64)
            .sum::<u64>();
    }

    println!("Day 9.2: {}", checksum);
}
