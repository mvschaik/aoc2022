use std::collections::HashMap;
use std::io::{self, BufRead};

fn main() {
    let mut cur_dir = vec![];
    let mut sizes = HashMap::new();
    for line in io::stdin().lock().lines() {
        let line = line.unwrap();
        let parts: Vec<&str> = line.split(" ").collect();
        if parts[0] == "$" {
            if parts[1] == "cd" {
                if parts[2] == "/" {
                    cur_dir.clear();
                } else if parts[2] == ".." {
                    cur_dir.pop();
                } else {
                    cur_dir.push(String::from(parts[2]));
                }
            }
            if parts[1] == "ls" {}
        } else {
            if parts[0] == "dir" {
                // Will be traversed later.
            } else {
                let size: i32 = parts[0].parse().unwrap();

                *sizes.entry("/".to_string()).or_insert(0) += size;
                let mut trav_dir = String::new();
                for dir in &cur_dir {
                    trav_dir += &("/".to_owned() + dir);
                    *sizes.entry(trav_dir.to_string()).or_insert(0) += size;
                }
            }
        }
    }

    let free_space = 70000000 - sizes.get("/").unwrap();
    let space_needed = 30000000 - free_space;

    let mut step1 = 0;
    let mut step2 = i32::MAX;
    for (_name, size) in &sizes {
        if *size <= 100000 {
            step1 += size;
        }
        if *size > space_needed {
            step2 = *size.min(&step2);
        }
    }
    println!("Step 1: {}", step1);
    println!("Step 2: {}", step2);
}
