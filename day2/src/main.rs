use std::io::{self, BufRead};

fn parse_line(line: &str) -> (&str, &str) {
    let mut parts = line.split_whitespace();
    (parts.next().unwrap(), parts.next().unwrap())
}

fn calc_score_part1(choices: (&str, &str)) -> i32 {
    match choices {
        ("A", "X") => 1 + 3, // rock - rock = draw
        ("A", "Y") => 2 + 6, // rock - paper = win
        ("A", "Z") => 3 + 0, // rock - scissors = lose
        ("B", "X") => 1 + 0, // paper - rock = lose
        ("B", "Y") => 2 + 3, // paper - paper = draw
        ("B", "Z") => 3 + 6, // paper - scissors = win
        ("C", "X") => 1 + 6, // scissors - rock = win
        ("C", "Y") => 2 + 0, // scissors - paper = lose
        ("C", "Z") => 3 + 3, // scissors - scissors = draw
        _ => panic!("Invalid input: {:?}", choices),
    }
}

fn calc_score_part2(choices: (&str, &str)) -> i32 {
    match choices {
        ("A", "X") => 3 + 0, // rock - lose = scissors
        ("A", "Y") => 1 + 3, // rock - draw = rock
        ("A", "Z") => 2 + 6, // rock - win = paper
        ("B", "X") => 1 + 0, // paper - lose = rock
        ("B", "Y") => 2 + 3, // paper - draw = paper
        ("B", "Z") => 3 + 6, // paper - win = scissors
        ("C", "X") => 2 + 0, // scissors - lose = paper
        ("C", "Y") => 3 + 3, // scissors - draw = scissors
        ("C", "Z") => 1 + 6, // scissors - win = rock
        _ => panic!("Invalid input: {:?}", choices),
    }
}

fn main() {
    let mut score1 = 0;
    let mut score2 = 0;

    for line in io::stdin().lock().lines() {
        let line = line.unwrap();
        let choices = parse_line(&line);
        score1 += calc_score_part1(choices);
        score2 += calc_score_part2(choices);
    }
    println!("Part 1: Score: {}", score1);
    println!("Part 2: Score: {}", score2);
}
