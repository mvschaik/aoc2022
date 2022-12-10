use std::io::{self, BufRead};

fn main() {
    let mut x = 1;

    // For part 1.
    let mut result = 0;
    let mut cycle = 0;
    let mut interesting = 20;
    let interval = 40;

    // For part 2.
    let mut current_line = String::new();
    let mut draw = |x| {
        let pos = current_line.len() as i32;
        current_line += if x - 1 <= pos && pos <= x + 1 { "#" } else { "." };
        if current_line.len() == 40 {
            println!("{}", current_line);
            current_line = String::new();
        }
    };

    for line in io::stdin().lock().lines() {
        let line = line.unwrap();
        let parts: Vec<&str> = line.split_whitespace().collect();
        match parts[0] {
            "noop" => {
                cycle += 1;
                draw(x);
            }
            "addx" => {
                cycle += 2;
                draw(x);
                draw(x);
                if cycle >= interesting {
                    result += x * interesting;
                    interesting += interval
                }
                x += parts[1].parse::<i32>().unwrap();
            }
            _ => panic!("Invalid instruction"),
        }
    }
    println!("Part 1: {}", result);
}
