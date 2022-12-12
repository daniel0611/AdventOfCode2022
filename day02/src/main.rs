use aoc_utils::PuzzleInput;
const DAY: u8 = 2;

fn main() {
    let input = PuzzleInput::get_input(DAY);
    println!("A: {}", solve_a(&input));
    println!("B: {}", solve_b(&input));
}

#[derive(Copy, Clone)]
enum HandShape {
    Rock,
    Paper,
    Scissors,
}

impl HandShape {
    fn from_char(c: char) -> HandShape {
        match c {
            'A' | 'X' => HandShape::Rock,
            'B' | 'Y' => HandShape::Paper,
            'C' | 'Z' => HandShape::Scissors,
            _ => panic!("Invalid hand shape: {}", c),
        }
    }

    fn get_value(&self) -> usize {
        match self {
            HandShape::Rock => 1,
            HandShape::Paper => 2,
            HandShape::Scissors => 3,
        }
    }
}

enum DesiredWinState {
    Win,
    Draw,
    Lose,
}

impl DesiredWinState {
    fn from_char(c: char) -> DesiredWinState {
        match c {
            'Z' => DesiredWinState::Win,
            'Y' => DesiredWinState::Draw,
            'X' => DesiredWinState::Lose,
            _ => panic!("Invalid desired win state"),
        }
    }

    fn get_needed_shape(&self, other: HandShape) -> HandShape {
        match self {
            DesiredWinState::Win => match other {
                HandShape::Rock => HandShape::Paper,
                HandShape::Paper => HandShape::Scissors,
                HandShape::Scissors => HandShape::Rock,
            },
            DesiredWinState::Draw => other,
            DesiredWinState::Lose => match other {
                HandShape::Rock => HandShape::Scissors,
                HandShape::Paper => HandShape::Rock,
                HandShape::Scissors => HandShape::Paper,
            },
        }
    }
}

fn calculate_score(strategy: (HandShape, HandShape)) -> usize {
    match strategy {
        (other, HandShape::Rock) => match other {
            HandShape::Rock => 3,
            HandShape::Paper => 0,
            HandShape::Scissors => 6,
        },
        (other, HandShape::Paper) => match other {
            HandShape::Rock => 6,
            HandShape::Paper => 3,
            HandShape::Scissors => 0,
        },
        (other, HandShape::Scissors) => match other {
            HandShape::Rock => 0,
            HandShape::Paper => 6,
            HandShape::Scissors => 3,
        },
    }
}

fn solve_a(input: &PuzzleInput) -> usize {
    let strategy: Vec<_> = input
        .lines()
        .map(|line| {
            let shapes: Vec<_> = line
                .split(' ')
                .map(|s| HandShape::from_char(s.chars().next().unwrap()))
                .collect();
            (shapes[0], shapes[1])
        })
        .collect();

    strategy
        .iter()
        .map(|s| calculate_score(*s) + s.1.get_value())
        .sum()
}

fn solve_b(input: &PuzzleInput) -> usize {
    let strategy: Vec<_> = input
        .lines()
        .map(|line| {
            let shapes: Vec<_> = line.split(' ').collect();
            (
                HandShape::from_char(shapes[0].chars().next().unwrap()),
                DesiredWinState::from_char(shapes[1].chars().next().unwrap()),
            )
        })
        .collect();

    strategy
        .iter()
        .map(|s| {
            let needed_shape = s.1.get_needed_shape(s.0);
            calculate_score((s.0, needed_shape)) + needed_shape.get_value()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "A Y
B X
C Z";

    #[test]
    fn test_no_panic() {
        let input = PuzzleInput::get_input(DAY);
        solve_a(&input);
        solve_b(&input);
    }

    #[test]
    fn test_solve_a() {
        assert_eq!(solve_a(&PuzzleInput::new(TEST_INPUT)), 15);
    }

    #[test]
    fn test_solve_b() {
        assert_eq!(solve_b(&PuzzleInput::new(TEST_INPUT)), 12);
    }
}
