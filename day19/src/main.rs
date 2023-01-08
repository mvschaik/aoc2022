use regex::Regex;
use std::collections::VecDeque;
use std::io::stdin;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct Resources {
    ore: i32,
    clay: i32,
    obs: i32,
    geode: i32,

    ore_robots: i32,
    clay_robots: i32,
    obs_robots: i32,
    geode_robots: i32,

    time: i32,
}

impl Resources {
    fn new(time: i32) -> Resources {
        Resources {
            ore: 0,
            clay: 0,
            obs: 0,
            geode: 0,

            ore_robots: 1,
            clay_robots: 0,
            obs_robots: 0,
            geode_robots: 0,

            time,
        }
    }
}

#[derive(Debug)]
struct Costs {
    ore_cost_ore: i32,
    clay_cost_ore: i32,
    obs_cost_ore: i32,
    obs_cost_clay: i32,
    geode_cost_ore: i32,
    geode_cost_obs: i32,
}

fn div_ceil(a: i32, b: i32) -> i32 { (a + b - 1) / b }

fn run(time: i32, costs: &Costs) -> i32 {
    let mut max_possible_geodes = Vec::new();
    for i in 0..(time + 1) {
        max_possible_geodes.push(i + max_possible_geodes.last().unwrap_or(&0));
    }

    let mut max_score = 0;
    let mut todo = VecDeque::new();
    todo.push_back(Resources::new(time));
    while let Some(r) = todo.pop_back() {
        if r.time < 0 {
            continue;
        }
        if r.geode + r.geode_robots * r.time + max_possible_geodes[r.time as usize] < max_score {
            continue;
        }

        // Buy ore bot
        let wait_time = div_ceil(costs.ore_cost_ore - r.ore, r.ore_robots).max(0) + 1;
        if wait_time < r.time {
            todo.push_back(Resources {
                ore: r.ore + r.ore_robots * wait_time - costs.ore_cost_ore,
                clay: r.clay + r.clay_robots * wait_time,
                obs: r.obs + r.obs_robots * wait_time,
                geode: r.geode + r.geode_robots * wait_time,

                ore_robots: r.ore_robots + 1,
                time: r.time - wait_time,
                ..r
            });
        }

        // Buy clay bot
        let wait_time = div_ceil(costs.clay_cost_ore - r.ore, r.ore_robots).max(0) + 1;
        if wait_time < r.time {
            todo.push_back(Resources {
                ore: r.ore + r.ore_robots * wait_time - costs.clay_cost_ore,
                clay: r.clay + r.clay_robots * wait_time,
                obs: r.obs + r.obs_robots * wait_time,
                geode: r.geode + r.geode_robots * wait_time,

                clay_robots: r.clay_robots + 1,
                time: r.time - wait_time,
                ..r
            });
        }

        // Buy obs bot
        if r.clay_robots > 0 {
            let wait_time = div_ceil(costs.obs_cost_ore - r.ore, r.ore_robots)
                .max(div_ceil(costs.obs_cost_clay - r.clay, r.clay_robots))
                .max(0)
                + 1;
            if wait_time < r.time {
                todo.push_back(Resources {
                    ore: r.ore + r.ore_robots * wait_time - costs.obs_cost_ore,
                    clay: r.clay + r.clay_robots * wait_time - costs.obs_cost_clay,
                    obs: r.obs + r.obs_robots * wait_time,
                    geode: r.geode + r.geode_robots * wait_time,

                    obs_robots: r.obs_robots + 1,
                    time: r.time - wait_time,
                    ..r
                });
            }
        }

        // Buy geode bot
        if r.obs_robots > 0 {
            let wait_time = div_ceil(costs.geode_cost_ore - r.ore, r.ore_robots)
                .max(div_ceil(costs.geode_cost_obs - r.obs, r.obs_robots))
                .max(0)
                + 1;
            if wait_time < r.time {
                todo.push_back(Resources {
                    ore: r.ore + r.ore_robots * wait_time - costs.geode_cost_ore,
                    clay: r.clay + r.clay_robots * wait_time,
                    obs: r.obs + r.obs_robots * wait_time - costs.geode_cost_obs,
                    geode: r.geode + r.geode_robots * wait_time,

                    geode_robots: r.geode_robots + 1,
                    time: r.time - wait_time,
                    ..r
                });
            }
        }

        if r.geode_robots > 0 {
            max_score = max_score.max(r.geode + r.geode_robots * r.time);
        }
    }
    max_score
}

fn main() {
    let blueprint_re = Regex::new(r"(?x)
        Blueprint\s(?P<id>\d+):\s
             Each\sore\srobot\scosts\s(?P<ore_cost>\d+)\sore.\s
             Each\sclay\srobot\scosts\s(?P<clay_cost>\d+)\sore.\s
             Each\sobsidian\srobot\scosts\s(?P<obs_cost_ore>\d+)\sore\sand\s(?P<obs_cost_clay>\d+)\sclay.\s
             Each\sgeode\srobot\scosts\s(?P<geode_cost_ore>\d+)\sore\sand\s(?P<geode_cost_obs>\d+)\sobsidian.")
        .unwrap();

    let mut blueprints = Vec::new();

    for line in stdin().lines() {
        let line = line.unwrap();
        let cap = blueprint_re.captures(&line).unwrap();
        blueprints.push(Costs {
            ore_cost_ore: cap.name("ore_cost").unwrap().as_str().parse().unwrap(),
            clay_cost_ore: cap.name("clay_cost").unwrap().as_str().parse().unwrap(),
            obs_cost_ore: cap.name("obs_cost_ore").unwrap().as_str().parse().unwrap(),
            obs_cost_clay: cap.name("obs_cost_clay").unwrap().as_str().parse().unwrap(),
            geode_cost_ore: cap.name("geode_cost_ore").unwrap().as_str().parse().unwrap(),
            geode_cost_obs: cap.name("geode_cost_obs").unwrap().as_str().parse().unwrap(),
        });
    }

    let mut total_q = 0;
    for (i, costs) in blueprints.iter().enumerate() {
        let blueprint = i as i32 + 1;
        let max_score = run(24, costs);
        println!("Blueprint {}: {} geodes", blueprint, max_score);
        total_q += blueprint * max_score;
    }
    println!("Part 1: {}", total_q);

    let mut result = 1;
    for costs in blueprints.iter().take(3) {
        let score = run(32, costs);
        println!("{} geodes", score);
        result *= score;
    }
    println!("Part 2: {}", result);
}
