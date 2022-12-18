use std::collections::HashSet;
use std::io::{stdin, BufRead};

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
struct Pos {
    x: i32,
    y: i64,
}

impl std::ops::Add<Pos> for Pos {
    type Output = Pos;

    fn add(self, rhs: Pos) -> Self::Output { Pos { x: self.x + rhs.x, y: self.y + rhs.y } }
}

struct Block {
    pieces: Vec<Pos>,
    width: i32,
}

fn collides(world: &HashSet<Pos>, block: &Block, offset: &Pos) -> bool {
    offset.x < 0
        || offset.x + block.width > 7
        || offset.y < 0
        || block.pieces.iter().map(|p| *p + *offset).any(|p| world.contains(&p))
}

#[allow(dead_code)]
fn print_world(world: &HashSet<Pos>, height: i64, len: i64) {
    for row in (height - len)..height {
        print!("{:-10}|", row);
        for col in 0..7 {
            print! {"{}", if world.contains(&Pos{x: col, y: row}) { "#" } else { "." }}
        }
        println!("|");
    }
}

fn gc(world: &mut HashSet<Pos>, height: i64) { world.retain(|x| x.y > (height - 100i64)) }

fn main() {
    let blocks = vec![
        Block {
            pieces: vec![
                Pos { x: 0, y: 0 },
                Pos { x: 1, y: 0 },
                Pos { x: 2, y: 0 },
                Pos { x: 3, y: 0 },
            ],
            width: 4,
        },
        Block {
            pieces: vec![
                Pos { x: 0, y: 1 },
                Pos { x: 1, y: 0 },
                Pos { x: 1, y: 1 },
                Pos { x: 1, y: 2 },
                Pos { x: 2, y: 1 },
            ],
            width: 3,
        },
        Block {
            pieces: vec![
                Pos { x: 0, y: 0 },
                Pos { x: 1, y: 0 },
                Pos { x: 2, y: 0 },
                Pos { x: 2, y: 1 },
                Pos { x: 2, y: 2 },
            ],
            width: 3,
        },
        Block {
            pieces: vec![
                Pos { x: 0, y: 0 },
                Pos { x: 0, y: 1 },
                Pos { x: 0, y: 2 },
                Pos { x: 0, y: 3 },
            ],
            width: 1,
        },
        Block {
            pieces: vec![
                Pos { x: 0, y: 0 },
                Pos { x: 0, y: 1 },
                Pos { x: 1, y: 0 },
                Pos { x: 1, y: 1 },
            ],
            width: 2,
        },
    ];

    let fall = Pos { x: 0, y: -1 };

    let line = stdin().lock().lines().next().unwrap().unwrap();
    let mut jetstream = line
        .chars()
        .map(|c| match c {
            '<' => Pos { x: -1, y: 0 },
            '>' => Pos { x: 1, y: 0 },
            _ => panic!("Invalid jet"),
        })
        .cycle();

    let mut world = HashSet::new();
    let mut height = 0;
    let gcfreq = 1024 * 1024;

    let chunk_size = line.len() * blocks.len();
    let warmup_chunks = 2;
    let mut rec = false;
    let mut fprint = Vec::new();
    let mut check_at_index: i32 = -1;
    let mut period_start_height = 0;
    let mut period_end_height = 0;
    let mut skipped_height = 0;

    let mut limit = 1000000000000;
    for (i, block) in blocks.iter().cycle().enumerate() {
        if i == limit {
            break;
        }
        if i == 2022 {
            println!("Step 1: {}", height);
        }

        if i % gcfreq == 0 {
            gc(&mut world, height);
        }

        if i % chunk_size == 0 && i / chunk_size >= warmup_chunks {
            if fprint.len() == 0 {
                period_start_height = height;
                rec = true;
            } else if skipped_height == 0 {
                period_end_height = height;
                check_at_index = 0;
            }
        }

        let prev_height = height;

        let mut block_pos = Pos { x: 2, y: height + 3 };
        loop {
            let after_jet = block_pos + jetstream.next().unwrap();

            if !collides(&world, &block, &after_jet) {
                block_pos = after_jet;
            }

            let after_fall = block_pos + fall;
            if !collides(&world, &block, &after_fall) {
                block_pos = after_fall;
            } else {
                block.pieces.iter().map(|p| *p + block_pos).for_each(|p| {
                    world.insert(p);
                    height = height.max(p.y + 1);
                });
                break;
            }
        }

        if rec {
            fprint.push(height - prev_height);
            if fprint.len() == chunk_size {
                rec = false;
            }
        }

        if check_at_index > -1 {
            if fprint[check_at_index as usize] != height - prev_height {
                check_at_index = -1;
                continue;
            }
            check_at_index += 1;
            if check_at_index == fprint.len() as i32 {
                check_at_index = -1;

                let i = i + 1;
                let period_size = i - warmup_chunks * chunk_size - chunk_size;
                let periods_to_skip = (limit - i) / period_size;
                let blocks_to_skip = period_size * periods_to_skip;
                limit -= blocks_to_skip;

                let period_height = period_end_height - period_start_height;
                skipped_height = periods_to_skip as i64 * period_height;
            }
        }
    }
    println!("Step 2: {}", height + skipped_height);
}
