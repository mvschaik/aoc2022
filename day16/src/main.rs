use itertools::Itertools;
use regex::Regex;
use std::collections::{HashMap, HashSet, VecDeque};
use std::io::{stdin, BufRead};

#[derive(Debug)]
struct Valve {
    rate: i32,
    tunnels: Vec<String>,
}

#[derive(Clone)]
struct State {
    position: String,
    visited: HashSet<String>,
    time_left: i32,
    pressure: i32,
}

impl State {
    fn new(time: i32, pos: String) -> State {
        State {
            position: pos.clone(),
            visited: HashSet::from_iter(vec![pos]),
            time_left: time,
            pressure: 0,
        }
    }

    fn next(&self, new_pos: &str, dist: i32, rate: i32) -> Option<State> {
        if self.visited.contains(new_pos) {
            return None;
        }
        let time_left = &self.time_left - dist - 1;
        if time_left < 0 {
            return None;
        }

        let new_pos = new_pos.to_string();

        let mut new_state = (*self).clone();
        new_state.position = new_pos.clone();
        new_state.visited.insert(new_pos);
        new_state.time_left = time_left;
        new_state.pressure += time_left * rate;
        Some(new_state)
    }
}

fn distance(system: &HashMap<String, Valve>, from: &str, to: &str) -> i32 {
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    queue.push_back((from, 0));
    while let Some((pos, dist)) = queue.pop_front() {
        visited.insert(pos.to_string());
        for next in &system[pos].tunnels {
            if next == to {
                return dist + 1;
            }
            if !visited.contains(next) {
                queue.push_back((&next, dist + 1));
            }
        }
    }
    i32::MAX
}

fn most_pressure(
    time: i32,
    dist: &HashMap<&str, Vec<(&str, i32)>>,
    valves: &HashMap<&str, &Valve>,
    allowed: &HashSet<&str>,
) -> i32 {
    let mut result = 0;
    let mut todo = VecDeque::new();
    todo.push_back(State::new(time, String::from("AA")));
    while let Some(state) = todo.pop_front() {
        result = result.max(state.pressure);
        for (next, dist) in &dist[&*state.position] {
            if !allowed.contains(next) {
                continue;
            }
            if let Some(new_state) = state.next(next, *dist, valves[next].rate) {
                todo.push_back(new_state);
            }
        }
    }
    result
}

fn fprint(valves: &HashSet<&str>) -> String { valves.into_iter().sorted().join("") }

fn main() {
    let valve_re = Regex::new(r"Valve (?P<valve>\w\w) has flow rate=(?P<rate>\d+); tunnels? leads? to valves? (?P<tunnels>.*)").unwrap();
    let mut world = HashMap::new();

    for line in stdin().lock().lines() {
        let line = line.unwrap();
        let cap = valve_re.captures(&line).unwrap();
        world.insert(
            cap.name("valve").unwrap().as_str().to_string(),
            Valve {
                rate: cap.name("rate").unwrap().as_str().parse().unwrap(),
                tunnels: cap
                    .name("tunnels")
                    .unwrap()
                    .as_str()
                    .split(", ")
                    .map(|s| s.to_string())
                    .collect(),
            },
        );
    }

    let valves: HashMap<&str, &Valve> = world
        .iter()
        .filter(|(name, valve)| *name == "AA" || valve.rate > 0)
        .map(|(name, value)| (&name[..], value))
        .collect();
    let mut dist = HashMap::new();
    for from in valves.keys() {
        for to in valves.keys() {
            if from == to {
                continue;
            }
            let d = distance(&world, from, to);
            dist.entry(*from).or_insert(Vec::new()).push((*to, d));
        }
    }

    let all_valves = valves.keys().copied().collect();
    println!("Step 1: {}", most_pressure(30, &dist, &valves, &all_valves));

    let mut results = HashMap::new();
    let mut complements = Vec::new();
    for s in valves.keys().powerset() {
        let s: HashSet<&str> = s.into_iter().copied().collect();
        let fp = fprint(&s);
        results.insert(fp.clone(), most_pressure(26, &dist, &valves, &s));

        let compl = all_valves.difference(&s.into()).copied().collect();
        complements.push((fp, fprint(&compl)));
    }

    let mut best = 0;
    for (human, elephant) in complements {
        best = best.max(results[&human] + results[&elephant]);
    }
    println!("Step 2: {}", best);
}
