use std::collections::{HashSet, VecDeque};
use std::io;

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
struct Pos {
    row: usize,
    col: usize,
}

struct State {
    pos: Pos,
    dist: usize,
}

impl State {
    fn next_states(&self) -> impl Iterator<Item = State> {
        vec![
            State { pos: Pos { row: self.pos.row + 1, col: self.pos.col }, dist: self.dist + 1 },
            State { pos: Pos { row: self.pos.row, col: self.pos.col + 1 }, dist: self.dist + 1 },
            State {
                pos: Pos { row: self.pos.row.saturating_sub(1), col: self.pos.col },
                dist: self.dist + 1,
            },
            State {
                pos: Pos { row: self.pos.row, col: self.pos.col.saturating_sub(1) },
                dist: self.dist + 1,
            },
        ]
        .into_iter()
    }
}

fn main() {
    let mut start: Pos = Pos { row: 0, col: 0 };
    let mut end: Pos = Pos { row: 0, col: 0 };

    let heights: Vec<Vec<u8>> = io::stdin()
        .lines()
        .enumerate()
        .map(|(row, line)| {
            line.unwrap()
                .chars()
                .enumerate()
                .map(|(col, c)| match c {
                    'S' => {
                        start = Pos { row, col };
                        0
                    }
                    'E' => {
                        end = Pos { row, col };
                        25
                    }
                    _ => c as u8 - 'a' as u8,
                })
                .collect()
        })
        .collect();

    let rows = 0..heights.len();
    let cols = 0..heights[0].len();

    let mut step2result = None;

    let mut queue = VecDeque::from([State { pos: end, dist: 0 }]);
    let mut visited = HashSet::from([end]);

    while let Some(s) = queue.pop_front() {
        if heights[s.pos.row][s.pos.col] == 0 {
            step2result = step2result.or(Some(s.dist));
        }
        if s.pos == start {
            println!("Step 1: {}", s.dist);
            println!("Step 2: {}", step2result.unwrap());
            break;
        }

        s.next_states()
            .filter(|new_s| rows.contains(&new_s.pos.row) && cols.contains(&new_s.pos.col))
            .filter(|new_s| {
                heights[new_s.pos.row][new_s.pos.col]
                    >= heights[s.pos.row][s.pos.col].saturating_sub(1)
            })
            .filter(|new_s| visited.insert(new_s.pos))
            .for_each(|new_s| queue.push_back(new_s));
    }
}
