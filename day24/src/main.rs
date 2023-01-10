use std::collections::{HashSet, VecDeque};
use std::io::stdin;

type World = Vec<Vec<Loc>>;

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
struct Pos {
    row: usize,
    col: usize,
}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
enum Dir {
    N,
    W,
    S,
    E,
}

#[derive(Debug, PartialEq, Clone)]
enum Loc {
    Wall,
    Blizzards(HashSet<Dir>),
}

use Loc::*;

fn to_loc(c: char) -> Loc {
    match c {
        '#' => Wall,
        '.' => Blizzards(HashSet::new()),
        '>' => Blizzards(HashSet::from([Dir::E])),
        '<' => Blizzards(HashSet::from([Dir::W])),
        'v' => Blizzards(HashSet::from([Dir::S])),
        '^' => Blizzards(HashSet::from([Dir::N])),
        _ => panic!("Invalid character {}", c),
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
struct State {
    pos: Pos,
    time: usize,
}

#[allow(dead_code)]
fn print_world(world: &World, pos: &Pos) {
    for (row, line) in world.iter().enumerate() {
        for (col, loc) in line.iter().enumerate() {
            print!(
                "{}",
                if pos == &(Pos { row, col }) {
                    "E".to_string()
                } else {
                    match loc {
                        Wall => "#".to_string(),
                        Blizzards(s) => {
                            if s.len() > 1 {
                                format!("{}", s.len())
                            } else {
                                match s.iter().next() {
                                    Some(Dir::N) => "^",
                                    Some(Dir::W) => "<",
                                    Some(Dir::E) => ">",
                                    Some(Dir::S) => "v",
                                    None => ".",
                                }
                                .to_string()
                            }
                        }
                    }
                }
            );
        }
        println!();
    }
}

fn storm(world: &World) -> World {
    let mut new_world: World = Vec::new();
    let height = world.len();
    let width = world[0].len();
    for row in world {
        new_world.push(
            row.iter().map(|l| if *l == Wall { Wall } else { Blizzards(HashSet::new()) }).collect(),
        );
    }
    for (y, row) in world.iter().enumerate() {
        for (x, loc) in row.iter().enumerate() {
            match loc {
                Wall => {}
                Blizzards(directions) => {
                    for dir in directions {
                        match dir {
                            Dir::N => {
                                if let Blizzards(s) =
                                    &mut new_world[if y == 1 { height - 2 } else { y - 1 }][x]
                                {
                                    s.insert(Dir::N);
                                }
                            }
                            Dir::W => {
                                if let Blizzards(s) =
                                    &mut new_world[y][if x == 1 { width - 2 } else { x - 1 }]
                                {
                                    s.insert(Dir::W);
                                }
                            }
                            Dir::S => {
                                if let Blizzards(s) =
                                    &mut new_world[if y == height - 2 { 1 } else { y + 1 }][x]
                                {
                                    s.insert(Dir::S);
                                }
                            }
                            Dir::E => {
                                if let Blizzards(s) =
                                    &mut new_world[y][if x == width - 2 { 1 } else { x + 1 }]
                                {
                                    s.insert(Dir::E);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    new_world
}

fn moves(p: &Pos) -> Vec<Pos> {
    let mut result: Vec<Pos> = Vec::new();
    result.push(p.clone());
    if p.row > 1 {
        result.push(Pos { row: p.row - 1, ..*p });
    }
    if p.col > 1 {
        result.push(Pos { col: p.col - 1, ..*p });
    }
    result.push(Pos { row: p.row + 1, ..*p });
    result.push(Pos { col: p.col + 1, ..*p });
    result
}

fn main() {
    let mut initial_world: World = Vec::new();
    for line in stdin().lines() {
        initial_world.push(line.unwrap().chars().map(&to_loc).collect());
    }

    let world_height = initial_world.len();
    let world_width = initial_world[0].len();

    let origin = Pos { row: 0, col: initial_world[0].iter().position(|l| *l != Wall).unwrap() };

    let mut worlds = vec![initial_world];

    let mut step1done = false;

    let mut todo = VecDeque::new();
    todo.push_front(State { pos: origin, time: 0 });
    let mut visited = HashSet::new();
    while let Some(s) = todo.pop_back() {
        if !visited.insert(s.clone()) {
            continue;
        }

        if !step1done && s.pos.row == world_height - 1 {
            println!("Step 1: {}", s.time);
            step1done = true;
        } else if s.pos.row == 3 * (world_height - 1) {
            println!("Step 2: {}", s.time);
            break;
        }

        while worlds.len() < s.time + 2 {
            worlds.push(storm(worlds.last().unwrap()));
        }
        let world = &worlds[s.time + 1];

        for m in moves(&s.pos) {
            if m.col >= world_width {
                continue;
            }

            let row = if m.row < world_height {
                m.row
            } else if m.row < 2 * world_height - 1 {
                2 * (world_height - 1) - m.row
            } else {
                m.row + 2 - 2 * world_height
            };

            match &world[row][m.col] {
                Blizzards(b) if b.is_empty() => todo.push_front(State { pos: m, time: s.time + 1 }),
                _ => {}
            }
        }
    }
}
