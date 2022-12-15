use lazy_static::lazy_static;
use regex::Regex;
use std::collections::{BTreeMap, HashSet};
use std::env::args;
use std::io::{self, BufRead};
use std::ops::Range;

#[derive(Hash, Eq, PartialEq, Copy, Clone)]
struct Pos {
    x: i32,
    y: i32,
}

impl Pos {
    fn dist(&self, other: &Pos) -> i32 { (self.x - other.x).abs() + (self.y - other.y).abs() }
}

struct RangeSet<T> {
    ranges: BTreeMap<T, Range<T>>,
}

impl<T: Ord + Copy> RangeSet<T> {
    fn new() -> RangeSet<T> { RangeSet { ranges: BTreeMap::new() } }

    fn add(&mut self, mut new_range: Range<T>) {
        let mut to_replace = Vec::new();
        for (s, r) in self.ranges.iter() {
            if to_replace.is_empty() && new_range.start < *s && new_range.end < *s {
                self.ranges.insert(new_range.start, new_range);
                return;
            } else if overlaps(&new_range, r) {
                new_range = (new_range.start.min(r.start))..(new_range.end.max(r.end));
                to_replace.push(*s);
            }
        }
        for s in &to_replace {
            self.ranges.remove(s);
        }
        self.ranges.insert(new_range.start, new_range);
    }

    #[allow(dead_code)]
    fn contains(&self, x: T) -> bool {
        for (start, r) in &self.ranges {
            if x < *start {
                continue;
            }
            if r.contains(&x) {
                return true;
            }
        }
        false
    }

    fn clamp(&self, range: Range<T>) -> RangeSet<T> {
        let mut result = RangeSet::new();
        for (_, r) in &self.ranges {
            result.add(r.start.clamp(range.start, range.end)..r.end.clamp(range.start, range.end));
        }
        result
    }
}

fn overlaps<T: PartialOrd>(a: &Range<T>, b: &Range<T>) -> bool {
    a.start <= b.end && a.end >= b.start
}

fn parse_line(line: &str) -> (Pos, Pos) {
    lazy_static! {
        static ref LINE_RE: Regex = Regex::new(r"Sensor at x=(?P<sx>-?\d+), y=(?P<sy>-?\d+): closest beacon is at x=(?P<bx>-?\d+), y=(?P<by>-?\d+)").unwrap();
    }
    let captures = LINE_RE.captures(&line).unwrap();
    let sensor = Pos {
        x: captures.name("sx").unwrap().as_str().parse().unwrap(),
        y: captures.name("sy").unwrap().as_str().parse().unwrap(),
    };
    let beacon = Pos {
        x: captures.name("bx").unwrap().as_str().parse().unwrap(),
        y: captures.name("by").unwrap().as_str().parse().unwrap(),
    };
    (sensor, beacon)
}

fn main() {
    let size = if args().any(|s| s.contains("debug")) { 20 } else { 4000000 };
    let step1_y = size / 2;
    let mut beacons_at_step1_y = HashSet::new();
    let mut occupations = BTreeMap::new();
    for line in io::stdin().lock().lines() {
        let (sensor, beacon) = parse_line(&line.unwrap());
        if beacon.y == step1_y {
            beacons_at_step1_y.insert(beacon);
        }

        let range = sensor.dist(&beacon);
        for y in 0..size {
            let distance_to_row = (y - sensor.y).abs();
            let half_width = (range - distance_to_row).abs();
            if distance_to_row > range {
                continue;
            }
            occupations
                .entry(y)
                .or_insert(RangeSet::new())
                .add((sensor.x - half_width)..(1 + (sensor.x + half_width)));
        }
    }
    println!(
        "Step 1: {}",
        occupations[&step1_y].ranges.values().map(|r| r.len()).sum::<usize>()
            - beacons_at_step1_y.len()
    );
    for (y, occ) in occupations {
        let clamped = occ.clamp(0..size);
        if clamped.ranges.len() != 1 {
            println!("Step 2: {}", clamped.ranges[&0].end as i64 * 4000000 + y as i64);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_overlap() {
        assert!(overlaps(&(5..10), &(3..15)));
        assert!(overlaps(&(5..10), &(10..15)));
        assert!(overlaps(&(5..10), &(5..10)));
        assert!(overlaps(&(10..15), &(5..10)));
        assert!(overlaps(&(10..15), &(5..10)));
        assert!(overlaps(&(5..20), &(10..15)));

        assert!(!overlaps(&(5..10), &(11..15)));
        assert!(!overlaps(&(10..20), &(3..9)));
    }

    #[test]
    fn add_range_before() {
        let mut r = RangeSet::new();
        r.add(10..20);
        r.add(30..40);

        r.add(1..5);

        assert_eq!(3, r.ranges.len());
    }

    #[test]
    fn add_range_touching() {
        let mut r = RangeSet::new();
        r.add(10..20);
        r.add(30..40);

        r.add(5..10);

        assert_eq!(2, r.ranges.len());
        assert_eq!(*r.ranges.iter().next().unwrap().1, 5..20);

        r.add(20..25);
        assert_eq!(*r.ranges.iter().next().unwrap().1, 5..25);
    }

    #[test]
    fn add_range_overlap() {
        let mut r = RangeSet::new();
        r.add(10..20);
        r.add(30..40);

        r.add(5..15);

        assert_eq!(2, r.ranges.len());
        assert_eq!(*r.ranges.iter().next().unwrap().1, 5..20);

        r.add(10..15);

        assert_eq!(2, r.ranges.len());
        assert_eq!(*r.ranges.iter().next().unwrap().1, 5..20);

        r.add(15..25);

        assert_eq!(2, r.ranges.len());
        assert_eq!(*r.ranges.iter().next().unwrap().1, 5..25);
    }

    #[test]
    fn add_range_joining() {
        let mut r = RangeSet::new();
        r.add(10..20);
        r.add(30..40);

        r.add(20..30);

        assert_eq!(1, r.ranges.len());
        assert_eq!(*r.ranges.iter().next().unwrap().1, 10..40);
    }
}
