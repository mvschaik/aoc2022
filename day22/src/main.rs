use core::ops::Range;
use std::collections::HashMap;
use std::io::{self, BufRead};
use std::ops;

#[derive(Clone, Copy, Debug, PartialEq, Hash, Eq)]
struct Pos {
    row: i32,
    col: i32,
}

impl Pos {
    fn turn(&self, dir: char) -> Pos {
        match dir {
            'L' => Pos { row: -self.col, col: self.row },
            'R' => Pos { row: self.col, col: -self.row },
            _ => panic!("Invalid direction {}", dir),
        }
    }
}

impl ops::Add<Pos> for Pos {
    type Output = Pos;
    fn add(self, rhs: Pos) -> Self::Output {
        Pos { row: self.row + rhs.row, col: self.col + rhs.col }
    }
}

impl ops::Mul<i32> for Pos {
    type Output = Pos;
    fn mul(self, rhs: i32) -> Self::Output { Pos { row: self.row * rhs, col: self.col * rhs } }
}

#[derive(Clone)]
struct World {
    data: Vec<Vec<char>>,
    col_ranges: Vec<Range<usize>>,
    row_ranges: Vec<Range<usize>>,
    pos: Pos,
    dir: Pos,
    edge_size: i32,
    connections: BiMap<Edge>,
}

#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]
enum Side {
    N,
    W,
    S,
    E,
}

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
struct Edge {
    side: Side,
    quadrant: Pos,
}

#[derive(Clone)]
struct BiMap<T>
where
    T: Eq + std::hash::Hash + Clone,
{
    map: HashMap<T, T>,
}

impl<T> BiMap<T>
where
    T: Eq + std::hash::Hash + Clone,
{
    fn new() -> BiMap<T> { BiMap { map: HashMap::new() } }
    fn insert(&mut self, k: T, v: T) {
        assert!(self.map.insert(k.clone(), v.clone()).is_none());
        assert!(self.map.insert(v, k).is_none());
    }
    fn get(&self, k: &T) -> Option<&T> { self.map.get(k) }
}

fn clamp(mut n: i32, range: &Range<usize>) -> i32 {
    while n < range.start as i32 {
        n += (range.end - range.start) as i32;
    }
    while n >= range.end as i32 {
        n -= (range.end - range.start) as i32;
    }
    n
}

impl World {
    fn new(world: Vec<Vec<char>>) -> World {
        let mut col_ranges = Vec::new();
        let mut longest_len = 0;

        for line in &world {
            let start = line.iter().position(|c| *c != EMPTY).unwrap();
            let end = line[start..].iter().position(|c| *c == EMPTY).unwrap_or(line.len());
            col_ranges.push(start..end);
            longest_len = longest_len.max(line.len());
        }

        let mut row_ranges = Vec::new();
        for col in 0..longest_len {
            let start =
                world.iter().position(|row| row.get(col).unwrap_or(&EMPTY) != &EMPTY).unwrap();
            let end = start
                + world[start..]
                    .iter()
                    .position(|row| row.get(col).unwrap_or(&EMPTY) == &EMPTY)
                    .unwrap_or(world.len() - start);
            row_ranges.push(start..end);
        }

        let initial_col = col_ranges[0].start as i32;

        // 4x3 or 3x4 quadrants.
        let edge_size = row_ranges.len().max(col_ranges.len()) as i32 / 4;

        // Ugh...
        let mut connections = BiMap::new();
        if edge_size == 4 {
            connections.insert(
                Edge { quadrant: Pos { row: 0, col: 2 }, side: Side::W },
                Edge { quadrant: Pos { row: 1, col: 1 }, side: Side::N },
            );
            connections.insert(
                Edge { quadrant: Pos { row: 0, col: 2 }, side: Side::N },
                Edge { quadrant: Pos { row: 1, col: 0 }, side: Side::N },
            );
            connections.insert(
                Edge { quadrant: Pos { row: 0, col: 2 }, side: Side::E },
                Edge { quadrant: Pos { row: 2, col: 3 }, side: Side::E },
            );
            connections.insert(
                Edge { quadrant: Pos { row: 1, col: 0 }, side: Side::W },
                Edge { quadrant: Pos { row: 2, col: 3 }, side: Side::S },
            );
            connections.insert(
                Edge { quadrant: Pos { row: 1, col: 0 }, side: Side::S },
                Edge { quadrant: Pos { row: 2, col: 2 }, side: Side::S },
            );
            connections.insert(
                Edge { quadrant: Pos { row: 1, col: 1 }, side: Side::S },
                Edge { quadrant: Pos { row: 2, col: 2 }, side: Side::W },
            );
            connections.insert(
                Edge { quadrant: Pos { row: 1, col: 2 }, side: Side::E },
                Edge { quadrant: Pos { row: 2, col: 3 }, side: Side::N },
            );
        } else if edge_size == 50 {
            connections.insert(
                Edge { quadrant: Pos { row: 0, col: 1 }, side: Side::W },
                Edge { quadrant: Pos { row: 2, col: 0 }, side: Side::W },
            );
            connections.insert(
                Edge { quadrant: Pos { row: 0, col: 1 }, side: Side::N },
                Edge { quadrant: Pos { row: 3, col: 0 }, side: Side::W },
            );
            connections.insert(
                Edge { quadrant: Pos { row: 0, col: 2 }, side: Side::N },
                Edge { quadrant: Pos { row: 3, col: 0 }, side: Side::S },
            );
            connections.insert(
                Edge { quadrant: Pos { row: 0, col: 2 }, side: Side::E },
                Edge { quadrant: Pos { row: 2, col: 1 }, side: Side::E },
            );
            connections.insert(
                Edge { quadrant: Pos { row: 0, col: 2 }, side: Side::S },
                Edge { quadrant: Pos { row: 1, col: 1 }, side: Side::E },
            );
            connections.insert(
                Edge { quadrant: Pos { row: 1, col: 1 }, side: Side::W },
                Edge { quadrant: Pos { row: 2, col: 0 }, side: Side::N },
            );
            connections.insert(
                Edge { quadrant: Pos { row: 3, col: 0 }, side: Side::E },
                Edge { quadrant: Pos { row: 2, col: 1 }, side: Side::S },
            );
        }

        World {
            data: world,
            col_ranges,
            row_ranges,
            edge_size,
            pos: Pos { row: 0, col: initial_col },
            dir: Pos { row: 0, col: 1 },
            connections,
        }
    }

