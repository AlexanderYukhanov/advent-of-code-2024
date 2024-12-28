use std::collections::{HashMap, HashSet};
use std::io::BufRead;

#[derive(Default)]
pub struct Graph {
    adjustment_list: HashMap<i32, Vec<i32>>,
}

impl Graph {
    pub fn new() -> Self {
        Graph::default()
    }

    pub fn add_edge(&mut self, from: i32, to: i32) {
        self.adjustment_list.entry(from).or_default().push(to);
    }

    pub fn check_order(&self, nodes: &[i32]) -> bool {
        let mut prohibited: HashSet<i32> = HashSet::new();
        for node in nodes {
            if !prohibited.insert(*node) {
                return false;
            }
            if let Some(nodes) = self.adjustment_list.get(node) {
                prohibited.extend(nodes);
            }
        }
        true
    }

    pub fn arrange(&self, nodes: &[i32]) -> i32 {
        let mut nodes_set: HashSet<i32> = HashSet::with_capacity(nodes.len());
        nodes_set.extend(nodes);
        *nodes.into_iter().find(|node|
            if let Some(innodes) = self.adjustment_list.get(node) {
                nodes.len() / 2 == innodes.into_iter().filter(|n| nodes_set.contains(n)).count()
            } else {
                false
            } 
        ).unwrap()
    }
}

fn main() {
    let mut graph = Graph::new();
    let reader = std::io::BufReader::new(std::fs::File::open("input.txt").unwrap());
    let mut result = 0;
    let mut result2 = 0;
    reader.lines().for_each(|line| match line.unwrap() {
        value if value.contains("|") => {
            let (before, after) = value.split_once("|").unwrap();
            let before: i32 = before.parse().unwrap();
            let after: i32 = after.parse().unwrap();
            graph.add_edge(after, before);
        }
        value if value.is_empty() => {}
        value => {
            let pages: Vec<i32> = value
                .split(",")
                .into_iter()
                .map(|s| s.parse().unwrap())
                .collect();
            if graph.check_order(&pages) {
                result += pages[pages.len() / 2];
            } else {
                result2 += graph.arrange(&pages);
            }
        }
    });
    println!("Day 5.1: {}", result);
    println!("Day 5.2: {}", result2);
}
