use std::io::{self, BufRead};
use std::ops::RangeInclusive;

fn parse_line(s: &str) -> Option<(RangeInclusive<i32>, RangeInclusive<i32>)> {
    let mut elves = s.split(",");
    let mut elf1 = elves.next()?.split("-");
    let mut elf2 = elves.next()?.split("-");
    Some((
        (elf1.next()?.parse::<i32>().unwrap()..=elf1.next()?.parse::<i32>().unwrap()),
        (elf2.next()?.parse::<i32>().unwrap()..=elf2.next()?.parse::<i32>().unwrap()),
    ))
}

fn main() {
    let mut step1 = 0;
    let mut step2 = 0;

    for line in io::stdin().lock().lines() {
        let line = line.unwrap();

        let (e1, e2) = parse_line(&line).unwrap();

        if (e1.contains(e2.start()) && e1.contains(e2.end()))
            || (e2.contains(e1.start()) && e2.contains(e1.end()))
        {
            step1 += 1;
        }

        if e1.contains(e2.start()) || e1.contains(e2.end()) || e2.contains(e1.start()) {
            step2 += 1;
        }
    }

    println!("Step1: {}", step1);
    println!("Step2: {}", step2);
}
