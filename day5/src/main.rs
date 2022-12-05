use regex::Regex;
use std::io::{self, BufRead};

type State = Vec<Vec<char>>;

fn main() {
    let stack_row_re = Regex::new(r"((\[(?P<crate>\w)\]|   ) ?)").unwrap();
    let move_re = Regex::new(r"move (?P<count>\d+) from (?P<from>\d+) to (?P<to>\d+)").unwrap();

    // 10 should be enough stacks for anyone!
    let mut state9000: State = vec![vec![]; 10];

    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // First parse initial state.
    while let Some(line) = lines.next() {
        let line = line.unwrap();
        if line.is_empty() {
            break;
        }

        for (i, group) in stack_row_re.captures_iter(&line).enumerate() {
            if let Some(c) = &group.name("crate") {
                state9000[i].push(c.as_str().chars().next().unwrap());
            }
        }
    }
    for line in &mut state9000 {
        line.reverse();
    }

    let mut state9001 = state9000.clone();

    // Parse moves.
    while let Some(line) = lines.next() {
        if let Some(group) = move_re.captures(&line.unwrap()) {
            let count: usize = group.name("count").unwrap().as_str().parse().unwrap();
            let from: usize = group.name("from").unwrap().as_str().parse().unwrap();
            let to: usize = group.name("to").unwrap().as_str().parse().unwrap();

            for _ in 0..count {
                let c = state9000[from - 1].pop().unwrap();
                state9000[to - 1].push(c);
            }

            let from_stack = &mut state9001[from - 1];
            let offset = from_stack.len() - count;
            let mut c = from_stack.drain(offset..).collect();
            state9001[to - 1].append(&mut c);
        }
    }

    let result1: String = state9000
        .iter()
        .map(|stack| stack.last())
        .filter_map(|c| c)
        .collect();
    let result2: String = state9001
        .iter()
        .map(|stack| stack.last())
        .filter_map(|c| c)
        .collect();
    println!("Step 1: result={}", result1);
    println!("Step 2: result={}", result2);
}
