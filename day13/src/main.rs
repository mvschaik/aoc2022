use std::cmp::Ordering;
use std::io::{self, BufRead};

#[derive(Debug, PartialEq, Clone, Eq, Ord)]
enum El {
    Num(u32),
    List(Vec<El>),
}

use El::*;

macro_rules! list {
    ($expr:expr) => {
        List(vec![$expr])
    };
}

impl PartialOrd for El {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Num(a), Num(b)) => a.partial_cmp(b),
            (Num(_), List(_)) => list!(self.clone()).partial_cmp(other),
            (List(_), Num(_)) => self.partial_cmp(&list!(other.clone())),
            (List(aa), List(bb)) => match (aa.is_empty(), bb.is_empty()) {
                (true, true) => Some(Ordering::Equal),
                (false, true) => Some(Ordering::Greater),
                (true, false) => Some(Ordering::Less),
                (false, false) => match aa[0].partial_cmp(&bb[0]).unwrap() {
                    Ordering::Equal => {
                        List(Vec::from(&aa[1..])).partial_cmp(&List(Vec::from(&bb[1..])))
                    }
                    _ => aa[0].partial_cmp(&bb[0]),
                },
            },
        }
    }
}

fn parse_el(mut s: &str) -> (El, &str) {
    match s.chars().nth(0).unwrap() {
        '[' => {
            let mut content = Vec::new();
            s = &s[1..];
            loop {
                if s.chars().nth(0).unwrap() == ']' {
                    break;
                }
                let (parsed, rest) = parse_el(s);
                s = rest;
                content.push(parsed);
                if s.chars().nth(0).unwrap() == ',' {
                    s = &s[1..];
                }
            }
            (List(content), &s[1..])
        }
        _ => {
            let mut n = 0;
            while let Some(c) = s.chars().nth(0).unwrap().to_digit(10) {
                n *= 10;
                n += c;
                s = &s[1..];
            }
            (Num(n), s)
        }
    }
}

fn main() {
    let mut result1 = 0;

    let divider1 = list!(list!(Num(2)));
    let divider2 = list!(list!(Num(6)));
    let mut all_packets = vec![divider1.clone(), divider2.clone()];

    let mut lines = io::stdin().lock().lines();
    let mut index = 0;
    loop {
        index += 1;
        let (l1, _) = parse_el(&lines.next().unwrap().unwrap());
        let (l2, _) = parse_el(&lines.next().unwrap().unwrap());
        if l1 < l2 {
            result1 += index;
        }

        all_packets.push(l1);
        all_packets.push(l2);

        if lines.next().is_none() {
            break;
        }
    }
    println!("Part 1: {}", result1);

    all_packets.sort();
    let mut result2 = 1;
    for (i, el) in all_packets.iter().enumerate() {
        if *el == divider1 || *el == divider2 {
            result2 *= i + 1
        }
    }
    println!("Part 2: {}", result2);
}
