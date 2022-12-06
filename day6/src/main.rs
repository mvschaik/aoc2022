use std::collections::HashSet;
use std::io::{self, BufRead};

fn marker_pos(s: &str, marker_len: usize) -> i32 {
    for (i, seq) in s.chars().collect::<Vec<char>>().windows(marker_len).enumerate() {
        let set: HashSet<&char> = HashSet::from_iter(seq);
        if set.len() == marker_len {
            return (i + marker_len).try_into().unwrap();
        }
    }
    return 0;
}

fn main() {
    for line in io::stdin().lock().lines() {
        let line = line.unwrap();
        println!("Line: {}", line);
        println!("Step 1: position {}", marker_pos(&line, 4));
        println!("Step 2: position {}", marker_pos(&line, 14));
    }
}
