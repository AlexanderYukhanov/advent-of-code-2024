-op
use std::{
    collections::{HashMap, HashSet, VecDeque},
    future::pending,
    vec,
};

#[derive(Debug, Clone)]
struct Gate {
    inputs: Vec<bool>,
    op: isize, // 0 - AND, 1 - OR, 2 - XOR
    output: String,
}

impl Gate {
    fn new(op: &str, output: &str) -> Self {
        Gate {
            inputs: vec![],
            op: match op {
                "AND" => 0,
                "OR" => 1,
                "XOR" => 2,
                _ => panic!("Unexpected operation: {}", op),
            },
            output: output.to_owned(),
        }
    }

    fn calculate(&self) -> bool {
        if self.inputs.len() < 2 {
            panic!("Need exactly two inputs, but have {:?}", self.inputs);
        }
        match self.inputs[..] {
            [a, b] => match self.op {
                0 => a && b,
                1 => a || b,
                _ => a ^ b,
            },
            _ => false,
        }
    }
}

struct Part1Solver {
    inputs: HashMap<String, bool>,
    gates: Vec<Gate>,
    graph: HashMap<String, Vec<usize>>,
}

impl Part1Solver {
    fn new(
        inputs: HashMap<String, bool>,
        gates: Vec<Gate>,
        graph: HashMap<String, Vec<usize>>,
    ) -> Self {
        Self {
            inputs,
            gates,
            graph,
        }
    }

    fn solve(&mut self) -> u64 {
        let mut pending = self
            .inputs
            .iter()
            .map(|(k, &v)| (k.to_owned(), v))
            .collect::<VecDeque<_>>();
        while let Some((wire, state)) = pending.pop_front() {
            self.inputs.insert(wire.clone(), state);
            if let Some(edges) = self.graph.get(&wire) {
                for &edge in edges.iter() {
                    let gate = &mut self.gates[edge];
                    gate.inputs.push(state);
                    if gate.inputs.len() == 2 {
                        pending.push_back((gate.output.clone(), gate.calculate()));
                    }
                }
            }
        }
        let mut result = 0u64;
        for ind in 0..64 {
            if let Some(v) = self.inputs.get(&format!("z{:02}", ind)) {
                result |= (if *v { 1 } else { 0 }) << ind;
            }
        }
        result
    }
}

struct Part2Solver {
    gates: Vec<Gate>,
    graph: HashMap<String, Vec<usize>>,
    size: usize,
}

impl Part2Solver {
    fn new(gates: Vec<Gate>, graph: HashMap<String, Vec<usize>>, size: usize) -> Self {
        Self { gates, graph, size }
    }

    fn solve(&self) {
        let mut discovered: HashSet<String> = HashSet::new();
        for ind in 0..self.size {
            let mut gates = self.gates.clone();
            for g in gates.iter_mut() {
                match &g.output {
                    val if val == "ksv" => g.output = "z06".to_owned(),
                    val if val == "z06" => g.output = "ksv".to_owned(),
                    val if val == "nbd" => g.output = "kbs".to_owned(),
                    val if val == "kbs" => g.output = "nbd".to_owned(),
                    val if val == "tqq" => g.output = "z20".to_owned(),
                    val if val == "z20" => g.output = "tqq".to_owned(),
                    val if val == "ckb" => g.output = "a".to_owned(),
                    val if val == "z39" => g.output = "ckb".to_owned(),
                    _ => {}
                };
            }
            let mut states = HashMap::new();
            let mut pending: VecDeque<(String, bool)> = VecDeque::new();
            for n in 0..=ind {
                pending.push_back((format!("x{:02}", n), true)); //false));
                pending.push_back((format!("y{:02}", n), true));
            }

            let mut gates = gates.clone();
            while let Some((wire, state)) = pending.pop_front() {
                states.insert(wire.clone(), state);
                if let Some(edges) = self.graph.get(&wire) {
                    for &edge in edges.iter() {
                        let gate = &mut gates[edge];
                        gate.inputs.push(state);
                        if gate.inputs.len() == 2 {
                            pending.push_back((gate.output.clone(), gate.calculate()));
                        }
                    }
                }
            }

            for n in 0..self.size {
                if let Some(v) = states.get(&format!("z{:02}", n)) {
                    print!("{}", if *v { 1 } else { 0 });
                } else {
                    print!("?");
                }
            }
            println!();
            for gate in gates {
                if gate.inputs.len() >= 2 {
                    if discovered.insert(gate.output.clone()) {
                        println!(
                            "{} {}:{}->{}",
                            match gate.op  {
                                0 => "AND",
                                1 => "OR",
                                _ => "XOR",
                            },
                            gate.output,
                            gate.inputs.len(),
                            gate.calculate()
                        );
                    }
                }
            }
        }
    }
}

fn main() {
    let mut inputs: HashMap<String, bool> = HashMap::new();
    let mut gates: Vec<Gate> = vec![];
    let mut graph: HashMap<String, Vec<usize>> = HashMap::new();

    std::fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .for_each(|line| {
            if line.contains(":") {
                if let Some((wire, value)) = line.split_once(": ") {
                    inputs.insert(wire.to_owned(), value == "1");
                }
            }
            if line.contains("->") {
                if let [fst, op, snd, _, out] =
                    line.split_ascii_whitespace().collect::<Vec<_>>()[..]
                {
                    gates.push(Gate::new(op, out));
                    graph
                        .entry(fst.to_owned())
                        .or_insert(vec![])
                        .push(gates.len() - 1);
                    graph
                        .entry(snd.to_owned())
                        .or_insert(vec![])
                        .push(gates.len() - 1);
                }
            }
        });

    let mut solver = Part1Solver::new(inputs.clone(), gates.clone(), graph.clone());
    println!("Day 24.1: {}", solver.solve());
    let solver = Part2Solver::new(gates.clone(), graph.clone(), 45);
    solver.solve();
}
