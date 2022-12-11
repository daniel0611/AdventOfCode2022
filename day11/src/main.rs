use aoc_utils::PuzzleInput;
const DAY: u8 = 11;

fn main() {
    let input = PuzzleInput::get_input(DAY);
    println!("A: {}", solve_a(&input));
    println!("B: {}", solve_b(&input));
}

#[derive(Clone)]
struct Operation {
    is_multiply: bool,
    is_old_value_used: bool,
    value: Option<u64>,
}

impl Operation {
    fn new(s: &str) -> Self {
        let mut parts = s.split_whitespace();
        let _ = parts.next(); // Operation:
        let _ = parts.next(); // new
        let _ = parts.next(); // =
        let _ = parts.next(); // old

        let op = parts.next().unwrap(); // + or *
        let value_str = parts.next().unwrap(); // number or old

        Self {
            is_multiply: op == "*",
            is_old_value_used: value_str == "old",
            value: value_str.parse().ok(),
        }
    }

    fn apply(&self, old_value: u64) -> u64 {
        let other_operand = if self.is_old_value_used {
            old_value
        } else {
            self.value.unwrap()
        };

        if self.is_multiply {
            old_value * other_operand
        } else {
            old_value + other_operand
        }
    }
}

#[derive(Clone)]
struct Monkey {
    item_worry_levels: Vec<u64>,
    operation: Operation,
    test_divisor: u64,
    true_target_monkey: usize,
    false_target_monkey: usize,
    item_inspect_count: u64,
}

impl Monkey {
    fn new(lines: Vec<String>) -> Self {
        let starting_items = lines[1]
            .replace("Starting items: ", "")
            .trim()
            .split(", ")
            .map(|s| s.parse().unwrap())
            .collect();

        let operation = Operation::new(&lines[2]);
        let test_divisor = lines[3]
            .replace("Test: divisible by", "")
            .trim()
            .parse()
            .unwrap();

        let true_target_monkey = lines[4]
            .replace("If true: throw to monkey", "")
            .trim()
            .parse()
            .unwrap();
        let false_target_monkey = lines[5]
            .replace("If false: throw to monkey", "")
            .trim()
            .parse()
            .unwrap();

        Monkey {
            item_worry_levels: starting_items,
            operation,
            test_divisor,
            true_target_monkey,
            false_target_monkey,
            item_inspect_count: 0,
        }
    }

    fn step(&mut self, monkey_list: &mut [Monkey], divide_by_three: bool, common_divisor: u64) {
        while !self.item_worry_levels.is_empty() {
            self.item_inspect_count += 1;

            let worry_level = self.item_worry_levels.remove(0);
            let mut new_worry_level = self.operation.apply(worry_level);
            if divide_by_three {
                new_worry_level /= 3;
            }

            if new_worry_level % self.test_divisor == 0 {
                monkey_list[self.true_target_monkey]
                    .item_worry_levels
                    .push(new_worry_level % common_divisor);
            } else {
                monkey_list[self.false_target_monkey]
                    .item_worry_levels
                    .push(new_worry_level % common_divisor);
            }
        }
    }
}

fn simulate(input:&PuzzleInput, rounds: u64, part_a: bool) -> u64 {
    let mut monkeys: Vec<_> = input
        .raw_input
        .split("\n\n")
        .map(|lines| Monkey::new(lines.split('\n').map(|s| s.to_string()).collect()))
        .collect();

    let common_divisor = if !part_a {
        monkeys.iter().map(|m| m.test_divisor).product()
    } else {
        u64::MAX
    };

    for _ in 0..rounds {
        for i in 0..monkeys.len() {
            let mut monkey = monkeys[i].clone();
            monkey.step(&mut monkeys, part_a,  common_divisor);
            monkeys[i] = monkey;
        }
    }

    let mut scores = monkeys
        .iter()
        .map(|m| m.item_inspect_count)
        .collect::<Vec<_>>();
    scores.sort();
    scores.reverse();

    scores.iter().take(2).product()
}

fn solve_a(input: &PuzzleInput) -> u64 {
    simulate(input, 20, true)
}

fn solve_b(input: &PuzzleInput) -> u64 {
    simulate(input, 10_000, false)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";

    #[test]
    fn test_no_panic() {
        let input = PuzzleInput::get_input(DAY);
        solve_a(&input);
        solve_b(&input);
    }

    #[test]
    fn test_solve_a() {
        assert_eq!(solve_a(&PuzzleInput::new(TEST_INPUT)), 10605);
    }

    #[test]
    fn test_solve_b() {
        assert_eq!(solve_b(&PuzzleInput::new(TEST_INPUT)), 2713310158);
    }
}