    fn mv(&mut self, dist: i32) {
        for _ in 0..dist {
            let mut next_pos = self.pos + self.dir;
            if self.dir.col == 0 {
                next_pos.row = clamp(next_pos.row, &self.row_ranges[next_pos.col as usize]);
            } else {
                next_pos.col = clamp(next_pos.col, &self.col_ranges[next_pos.row as usize]);
            }
            if self.get(&next_pos) != OPEN {
                break;
            }
            self.pos = next_pos;
        }
    }

    fn mv2(&mut self, dist: i32) {
        for _ in 0..dist {
            let mut next_pos = self.pos + self.dir;
            let quadrant =
                Pos { row: self.pos.row / self.edge_size, col: self.pos.col / self.edge_size };
            let index_on_side;
            let side = if self.dir.col == 0 {
                index_on_side = next_pos.col % self.edge_size;
                let range = &self.row_ranges[next_pos.col as usize];
                if next_pos.row < range.start as i32 {
                    Some(Side::N)
                } else if next_pos.row >= range.end as i32 {
                    Some(Side::S)
                } else {
                    None
                }
            } else {
                index_on_side = next_pos.row % self.edge_size;
                let range = &self.col_ranges[next_pos.row as usize];
                if next_pos.col < range.start as i32 {
                    Some(Side::W)
                } else if next_pos.col >= range.end as i32 {
                    Some(Side::E)
                } else {
                    None
                }
            };

            if side.is_some() {
                let side = side.unwrap();
                let connection = self.connections.get(&Edge { quadrant, side }).unwrap();
                next_pos = Pos {
                    row: connection.quadrant.row * self.edge_size,
                    col: connection.quadrant.col * self.edge_size,
                };
                match &connection.side {
                    Side::S => {
                        next_pos.row += self.edge_size - 1;
                    }
                    Side::E => {
                        next_pos.col += self.edge_size - 1;
                    }
                    _ => {}
                }
                match (&side, &connection.side) {
                    (Side::N, Side::W)
                    | (Side::W, Side::E)
                    | (Side::E, Side::W)
                    | (Side::S, Side::E) => next_pos.row += index_on_side,
                    (Side::N, Side::S)
                    | (Side::W, Side::N)
                    | (Side::E, Side::S)
                    | (Side::S, Side::N) => next_pos.col += index_on_side,
                    (Side::N, Side::E)
                    | (Side::W, Side::W)
                    | (Side::E, Side::E)
                    | (Side::S, Side::W) => next_pos.row += self.edge_size - 1 - index_on_side,
                    (Side::N, Side::N)
                    | (Side::W, Side::S)
                    | (Side::E, Side::N)
                    | (Side::S, Side::S) => next_pos.col += self.edge_size - 1 - index_on_side,
                }

                if self.get(&next_pos) != OPEN {
                    break;
                }
                match (&side, &connection.side) {
                    (Side::N, Side::E)
                    | (Side::W, Side::N)
                    | (Side::E, Side::S)
                    | (Side::S, Side::W) => self.turn('L'),
                    (Side::N, Side::W)
                    | (Side::W, Side::S)
                    | (Side::E, Side::N)
                    | (Side::S, Side::E) => self.turn('R'),
                    (Side::N, Side::S)
                    | (Side::W, Side::E)
                    | (Side::E, Side::W)
                    | (Side::S, Side::N) => {}
                    (Side::N, Side::N)
                    | (Side::W, Side::W)
                    | (Side::E, Side::E)
                    | (Side::S, Side::S) => {
                        self.turn('R');
                        self.turn('R');
                    }
                }
            } else {
                if self.get(&next_pos) != OPEN {
                    break;
                }
            }
            self.pos = next_pos;
        }
    }

