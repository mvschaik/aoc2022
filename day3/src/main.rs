use std::collections::HashSet;
use std::io::{self, BufRead};

fn score(c: &char) -> u32 {
    match c {
        'a'..='z' => 1 + (*c as u32) - ('a' as u32),
        'A'..='Z' => 27 + (*c as u32) - ('A' as u32),
        _ => panic!("Invalid character"),
    }
}

fn bag(s: &str) -> HashSet<u32> {
    s.chars().map(|c| score(&c)).collect()
}

fn main() {
    let mut total1 = 0;
    let mut total2 = 0;

    let mut common: HashSet<u32> = HashSet::new();

    for (i, line) in io::stdin().lock().lines().enumerate() {
        let line = line.unwrap();

        if i % 3 == 0 {
            // New elf
            common = bag(&line);
        } else {
            common = common.intersection(&bag(&line)).map(|x| *x).collect();
        }

        let (c1, c2) = line.split_at(line.len() / 2);
        total1 += bag(c1).intersection(&bag(c2)).sum::<u32>();

        if i % 3 == 2 {
            total2 += common.iter().sum::<u32>();
        }
    }
    println!("Step1: total: {}", total1);
    println!("Step2: total: {}", total2);
}
