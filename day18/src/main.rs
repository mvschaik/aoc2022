use std::collections::{HashSet, VecDeque};
use std::io::{stdin, BufRead};
use std::vec::IntoIter;

#[derive(Hash, Eq, PartialEq, Copy, Clone)]
struct Pos {
    x: i32,
    y: i32,
    z: i32,
}

impl Pos {
    fn sides(&self) -> IntoIter<Pos> {
        vec![
            Pos { x: self.x, y: self.y, z: self.z + 1 },
            Pos { x: self.x, y: self.y, z: self.z - 1 },
            Pos { x: self.x, y: self.y + 1, z: self.z },
            Pos { x: self.x, y: self.y - 1, z: self.z },
            Pos { x: self.x + 1, y: self.y, z: self.z },
            Pos { x: self.x - 1, y: self.y, z: self.z },
        ]
        .into_iter()
    }
}

fn main() {
    let mut lava = HashSet::new();
    let mut xmax = i32::MIN;
    let mut xmin = i32::MAX;
    let mut ymax = i32::MIN;
    let mut ymin = i32::MAX;
    let mut zmax = i32::MIN;
    let mut zmin = i32::MAX;
    for line in stdin().lock().lines() {
        match line.unwrap().split(",").collect::<Vec<_>>()[..] {
            [x, y, z] => {
                let p = Pos { x: x.parse().unwrap(), y: y.parse().unwrap(), z: z.parse().unwrap() };
                xmax = xmax.max(p.x);
                xmin = xmin.min(p.x);
                ymax = ymax.max(p.y);
                ymin = ymin.min(p.y);
                zmax = zmax.max(p.z);
                zmin = zmin.min(p.z);
                lava.insert(p);
            }
            _ => panic!("Malformed line"),
        }
    }

    let free_sides: usize =
        lava.iter().map(|p| p.sides().filter(|p| !lava.contains(p)).count()).sum();
    println!("Step 1: {free_sides}");

    let xrange = (xmin - 1)..(xmax + 2);
    let yrange = (ymin - 1)..(ymax + 2);
    let zrange = (zmin - 1)..(zmax + 2);

    let start = Pos { x: xmin, y: ymin, z: zmin };
    let mut todo = VecDeque::new();
    todo.push_back(start);
    let mut seen = HashSet::new();
    seen.insert(start);
    let mut sides = 0;
    while let Some(p) = todo.pop_back() {
        p.sides()
            .filter(|p| xrange.contains(&p.x) && yrange.contains(&p.y) && zrange.contains(&p.z))
            .for_each(|p| {
                if lava.contains(&p) {
                    sides += 1;
                } else {
                    if seen.insert(p) {
                        todo.push_back(p);
                    }
                }
            });
    }
    println!("Step 2: {sides}");
}