    fn turn(&mut self, dir: char) { self.dir = self.dir.turn(dir); }

    fn get(&self, p: &Pos) -> char { self.data[p.row as usize][p.col as usize] }

    fn facing(&self) -> i32 {
        match self.dir {
            Pos { row: 0, col: 1 } => 0,
            Pos { row: 1, col: 0 } => 1,
            Pos { row: 0, col: -1 } => 2,
            Pos { row: -1, col: 0 } => 3,
            _ => panic!("Invalid facing"),
        }
    }

    fn run(&mut self, course: &str, cube: bool) -> i32 {
        let mut it = course.chars().into_iter();
        let mut c = it.next();
        let mut dist = 0;
        loop {
            match c {
                Some(d) if !d.is_digit(10) => {
                    if cube {
                        self.mv2(dist)
                    } else {
                        self.mv(dist);
                    }
                    self.turn(d);
                    dist = 0;
                }
                Some(i) => {
                    dist *= 10;
                    dist += i.to_digit(10).unwrap() as i32;
                }
                None => {
                    if cube {
                        self.mv2(dist)
                    } else {
                        self.mv(dist);
                    }
                    break;
                }
            }
            c = it.next();
        }

        1000 * (self.pos.row + 1) + 4 * (self.pos.col + 1) + self.facing()
    }
}

static EMPTY: char = ' ';
static OPEN: char = '.';

fn main() {
    let stdin = io::stdin();
    let mut input = stdin.lock().lines();
    let mut world: Vec<Vec<char>> = Vec::new();
    loop {
        let line = input.next().unwrap().unwrap();

        if line.is_empty() {
            break;
        }

        world.push(line.chars().collect());
    }
    let world = World::new(world);
    let course = input.next().unwrap().unwrap();

    println!("Step 1: {}", world.clone().run(&course, false));
    println!("Step 2: {}", world.clone().run(&course, true));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_turn() {
        let p = Pos { row: 1, col: 0 };
        assert_eq!(p.turn('L'), Pos { row: 0, col: 1 });
        assert_eq!(p.turn('R'), Pos { row: 0, col: -1 });

        let p = Pos { row: 0, col: 1 };
        assert_eq!(p.turn('L'), Pos { row: -1, col: 0 });
        assert_eq!(p.turn('R'), Pos { row: 1, col: 0 });
    }

    #[test]
    fn test_clamp() {
        assert_eq!(clamp(10, &(5..12)), 10);
        assert_eq!(clamp(10, &(5..10)), 5);
        assert_eq!(clamp(10, &(5..9)), 6);
        assert_eq!(clamp(10, &(10..20)), 10);
        assert_eq!(clamp(9, &(10..20)), 19);
        assert_eq!(clamp(8, &(10..20)), 18);
    }

    #[test]
    fn test_mv2_w2n() {
        let mut world = small_world();
        world.pos = Pos { row: 2, col: 8 };
        world.dir = Pos { row: 0, col: -1 };

        world.mv2(1);
        assert_eq!(world.pos, Pos { row: 4, col: 6 });
        assert_eq!(world.dir, Pos { row: 1, col: 0 });
    }

    #[test]
    fn test_mv2_n2n() {
        let mut world = small_world();
        world.pos = Pos { row: 4, col: 1 };
        world.dir = Pos { row: -1, col: 0 };

        world.mv2(1);
        assert_eq!(world.pos, Pos { row: 0, col: 10 });
        assert_eq!(world.dir, Pos { row: 1, col: 0 });
    }

    #[test]
    fn test_mv2_n2w() {
        let mut world = small_world();
        world.pos = Pos { row: 4, col: 6 };
        world.dir = Pos { row: -1, col: 0 };

        world.mv2(1);
        assert_eq!(world.pos, Pos { row: 2, col: 8 });
        assert_eq!(world.dir, Pos { row: 0, col: 1 });
    }

    #[test]
    fn test_mv2_e2e() {
        let mut world = small_world();
        world.pos = Pos { row: 2, col: 11 };
        world.dir = Pos { row: 0, col: 1 };

        world.mv2(1);
        assert_eq!(world.pos, Pos { row: 9, col: 15 });
        assert_eq!(world.dir, Pos { row: 0, col: -1 });
    }

    fn small_world() -> World {
        World::new(vec![
            "        ....".chars().collect(),
            "        ....".chars().collect(),
            "        ....".chars().collect(),
            "        ....".chars().collect(),
            "............".chars().collect(),
            "............".chars().collect(),
            "............".chars().collect(),
            "............".chars().collect(),
            "        ........".chars().collect(),
            "        ........".chars().collect(),
            "        ........".chars().collect(),
            "        ........".chars().collect(),
        ])
    }
}
