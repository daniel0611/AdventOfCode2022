use std::cmp::Ordering;

use aoc_utils::PuzzleInput;
const DAY: u8 = 13;

fn main() {
    let input = PuzzleInput::get_input(DAY);
    println!("A: {}", solve_a(&input));
    println!("B: {}", solve_b(&input));
}

#[derive(Clone, Debug)]
enum Value {
    Number(usize),
    List(Vec<Value>),
}

impl Value {
    fn parse(line: &str) -> Value {
        println!("{}", line);
        if line.starts_with('[') {
            let without_brackets = &line[1..line.len() - 1];
            let mut values = vec![];

            let mut depth = 0;
            let mut start = 0;
            for (i, c) in without_brackets.chars().enumerate() {
                match c {
                    '[' => depth += 1,
                    ']' => depth -= 1,
                    ',' if depth == 0 => {
                        values.push(Value::parse(&without_brackets[start..i]));
                        start = i + 1;
                    }
                    _ => {}
                }
            }

            if !without_brackets.is_empty() {
                values.push(Value::parse(&without_brackets[start..]));
            }

            Value::List(values)
        } else {
            Value::Number(line.parse().unwrap())
        }
    }

    fn format(&self) -> String {
        match self {
            Value::Number(n) => n.to_string(),
            Value::List(l) => {
                let mut s = String::new();
                s.push('[');
                for (i, v) in l.iter().enumerate() {
                    if i > 0 {
                        s.push(',');
                    }
                    s.push_str(&v.format());
                }
                s.push(']');
                s
            }
        }
    }
}

#[derive(PartialEq, Debug)]
enum Order {
    Wrong,
    Neutral,
    Right,
}

#[derive(Debug)]
struct Pair(Value, Value);

impl Pair {
    fn parse(lines: &str) -> Pair {
        let mut lines = lines.lines();
        let l = Value::parse(lines.next().unwrap());
        let r = Value::parse(lines.next().unwrap());
        Pair(l, r)
    }

    fn compare(&self) -> Order {
        match self {
            Pair(Value::Number(l), Value::Number(r)) => {
                println!("{} {}", l, r);
                match l.cmp(r) {
                    Ordering::Less => Order::Right,
                    Ordering::Equal => Order::Neutral,
                    Ordering::Greater => Order::Wrong,
                }
            }
            Pair(Value::List(l), Value::List(r)) => {
                let mut i = 0;

                while i < l.len() && i < r.len() {
                    let p = Pair(l[i].clone(), r[i].clone());
                    match p.compare() {
                        Order::Neutral => {}
                        o => return o,
                    }

                    i += 1;
                }

                if i == l.len() {
                    // Left run out of values
                    Order::Right
                } else {
                    Order::Wrong
                }
            }
            Pair(Value::Number(l), Value::List(r)) => {
                Pair(Value::List(vec![Value::Number(*l)]), Value::List(r.clone())).compare()
            }
            Pair(Value::List(l), Value::Number(r)) => {
                Pair(Value::List(l.clone()), Value::List(vec![Value::Number(*r)])).compare()
            }
        }
    }
}

fn solve_a(input: &PuzzleInput) -> usize {
    let pairs: Vec<_> = input.raw_input.split("\n\n").map(Pair::parse).collect();

    pairs
        .iter()
        .enumerate()
        .filter(|(_, p)| {
            println!("{:?}", p);
            println!("{}", p.0.format());
            println!("{}", p.1.format());
            println!("{:?}", p.compare());
            p.compare() == Order::Right
        })
        .map(|(i, _)| i + 1)
        .sum()
}

fn solve_b(input: &PuzzleInput) -> usize {
    let mut values: Vec<_> = input.lines()
    .filter(|l| !l.is_empty())
    .map(|l| Value::parse(&l)).collect();

    let divider_packets = vec!["[[2]]", "[[6]]"];
    for div in divider_packets.iter() {
        values.push(Value::parse(div));
    }

    loop {
        let mut sorted = true;

        for i in 0..values.len() - 1 {
            let p = Pair(values[i].clone(), values[i + 1].clone());

            if let Order::Wrong = p.compare() {
                sorted = false;
                // swap values
                values[i] = p.1;
                values[i + 1] = p.0;
            }
        }

        if sorted {
            break;
        }
    }

    values
        .iter()
        .enumerate()
        .filter(|(_, v)| divider_packets.contains(&&v.format()[..]))
        .map(|(i, _)| i + 1)
        .product()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";

    #[test]
    fn test_no_panic() {
        let input = PuzzleInput::get_input(DAY);
        solve_a(&input);
        solve_b(&input);
    }

    #[test]
    fn test_solve_a() {
        assert_eq!(solve_a(&PuzzleInput::new(TEST_INPUT)), 13);
    }

    #[test]
    fn test_solve_b() {
        assert_eq!(solve_b(&PuzzleInput::new(TEST_INPUT)), 140);
    }
}
