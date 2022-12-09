use std::collections::HashSet;
use std::io::{self, BufRead};

#[derive(Copy, Clone, Hash, PartialEq, Eq, Debug)]
struct Pos {
    x: i32,
    y: i32,
}

fn follow(head: &Pos, tail: &Pos) -> Pos {
    let mut new_tail = tail.clone();
    if (head.x - tail.x).abs() > 1 || (head.y - tail.y).abs() > 1 {
        new_tail.x += (head.x - tail.x).signum();
        new_tail.y += (head.y - tail.y).signum();
    }
    new_tail
}

fn mv(knot: &Pos, dir: &str) -> Pos {
    match dir {
        "R" => Pos { x: knot.x + 1, y: knot.y },
        "L" => Pos { x: knot.x - 1, y: knot.y },
        "U" => Pos { x: knot.x, y: knot.y + 1 },
        "D" => Pos { x: knot.x, y: knot.y - 1 },
        _ => panic!("Invalid direction"),
    }
}

fn main() {
    let mut knots = vec![Pos { x: 0, y: 0 }; 10];
    let mut visited1 = HashSet::new();
    let mut visited2 = HashSet::new();

    visited1.insert(knots[1]);
    visited2.insert(*knots.last().unwrap());
    for line in io::stdin().lock().lines() {
        let line = line.unwrap();
        let parts: Vec<&str> = line.split_whitespace().collect();
        let direction = parts[0];
        let distance = parts[1].parse().unwrap();

        for _ in 0..distance {
            knots[0] = mv(&knots[0], direction);
            for i in 1..knots.len() {
                knots[i] = follow(&knots[i - 1], &knots[i]);
            }
            visited1.insert(knots[1]);
            visited2.insert(*knots.last().unwrap());
        }
    }
    println!("Step 1: visited={}", visited1.len());
    println!("Step 2: visited={}", visited2.len());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_follow_same_pos() {
        assert_eq!(follow(&Pos { x: 2, y: 2 }, &Pos { x: 2, y: 2 }), Pos { x: 2, y: 2 });
    }

    #[test]
    fn test_follow_touching() {
        assert_eq!(follow(&Pos { x: 2, y: 2 }, &Pos { x: 3, y: 2 }), Pos { x: 3, y: 2 });
        assert_eq!(follow(&Pos { x: 2, y: 2 }, &Pos { x: 1, y: 2 }), Pos { x: 1, y: 2 });
        assert_eq!(follow(&Pos { x: 2, y: 2 }, &Pos { x: 2, y: 3 }), Pos { x: 2, y: 3 });
        assert_eq!(follow(&Pos { x: 2, y: 2 }, &Pos { x: 2, y: 1 }), Pos { x: 2, y: 1 });
        assert_eq!(follow(&Pos { x: 2, y: 2 }, &Pos { x: 3, y: 3 }), Pos { x: 3, y: 3 });
        assert_eq!(follow(&Pos { x: 2, y: 2 }, &Pos { x: 1, y: 1 }), Pos { x: 1, y: 1 });
        assert_eq!(follow(&Pos { x: 2, y: 2 }, &Pos { x: 1, y: 3 }), Pos { x: 1, y: 3 });
        assert_eq!(follow(&Pos { x: 2, y: 2 }, &Pos { x: 3, y: 1 }), Pos { x: 3, y: 1 });
    }

    #[test]
    fn test_follow_straight() {
        assert_eq!(follow(&Pos { x: 2, y: 2 }, &Pos { x: 4, y: 2 }), Pos { x: 3, y: 2 });
        assert_eq!(follow(&Pos { x: 2, y: 2 }, &Pos { x: 0, y: 2 }), Pos { x: 1, y: 2 });
        assert_eq!(follow(&Pos { x: 2, y: 2 }, &Pos { x: 2, y: 4 }), Pos { x: 2, y: 3 });
        assert_eq!(follow(&Pos { x: 2, y: 2 }, &Pos { x: 2, y: 0 }), Pos { x: 2, y: 1 });
    }

    #[test]
    fn test_follow_diag() {
        assert_eq!(follow(&Pos { x: 2, y: 2 }, &Pos { x: 4, y: 3 }), Pos { x: 3, y: 2 });
        assert_eq!(follow(&Pos { x: 2, y: 2 }, &Pos { x: 3, y: 4 }), Pos { x: 2, y: 3 });
        assert_eq!(follow(&Pos { x: 2, y: 2 }, &Pos { x: 0, y: 1 }), Pos { x: 1, y: 2 });
        assert_eq!(follow(&Pos { x: 2, y: 2 }, &Pos { x: 1, y: 0 }), Pos { x: 2, y: 1 });
    }
}
