use std::io::stdin;

fn decode(s: &str) -> i64 {
    let mut result = 0;
    for c in s.chars() {
        result *= 5;
        result += match c {
            '=' => -2,
            '-' => -1,
            '0' => 0,
            '1' => 1,
            '2' => 2,
            _ => panic!("Invalid character {}", c),
        };
    }
    result
}

fn encode(mut i: i64) -> String {
    let mut result = Vec::new();
    while i > 0 {
        i += 2;
        result.push(match i % 5 {
            0 => '=',
            1 => '-',
            2 => '0',
            3 => '1',
            4 => '2',
            _ => panic!("Oops"),
        });
        i /= 5;
    }

    result.reverse();
    result.into_iter().collect()
}

fn main() {
    let mut sum = 0;
    for line in stdin().lines() {
        sum += decode(&line.unwrap());
    }
    println!("Step 1: {}", encode(sum));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode() {
        assert_eq!(decode("1"), 1);
        assert_eq!(decode("2"), 2);
        assert_eq!(decode("1="), 3);
        assert_eq!(decode("1-"), 4);
        assert_eq!(decode("10"), 5);
        assert_eq!(decode("11"), 6);
        assert_eq!(decode("12"), 7);
        assert_eq!(decode("2="), 8);
        assert_eq!(decode("2-"), 9);
        assert_eq!(decode("20"), 10);
        assert_eq!(decode("1=0"), 15);
        assert_eq!(decode("1-0"), 20);
        assert_eq!(decode("1=11-2"), 2022);
        assert_eq!(decode("1-0---0"), 12345);
        assert_eq!(decode("1121-1110-1=0"), 314159265);

        assert_eq!(decode("1=-0-2"), 1747);
        assert_eq!(decode("12111"), 906);
        assert_eq!(decode("2=0="), 198);
        assert_eq!(decode("21"), 11);
        assert_eq!(decode("2=01"), 201);
        assert_eq!(decode("111"), 31);
        assert_eq!(decode("20012"), 1257);
        assert_eq!(decode("112"), 32);
        assert_eq!(decode("1=-1="), 353);
        assert_eq!(decode("1-12"), 107);
        assert_eq!(decode("12"), 7);
        assert_eq!(decode("1="), 3);
        assert_eq!(decode("122"), 37);
    }

    #[test]
    fn test_encode() {
        assert_eq!(encode(1), "1");
        assert_eq!(encode(2), "2");
        assert_eq!(encode(3), "1=");
        assert_eq!(encode(4), "1-");
        assert_eq!(encode(5), "10");
        assert_eq!(encode(6), "11");
        assert_eq!(encode(7), "12");
        assert_eq!(encode(8), "2=");
        assert_eq!(encode(9), "2-");
        assert_eq!(encode(10), "20");
        assert_eq!(encode(15), "1=0");
        assert_eq!(encode(20), "1-0");
        assert_eq!(encode(2022), "1=11-2");
        assert_eq!(encode(12345), "1-0---0");
        assert_eq!(encode(314159265), "1121-1110-1=0");

        assert_eq!(encode(1747), "1=-0-2");
        assert_eq!(encode(906), "12111");
        assert_eq!(encode(198), "2=0=");
        assert_eq!(encode(11), "21");
        assert_eq!(encode(201), "2=01");
        assert_eq!(encode(31), "111");
        assert_eq!(encode(1257), "20012");
        assert_eq!(encode(32), "112");
        assert_eq!(encode(353), "1=-1=");
        assert_eq!(encode(107), "1-12");
        assert_eq!(encode(7), "12");
        assert_eq!(encode(3), "1=");
        assert_eq!(encode(37), "122");
    }
}
