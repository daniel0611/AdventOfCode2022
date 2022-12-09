use std::collections::HashSet;

use aoc_utils::PuzzleInput;
const DAY: u8 = 9;

fn main() {
    let input = PuzzleInput::get_input(DAY);
    println!("A: {}", solve_a(&input));
    println!("B: {}", solve_b(&input));
}

#[derive(Copy, Clone)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
}

impl From<char> for Direction {
    fn from(c: char) -> Self {
        match c {
            'U' => Direction::Up,
            'D' => Direction::Down,
            'R' => Direction::Right,
            'L' => Direction::Left,
            _ => panic!("Invalid direction: {}", c),
        }
    }
}

#[derive(Copy, Clone)]
struct Command {
    direction: Direction,
    distance: usize,
}

impl From<&String> for Command {
    fn from(s: &String) -> Self {
        let direction = Direction::from(s.chars().next().unwrap());
        let distance = s[1..].trim().parse().unwrap();
        Command {
            direction,
            distance,
        }
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
struct Position(isize, isize);

struct GameState {
    head_position: Position,
    knot_positions: [Position; 9],
    commands: Vec<Command>,
    visited_knot_positions: [HashSet<Position>; 9],
}

impl GameState {
    fn parse(input: &PuzzleInput) -> Self {
        let commands = input
            .lines()
            .iter()
            .map(Command::from)
            .collect();

        let mut visited_knot_positions = vec![];
        for _ in 0..9 {
            visited_knot_positions.push(HashSet::new());
            visited_knot_positions.last_mut().unwrap().insert(Position(0, 0));
        }

        GameState {
            head_position: Position(0, 0),
            knot_positions: [Position(0, 0); 9],
            commands,
            visited_knot_positions: visited_knot_positions.try_into().unwrap(),
        }
    }

    fn update_head(&mut self, direction: &Direction) {
        let (dx, dy) = match direction {
            Direction::Up => (0, 1),
            Direction::Down => (0, -1),
            Direction::Right => (1, 0),
            Direction::Left => (-1, 0),
        };

        self.head_position.0 += dx;
        self.head_position.1 += dy;
    }

    fn update_knot(&mut self, knot_index: usize) {
        let head_knot = if knot_index == 0 {self.head_position} else {self.knot_positions[knot_index - 1]};
        let tail_knot = self.knot_positions[knot_index];

        let (tx, ty) = (tail_knot.0, tail_knot.1);
        let (hx, hy) = (head_knot.0, head_knot.1);

        // if tail is adjacent (even diagonally) to head or on head, don't move
        if (tx - hx).abs() <= 1 && (ty - hy).abs() <= 1 {
            return;
        }

        let (dx, dy) = if tx == hx || ty == hy {
            // straight line
            if tx == hx {
                // vertical line
                if ty < hy {
                    // tail is above head
                    (0, 1)
                } else {
                    // tail is below head
                    (0, -1)
                }
            } else {
                // horizontal line
                if tx < hx {
                    // tail is left of head
                    (1, 0)
                } else {
                    // tail is right of head
                    (-1, 0)
                }
            }
        } else {
            // Diagonal
            if tx < hx {
                // tail is left of head
                if ty < hy {
                    // tail is above head
                    (1, 1)
                } else {
                    // tail is below head
                    (1, -1)
                }
            } else {
                // tail is right of head
                if ty < hy {
                    // tail is above head
                    (-1, 1)
                } else {
                    // tail is below head
                    (-1, -1)
                }
            }
        };

        self.knot_positions[knot_index].0 += dx;
        self.knot_positions[knot_index].1 += dy;
    }

    fn execute_step(&mut self, direction: &Direction) {
        self.update_head(direction);
        for knot_index in 0..9 {
            self.update_knot(knot_index);
            self.visited_knot_positions[knot_index].insert(self.knot_positions[knot_index].clone());
        }
    }

    fn execute_command(&mut self, command: &Command) {
        for _ in 0..command.distance {
            self.execute_step(&command.direction);
        }
    }

    fn simulate(&mut self) {
        for command in &self.commands.to_vec() {
            self.execute_command(command);
        }
    }
}

fn solve_a(input: &PuzzleInput) -> usize {
    let mut state = GameState::parse(input);
    
    state.simulate();
    state.visited_knot_positions[0].len()
}

fn solve_b(input: &PuzzleInput) -> usize {
    let mut state = GameState::parse(input);
    
    state.simulate();
    state.visited_knot_positions[8].len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_panic() {
        let input = PuzzleInput::get_input(DAY);
        solve_a(&input);
        solve_b(&input);
    }

    #[test]
    fn test_solve_a() {
        assert_eq!(solve_a(&PuzzleInput::new("R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2")), 13);
    }

    #[test]
    fn test_solve_b() {
        assert_eq!(solve_b(&PuzzleInput::new("R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20")), 36);
    }
}
