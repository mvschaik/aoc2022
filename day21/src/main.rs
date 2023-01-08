use regex::Regex;
use std::collections::HashMap;
use std::fmt::Debug;
use std::io::stdin;

type Monkeys = HashMap<String, Expr>;

#[derive(Debug)]
enum Expr {
    Add(String, String),
    Sub(String, String),
    Mul(String, String),
    Div(String, String),
    Eq(String, String),
    Val(i64),
    X,
}

use Expr::*;

impl Expr {
    fn eval(&self, monkeys: &Monkeys) -> i64 {
        match self {
            Add(a, b) => monkeys[a].eval(monkeys) + monkeys[b].eval(monkeys),
            Sub(a, b) => monkeys[a].eval(monkeys) - monkeys[b].eval(monkeys),
            Mul(a, b) => monkeys[a].eval(monkeys) * monkeys[b].eval(monkeys),
            Div(a, b) => monkeys[a].eval(monkeys) / monkeys[b].eval(monkeys),
            Val(n) => *n,
            X => panic!("And now what??"),
            Eq(_, _) => panic!("And now what??"),
        }
    }

    #[allow(dead_code)]
    fn print(&self, monkeys: &Monkeys) -> String {
        match self {
            Add(a, b) => format!("({} + {})", monkeys[a].print(monkeys), monkeys[b].print(monkeys)),
            Sub(a, b) => format!("({} - {})", monkeys[a].print(monkeys), monkeys[b].print(monkeys)),
            Mul(a, b) => format!("({} * {})", monkeys[a].print(monkeys), monkeys[b].print(monkeys)),
            Div(a, b) => format!("({} / {})", monkeys[a].print(monkeys), monkeys[b].print(monkeys)),
            Eq(a, b) => format!("{} = {}", monkeys[a].print(monkeys), monkeys[b].print(monkeys)),
            Val(x) => format!("{x}"),
            X => String::from("X"),
        }
    }
}

fn solve(expr: &str, monkeys: &Monkeys) -> i64 {
    let mut val = 0;
    let mut e = expr;

    loop {
        match &monkeys[e] {
            Eq(a, b) => {
                if let Val(x) = &monkeys[a] {
                    val = *x;
                    e = b;
                } else if let Val(x) = &monkeys[b] {
                    val = *x;
                    e = a;
                }
            }
            Add(a, b) => {
                if let Val(x) = &monkeys[a] {
                    val -= x;
                    e = b;
                } else if let Val(x) = &monkeys[b] {
                    val -= x;
                    e = a;
                }
            }
            Div(a, b) => {
                if let Val(x) = &monkeys[a] {
                    val = x / val;
                    e = b;
                } else if let Val(x) = &monkeys[b] {
                    val *= x;
                    e = a;
                }
            }
            Mul(a, b) => {
                if let Val(x) = &monkeys[a] {
                    val /= x;
                    e = b;
                } else if let Val(x) = &monkeys[b] {
                    val /= x;
                    e = a;
                }
            }
            Sub(a, b) => {
                if let Val(x) = &monkeys[a] {
                    val = x - val;
                    e = b;
                } else if let Val(x) = monkeys[b] {
                    val += x;
                    e = a;
                }
            }
            X => {
                return val;
            }
            Val(_) => panic!("Now what?"),
        }
    }
}

