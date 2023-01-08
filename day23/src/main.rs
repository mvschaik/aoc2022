use std::collections::hash_map::Entry;
use std::collections::hash_set::Iter;
use std::collections::{HashMap, HashSet};
use std::io::stdin;
use std::ops;
use std::ops::Range;

#[derive(Eq, Hash, Debug, Copy, Clone, PartialEq)]
struct Pos {
    x: i32,
    y: i32,
}

impl ops::Add for Pos {
    type Output = Pos;
    fn add(self, rhs: Self) -> Self::Output { Pos { x: self.x + rhs.x, y: self.y + rhs.y } }
}

static N: Pos = Pos { x: 0, y: -1 };
static NE: Pos = Pos { x: 1, y: -1 };
static E: Pos = Pos { x: 1, y: 0 };
static SE: Pos = Pos { x: 1, y: 1 };
static S: Pos = Pos { x: 0, y: 1 };
static SW: Pos = Pos { x: -1, y: 1 };
static W: Pos = Pos { x: -1, y: 0 };
static NW: Pos = Pos { x: -1, y: -1 };

fn get_range<S>(it: Iter<S>, select: fn(&S) -> i32) -> Range<i32> {
    let mut min = i32::MAX;
    let mut max = i32::MIN;
    for el in it {
        let val = select(el);
        min = min.min(val);
        max = max.max(val);
    }
    min..(max + 1)
}

#[allow(dead_code)]
fn print_world(world: &HashSet<Pos>) {
    let xrange = get_range(world.iter(), |p| p.x);
    let yrange = get_range(world.iter(), |p| p.y);
    for y in yrange {
        for x in xrange.clone() {
            print!("{}", if world.contains(&Pos { x, y }) { '#' } else { '.' });
        }
        println!()
    }
}

fn main() {
    let all_directions: Vec<Vec<&Pos>> =
        vec![vec![&N, &NE, &NW], vec![&S, &SE, &SW], vec![&W, &NW, &SW], vec![&E, &NE, &SE]];
    let surroundings = vec![&N, &NE, &E, &SE, &S, &SW, &W, &NW];

    let mut world = HashSet::new();
    for (y, line) in stdin().lines().enumerate() {
        for (x, c) in line.unwrap().chars().enumerate() {
            if c == '#' {
                world.insert(Pos { x: x as i32, y: y as i32 });
            }
        }
    }

    let mut directions = all_directions.iter().cycle();

    for i in 0.. {
        let mut new_world = HashMap::new();
        let mut anything_moved = false;
        for x in &world {
            if surroundings.iter().copied().all(|d| !world.contains(&(*x + *d))) {
                new_world.insert(*x, *x);
                continue;
            }
            let mut moved = false;
            for _ in 0..all_directions.len() {
                let dir = directions.next().unwrap();
                let d0 = dir[0];
                if !moved && !dir.into_iter().copied().any(|d| world.contains(&(*x + *d))) {
                    match new_world.entry(*x + *d0) {
                        Entry::Vacant(entry) => {
                            entry.insert(*x);
                        }
                        Entry::Occupied(entry) => {
                            let prev_pos = *entry.get();
                            entry.remove();
                            new_world.insert(prev_pos, prev_pos);
                            new_world.insert(*x, *x);
                        }
                    }
                    moved = true;
                }
            }
            if !moved {
                new_world.insert(*x, *x);
            }
            anything_moved = true;
        }

        world = HashSet::from_iter(new_world.keys().copied());
        directions.next(); // Offset

        if i == 10 {
            let xrange = get_range(world.iter(), |p: &Pos| p.x);
            let yrange = get_range(world.iter(), |p: &Pos| p.y);
            let free: usize = xrange
                .map(|x| yrange.clone().filter(|y| !world.contains(&Pos { x, y: *y })).count())
                .sum();
            println!("Step 1: {free}");
        }

        if !anything_moved {
            println!("Step 2: {}", i + 1);
            break;
        }
    }
}
