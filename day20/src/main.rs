use std::io::stdin;

type Value = i64;
type Msg = Vec<(usize, Value)>;

fn mix(msg: &mut Msg) {
    for i in 0..msg.len() {
        let pos_before = msg.iter().position(|(x, _)| i == *x).unwrap();
        mv(msg, pos_before, msg[pos_before].1);
    }
}


fn score(msg: &Msg) -> Value {
    let zero = msg.iter().position(|(_, x)| *x == 0).unwrap();
    let n1k = msg[(zero + 1000) % msg.len()].1;
    let n2k = msg[(zero + 2000) % msg.len()].1;
    let n3k = msg[(zero + 3000) % msg.len()].1;
    return n1k + n2k + n3k;
}

fn main() {
    let mut msg: Msg = Vec::new();
    for (i, line) in stdin().lines().enumerate() {
        msg.push((i, line.unwrap().parse().unwrap()));
    }

    let orig_msg = msg.clone();

    mix(&mut msg);

    println!("Step 1: {}", score(&msg));

    msg = orig_msg.into_iter().map(|(i, n)| (i, n * 811589153)).collect();
    for _ in 0..10 {
        mix(&mut msg);
    }
    println!("Step 2: {}", score(&msg));
}

fn mv<T>(msg: &mut Vec<T>, from_index: usize, offset: Value) {
    let x = msg.remove(from_index);
    let to_index = (from_index as Value + offset).rem_euclid(msg.len() as Value);
    msg.insert(to_index as usize, x);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mv_plus1() {
        let mut msg = vec![0, 1, 2, 3, 4];
        mv(&mut msg, 2, 1);
        assert_eq!(msg, vec![0, 1, 3, 2, 4]);
    }

    #[test]
    fn mv_min1() {
        let mut msg = vec![0, 1, 2, 3, 4];
        mv(&mut msg, 2, -1);
        assert_eq!(msg, vec![0, 2, 1, 3, 4]);
    }

    #[test]
    fn mv_plus2() {
        let mut msg = vec![0, 1, 2, 3, 4];
        mv(&mut msg, 2, 2);
        assert_eq!(msg, vec![2, 0, 1, 3, 4]);
    }

    #[test]
    fn mv_min2() {
        let mut msg = vec![0, 1, 2, 3, 4];
        mv(&mut msg, 2, -2);
        assert_eq!(msg, vec![2, 0, 1, 3, 4]);
    }

    #[test]
    fn mv_plus3() {
        let mut msg = vec![0, 1, 2, 3, 4];
        mv(&mut msg, 2, 3);
        assert_eq!(msg, vec![0, 2, 1, 3, 4]);
    }

    #[test]
    fn mv_cycle() {
        let mut msg = vec![0, 1, 2, 3, 4];
        mv(&mut msg, 2, 4);
        assert_eq!(msg, vec![0, 1, 2, 3, 4]);
        mv(&mut msg, 2, 5);
        assert_eq!(msg, vec![0, 1, 3, 2, 4]);
        mv(&mut msg, 3, 6);
        assert_eq!(msg, vec![0, 2, 1, 3, 4]);
        mv(&mut msg, 1, 7);
        assert_eq!(msg, vec![2, 0, 1, 3, 4]);
        mv(&mut msg, 4, 8);
        assert_eq!(msg, vec![4, 2, 0, 1, 3]);
    }
}

