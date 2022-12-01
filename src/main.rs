use std::collections::BinaryHeap;
use std::io::{self, BufRead};

fn main() {
    let mut heap = BinaryHeap::new();
    let mut cur_elf = 0;

    for line in io::stdin().lock().lines() {
        let line = line.unwrap();
        if line == "" {
            heap.push(cur_elf);
            cur_elf = 0;
            continue;
        }
        cur_elf += line.parse::<i32>().unwrap();
    }
    heap.push(cur_elf);

    let mut top3 = 0;
    top3 += heap.pop().unwrap();
    println!("Step 1: max is {}", top3);
    top3 += heap.pop().unwrap();
    top3 += heap.pop().unwrap();
    println!("Step 2: top3 is {}", top3);
}
