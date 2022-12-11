use num::Integer;
use regex::Regex;
use std::io;

type ItemType = i64;
type State = Vec<Vec<ItemType>>;

enum Op {
    Add(ItemType),
    Mul(ItemType),
    Square,
}

struct Monkey {
    op: Op,
    test: ItemType,
    if_true: usize,
    if_false: usize,
}

fn parse_monkeys(input: Vec<String>) -> Result<(Vec<Monkey>, State), Box<dyn std::error::Error>> {
    let monkey_re = Regex::new(r"Monkey \d+:")?;
    let starting_re = Regex::new(r" {2}Starting items: (.*)")?;
    let operation_re = Regex::new(r" {2}Operation: new = old (\+ \d+|\* \d+|\* old)")?;
    let test_re = Regex::new(r" {2}Test: divisible by (\d+)")?;
    let true_re = Regex::new(r" {4}If true: throw to monkey (\d+)")?;
    let false_re = Regex::new(r" {4}If false: throw to monkey (\d+)")?;

    let mut monkeys = Vec::new();
    let mut state = Vec::new();

    let mut iter = input.iter();
    loop {
        match iter.next() {
            None => break,
            Some(header) => assert!(monkey_re.is_match(header)),
        }
        let sm = starting_re.captures(iter.next().unwrap()).unwrap();
        state.push(sm[1].split(",").map(|s| s.trim().parse().unwrap()).collect());
        let op_matches = operation_re.captures(iter.next().unwrap()).unwrap();
        let op_parts: Vec<&str> = op_matches[1].split_whitespace().collect();
        let test_matches = test_re.captures(iter.next().unwrap()).unwrap();
        let true_matches = true_re.captures(iter.next().unwrap()).unwrap();
        let false_matches = false_re.captures(iter.next().unwrap()).unwrap();
        iter.next(); // newline

        monkeys.push(Monkey {
            op: match op_parts[0] {
                "*" => match op_parts[1] {
                    "old" => Op::Square,
                    other => Op::Mul(other.parse()?),
                },
                "+" => Op::Add(op_parts[1].parse()?),
                _ => panic!("Invalid operator"),
            },
            test: test_matches[1].parse()?,
            if_true: true_matches[1].parse()?,
            if_false: false_matches[1].parse()?,
        });
    }

    Ok((monkeys, state))
}

fn run<F>(monkeys: &Vec<Monkey>, mut state: State, rounds: usize, manage: F) -> ItemType
where
    F: Fn(ItemType) -> ItemType,
{
    let mut inspections = vec![0; state.len()];
    for _ in 0..rounds {
        for (i, monkey) in monkeys.iter().enumerate() {
            for item in state[i].clone() {
                inspections[i] += 1;
                let item = match monkey.op {
                    Op::Add(n) => item + n,
                    Op::Mul(n) => item * n,
                    Op::Square => item * item,
                };
                let item = manage(item);
                let dest_monkey =
                    if item % monkey.test == 0 { monkey.if_true } else { monkey.if_false };
                state[dest_monkey].push(item);
            }
            state[i].clear()
        }
    }
    inspections.sort();
    inspections.reverse();
    inspections[0..2].iter().product()
}

fn main() {
    let (monkeys, initial_state) =
        parse_monkeys(io::stdin().lines().map(|l| l.unwrap()).collect()).unwrap();

    println!("Part 1: {}", run(&monkeys, initial_state.clone(), 20, |i| i / 3));

    let lcm: ItemType = monkeys.iter().map(|m| m.test).reduce(|a, b| a.lcm(&b)).unwrap();
    println!("Part 2: {}", run(&monkeys, initial_state, 10000, |i| i % lcm));
}