fn main() {
    let line_re =
        Regex::new(r"(?P<name>.{4}): ((?P<val>\d+)|(?P<a>.{4}) (?P<op>[-+*/]) (?P<b>.{4}))")
            .unwrap();

    let mut deps = HashMap::new();

    let mut monkeys = HashMap::new();
    for line in stdin().lines() {
        let line = line.unwrap();

        let cap = line_re.captures(&line).unwrap();
        let name = cap.name("name").unwrap().as_str();
        monkeys.insert(
            String::from(name),
            match cap.name("val") {
                Some(x) => Val(x.as_str().parse().unwrap()),
                None => {
                    let a = String::from(cap.name("a").unwrap().as_str());
                    let b = String::from(cap.name("b").unwrap().as_str());
                    deps.insert(a.clone(), String::from(name));
                    deps.insert(b.clone(), String::from(name));
                    match cap.name("op").unwrap().as_str() {
                        "+" => Add(a, b),
                        "-" => Sub(a, b),
                        "*" => Mul(a, b),
                        "/" => Div(a, b),
                        _ => panic!("Can't happen :-)"),
                    }
                }
            },
        );
    }
    println!("Part 1: {}", monkeys["root"].eval(&monkeys));

    if let Add(a, b) = &monkeys["root"] {
        monkeys.insert(String::from("root"), Eq(a.clone(), b.clone()));
        monkeys.insert(String::from("humn"), X);

        let mut to_resolve = "humn";
        while deps.contains_key(to_resolve) {
            let looking = to_resolve;
            to_resolve = &deps[to_resolve];
            match &monkeys[to_resolve] {
                Add(a, b) | Sub(a, b) | Mul(a, b) | Div(a, b) | Eq(a, b) if a == looking => {
                    monkeys.insert(b.clone(), Val(monkeys[&*b].eval(&monkeys)));
                }
                Add(a, b) | Sub(a, b) | Mul(a, b) | Div(a, b) | Eq(a, b) if b == looking => {
                    monkeys.insert(a.clone(), Val(monkeys[&*a].eval(&monkeys)));
                }
                Val(_) => (),
                X => (),
                x @ _ => panic!("??? {:?}", x),
            }
        }
        println!("Part 2: {}", solve("root", &monkeys));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_eq() {
        let mut monkeys: Monkeys = HashMap::new();
        // 2 + x = 5
        monkeys.insert(String::from("root"), Eq(String::from("a"), String::from("b")));
        monkeys.insert(String::from("humn"), X);

        monkeys.insert(String::from("a"), Add(String::from("c"), String::from("humn")));
        monkeys.insert(String::from("b"), Val(5));
        monkeys.insert(String::from("c"), Val(2));

        assert_eq!(solve("root", &monkeys), 3);

        // 5 = 2 + x
        monkeys.insert(String::from("root"), Eq(String::from("b"), String::from("a")));

        assert_eq!(solve("root", &monkeys), 3);
    }

    #[test]
    fn test_solve_add() {
        let mut monkeys: Monkeys = HashMap::new();
        // 2 + x = 5
        monkeys.insert(String::from("root"), Eq(String::from("a"), String::from("b")));
        monkeys.insert(String::from("humn"), X);

        monkeys.insert(String::from("a"), Add(String::from("c"), String::from("humn")));
        monkeys.insert(String::from("b"), Val(5));
        monkeys.insert(String::from("c"), Val(2));

        assert_eq!(solve("root", &monkeys), 3);

        // x + 2 = 5
        monkeys.insert(String::from("a"), Add(String::from("humn"), String::from("c")));

        assert_eq!(solve("root", &monkeys), 3);
    }

    #[test]
    fn test_solve_sub() {
        let mut monkeys: Monkeys = HashMap::new();
        // 2 - x = 5
        monkeys.insert(String::from("root"), Eq(String::from("a"), String::from("b")));
        monkeys.insert(String::from("humn"), X);

        monkeys.insert(String::from("a"), Sub(String::from("c"), String::from("humn")));
        monkeys.insert(String::from("b"), Val(5));
        monkeys.insert(String::from("c"), Val(2));

        assert_eq!(solve("root", &monkeys), -3);

        // x - 2 = 5
        monkeys.insert(String::from("a"), Sub(String::from("humn"), String::from("c")));

        assert_eq!(solve("root", &monkeys), 7);
    }

    #[test]
    fn test_solve_mul() {
        let mut monkeys: Monkeys = HashMap::new();
        // 2 * x = 6
        monkeys.insert(String::from("root"), Eq(String::from("a"), String::from("b")));
        monkeys.insert(String::from("humn"), X);

        monkeys.insert(String::from("a"), Mul(String::from("c"), String::from("humn")));
        monkeys.insert(String::from("b"), Val(6));
        monkeys.insert(String::from("c"), Val(2));

        assert_eq!(solve("root", &monkeys), 3);

        // x * 2 = 6
        monkeys.insert(String::from("a"), Mul(String::from("humn"), String::from("c")));

        assert_eq!(solve("root", &monkeys), 3);
    }

    #[test]
    fn test_solve_div() {
        let mut monkeys: Monkeys = HashMap::new();
        // 6 / x = 2
        monkeys.insert(String::from("root"), Eq(String::from("a"), String::from("b")));
        monkeys.insert(String::from("humn"), X);

        monkeys.insert(String::from("a"), Div(String::from("c"), String::from("humn")));
        monkeys.insert(String::from("b"), Val(2));
        monkeys.insert(String::from("c"), Val(6));

        assert_eq!(solve("root", &monkeys), 3);

        // x / 6 = 2
        monkeys.insert(String::from("a"), Div(String::from("humn"), String::from("c")));

        assert_eq!(solve("root", &monkeys), 12);
    }
}
