use std::collections::{HashMap, HashSet, VecDeque};

fn combine(
    index: usize,
    allowed: &HashSet<String>,
    nodes: &Vec<String>,
    adjlist: &HashMap<String, Vec<String>>,
) -> HashSet<String> {
    let mut edg = adjlist
        .get(&nodes[index])
        .unwrap()
        .iter()
        .map(|v| v.to_owned())
        .collect::<HashSet<_>>();
    edg.insert(nodes[index].to_owned());
    allowed
        .intersection(&edg)
        .into_iter()
        .map(|v| (*v).to_owned())
        .collect::<HashSet<_>>()
}

fn recur(
    start: usize,
    allowed: &HashSet<String>,
    required: &Vec<String>,
    nodes: &Vec<String>,
    adjlist: &HashMap<String, Vec<String>>,
) -> (usize, Vec<String>) {
    if start == 0 {
        println!("{} {:?} {:?}", start, allowed, required);
    }
    if start == nodes.len() {
        return (0, vec![]);
    }
    let without = recur(start + 1, &allowed, required, nodes, adjlist);
    if !allowed.contains(&nodes[start]) {
        return without;
    }

    let updated = combine(start, &allowed, nodes, adjlist);
    for req in required.iter() {
        if !updated.contains(req) {
            return without;
        }
    }

    let mut required = required.clone();
    required.push(nodes[start].to_owned());
    let with = recur(start+1, &updated, &required, nodes, adjlist);
    if with.0 + 1 > without.0 {
        let mut withme = with.1.clone();
        withme.push(nodes[start].to_owned());
        return (with.0 + 1, withme);
    }
    return without;
}

fn dfs(adjlist: &HashMap<String, Vec<String>>) -> usize {
    let mut visited = HashSet::new();
    let mut max = 0;
    for head in adjlist.keys() {
        if !visited.insert(head.to_owned()) {
            continue;
        }
        let mut size = 1;
        let mut pending = VecDeque::new();
        pending.push_back(head.to_owned());
        while let Some(node) = pending.pop_front() {
            for edge in adjlist.get(&node).unwrap() {
                if visited.insert(edge.to_owned()) {
                    pending.push_back(edge.to_owned());
                    size += 1;
                }
            }
        }
        if size > max {
            max = size;
        }
    }
    max
}

fn main() {
    let mut adjlist = HashMap::new();
    std::fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .for_each(|line| {
            if let Some((lhs, rhs)) = line.split_once("-") {
                adjlist
                    .entry(lhs.to_owned())
                    .or_insert(vec![])
                    .push(rhs.to_owned());
                adjlist
                    .entry(rhs.to_owned())
                    .or_insert(vec![])
                    .push(lhs.to_owned());
            }
        });

    let mut best = (0, vec![]);
    for (from, endges) in adjlist.iter() {
        let mut allowed = endges.clone();
        allowed.push(from.to_owned());
        let cur = recur(
            0,
            &allowed.iter().map(|v| v.to_owned()).collect::<HashSet<_>>(),
            &vec![],
            &allowed,
            &adjlist,
        );
        if cur.0 > best.0 {
            best = cur;
        }
    }
    println!("Best: {:?}", best);
    best.1.sort();
    let part2 = best.1.join(",");
    println!("Day 23.2: {}", part2);

    let mut collected = HashSet::new();
    for (fst, edges) in adjlist.iter() {
        if !fst.starts_with("t") {
            continue;
        }
        if edges.len() < 2 {
            continue;
        }
        for e in edges.iter() {
            if let Some(second) = adjlist.get(e) {
                for third in second.iter() {
                    if edges.contains(third) {
                        let mut key = vec![fst.to_owned(), e.to_owned(), third.to_owned()];
                        key.sort();
                        collected.insert(key.join("-"));
                    }
                }
            }
        }
    }

    println!("Day 23.1: {}", collected.len());
}
