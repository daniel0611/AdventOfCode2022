use aoc_utils::PuzzleInput;
const DAY: u8 = 5;

struct Instruction {
    count: u8,
    source: usize,
    target: usize,
}

impl Instruction {
    fn new(s: &str) -> Self {
        // example: "move 2 from 2 to 1"
        let mut parts = s.split_whitespace();
        parts.next();
        let count = parts.next().unwrap().parse().unwrap();
        parts.next();
        let source: usize = parts.next().unwrap().parse().unwrap();
        parts.next();
        let target: usize = parts.next().unwrap().parse().unwrap();
        Self {
            count,
            source: source - 1, // convert to zero based indexing
            target: target - 1,
        }
    }
}

struct CrateState {
    crate_stacks: Vec<Vec<char>>,
    instructions: Vec<Instruction>,
}

impl CrateState {
    fn new(input: &PuzzleInput) -> Self {
        let parts = input.raw_input.split("\n\n").collect::<Vec<_>>();
        let crate_stacks = Self::parse_state(parts[0]);
        let instructions = parts[1].lines().map(Instruction::new).collect();
        Self {
            crate_stacks,
            instructions,
        }
    }

    fn parse_state(s: &str) -> Vec<Vec<char>> {
        // Example state:
        //     [D]
        // [N] [C]
        // [Z] [M] [P]
        //  1   2   3
        let count = s.lines().last().unwrap().split_whitespace().count();
        let mut stacks = vec![Vec::new(); count];
        for line in s.lines() {
            for (i, stack) in stacks.iter_mut().enumerate().take(count) {
                if let Some(c) = line.chars().nth(1 + 4 * i) {
                    if c != ' ' {
                        stack.insert(0, c);
                    }
                }
            }
        }

        stacks
    }

    fn execute_part1(&mut self) {
        for instruction in &self.instructions {
            for _ in 0..instruction.count {
                let c = self.crate_stacks[instruction.source].pop().unwrap();
                self.crate_stacks[instruction.target].push(c);
            }
        }
    }

    fn execute_part2(&mut self) {
        for instruction in &self.instructions {
            let mut crates = Vec::new();
            for _ in 0..instruction.count {
                crates.insert(0, self.crate_stacks[instruction.source].pop().unwrap());
            }
            for c in crates {
                self.crate_stacks[instruction.target].push(c);
            }
        }
    }

    fn get_top(&self) -> String {
        self.crate_stacks
            .iter()
            .map(|s| s.last().unwrap())
            .collect()
    }
}

fn main() {
    let input = PuzzleInput::get_input(DAY);
    println!("A: {}", solve_a(&input));
    println!("B: {}", solve_b(&input));
}

fn solve_a(input: &PuzzleInput) -> String {
    let mut state = CrateState::new(input);
    state.execute_part1();
    state.get_top()
}

fn solve_b(input: &PuzzleInput) -> String {
    let mut state = CrateState::new(input);
    state.execute_part2();
    state.get_top()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    #[test]
    fn test_no_panic() {
        let input = PuzzleInput::get_input(DAY);
        solve_a(&input);
        solve_b(&input);
    }

    #[test]
    fn test_solve_a() {
        assert_eq!(solve_a(&PuzzleInput::new(TEST_INPUT)), "CMZ");
    }

    #[test]
    fn test_solve_b() {
        assert_eq!(solve_b(&PuzzleInput::new(TEST_INPUT)), "MCD");
    }
}
