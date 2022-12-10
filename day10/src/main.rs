use aoc_utils::PuzzleInput;
const DAY: u8 = 10;

fn main() {
    let input = PuzzleInput::get_input(DAY);
    println!("A: {}", solve_a(&input));
    println!("B: \n{}", solve_b(&input));
}

#[derive(Copy, Clone)]
enum Instruction {
    Nop,
    AddX(i32),
}

impl Instruction {
    fn new(val: &str) -> Self {
        let mut parts = val.split_whitespace();
        let instruction = parts.next().unwrap();
        let value = parts.next().map(|p| p.parse::<i32>().unwrap());

        match instruction {
            "noop" => Instruction::Nop,
            "addx" => Instruction::AddX(value.unwrap()),
            _ => panic!("Unknown instruction: {}", instruction),
        }
    }
}

struct Cpu {
    x_register: i32,
    cycles: usize,
    instructions: Vec<Instruction>,
    current_instruction: Option<Instruction>,
}

impl Cpu {
    fn new(input: &PuzzleInput) -> Self {
        let instructions = input
            .lines()
            .iter()
            .map(|line| Instruction::new(line))
            .collect();

        Self {
            x_register: 1,
            cycles: 0,
            instructions,
            current_instruction: None,
        }
    }

    fn step(&mut self) {
        if let Some(instruction) = self.current_instruction {

            match instruction {
                Instruction::AddX(val) => {
                    self.x_register += val;
                },
                _ => panic!("Unknown instruction found"),
            }

            self.current_instruction = None;
        } else {
            let instruction = self.instructions.remove(0);

            match instruction {
                Instruction::Nop => {}
                Instruction::AddX(_) => {
                    self.current_instruction = Some(instruction);
                }
            }
        }

        self.cycles += 1;
    }

    fn is_done(&self) -> bool {
        self.instructions.is_empty() && self.current_instruction.is_none()
    }
}

fn solve_a(input: &PuzzleInput) -> usize {
    let mut cpu = Cpu::new(input);

    let mut signal_strength_sum = 0;

    while !cpu.is_done() {
        let previous_x = cpu.x_register;

        cpu.step();

        if cpu.cycles >= 20 && (cpu.cycles - 20) % 40 == 0 {
            signal_strength_sum += previous_x * cpu.cycles as i32;
        }
    }

    signal_strength_sum as usize
}

fn solve_b(input: &PuzzleInput) -> String {
    let mut cpu = Cpu::new(input);

    let mut pixels = [['.'; 40]; 6];

    while !cpu.is_done() {
        let sprite_x = cpu.x_register % 40;
        let crt_x = cpu.cycles % 40;
        let y = cpu.cycles / 40;

        if sprite_x >= 0 && (crt_x as i32 - sprite_x).abs() <= 1 {
            pixels[y as usize][crt_x] = '#';
        }

        cpu.step();

    }

    
    pixels
        .iter()
        .map(|row| row.iter().collect::<String>())
        .collect::<Vec<String>>()
        .join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";

    #[test]
    fn test_no_panic() {
        let input = PuzzleInput::get_input(DAY);
        solve_a(&input);
        solve_b(&input);
    }

    #[test]
    fn test_simple_program() {
        let input = PuzzleInput::new("noop
addx 3
addx -5");
        let mut cpu = Cpu::new(&input);

        cpu.step();
        assert_eq!(cpu.x_register, 1);
        assert_eq!(cpu.cycles, 1);

        cpu.step();
        assert_eq!(cpu.x_register, 1);
        assert_eq!(cpu.cycles, 2);

        cpu.step();
        assert_eq!(cpu.x_register, 4);
        assert_eq!(cpu.cycles, 3);

        cpu.step();
        assert_eq!(cpu.x_register, 4);
        assert_eq!(cpu.cycles, 4);

        cpu.step();
        assert_eq!(cpu.x_register, -1);
        assert_eq!(cpu.cycles, 5);

        assert!(cpu.is_done());
    }

    #[test]
    fn test_solve_a() {
        assert_eq!(solve_a(&PuzzleInput::new(TEST_INPUT)), 13140);
    }

    #[test]
    fn test_solve_b() {
        println!("{}", solve_b(&PuzzleInput::new(TEST_INPUT)));

        assert_eq!(solve_b(&PuzzleInput::new(TEST_INPUT)), "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....");
    }
}
