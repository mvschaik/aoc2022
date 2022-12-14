use regex::Regex;
use std::collections::HashSet;
use std::io;

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
struct Coord {
    x: i32,
    y: i32,
}

impl Coord {
    fn to(&self, other: &Coord) -> Coord {
        Coord { x: self.x + (other.x - self.x).signum(), y: self.y + (other.y - self.y).signum() }
    }
}

struct World {
    occ: HashSet<Coord>,
    void: i32,
}

fn parse_world(input: Vec<String>) -> World {
    let rock_re = Regex::new(r"(?P<x>\d+),(?P<y>\d+)").unwrap();
    let mut occ = HashSet::new();
    let mut void = 0;
    for line in input {
        let points: Vec<Coord> = rock_re
            .captures_iter(&line)
            .map(|cap| Coord {
                x: cap.name("x").unwrap().as_str().parse().unwrap(),
                y: cap.name("y").unwrap().as_str().parse().unwrap(),
            })
            .collect();

        points.windows(2).for_each(|pair| {
            let mut pos = pair[0];
            loop {
                occ.insert(pos);
                void = void.max(pos.y);
                if pos == pair[1] {
                    break;
                }
                pos = pos.to(&pair[1]);
            }
        });
    }
    World { occ, void }
}

fn main() {
    let mut world = parse_world(io::stdin().lines().map(|s| s.unwrap()).collect());
    let mut sand_path = vec![Coord { x: 500, y: 0 }];
    let mut at_rest = 0;
    let mut result1 = None;
    while let Some(pos) = sand_path.last() {
        if pos.y > world.void {
            result1 = result1.or(Some(at_rest));
        }
        if pos.y > world.void + 1 {
            // Ad-hoc floor.
            world.occ.insert(sand_path.pop().unwrap());
            continue;
        }
        if !world.occ.contains(&Coord { x: pos.x, y: pos.y + 1 }) {
            sand_path.push(Coord { x: pos.x, y: pos.y + 1 });
        } else if !world.occ.contains(&Coord { x: pos.x - 1, y: pos.y + 1 }) {
            sand_path.push(Coord { x: pos.x - 1, y: pos.y + 1 });
        } else if !world.occ.contains(&Coord { x: pos.x + 1, y: pos.y + 1 }) {
            sand_path.push(Coord { x: pos.x + 1, y: pos.y + 1 });
        } else {
            world.occ.insert(sand_path.pop().unwrap());
            at_rest += 1;
        }
    }
    println!("Step 1: {}", result1.unwrap());
    println!("Step 2: {}", at_rest)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to() {
        assert_eq!(Coord { x: 10, y: 10 }.to(&Coord { x: 20, y: 10 }), Coord { x: 11, y: 10 });
        assert_eq!(Coord { x: 10, y: 10 }.to(&Coord { x: 2, y: 10 }), Coord { x: 9, y: 10 });
        assert_eq!(Coord { x: 10, y: 10 }.to(&Coord { x: 10, y: 1 }), Coord { x: 10, y: 9 });
        assert_eq!(Coord { x: 10, y: 10 }.to(&Coord { x: 10, y: 100 }), Coord { x: 10, y: 11 });
    }
}
