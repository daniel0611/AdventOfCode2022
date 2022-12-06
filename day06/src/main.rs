use aoc_utils::PuzzleInput;
const DAY: u8 = 6;

fn main() {
    let input = PuzzleInput::get_input(DAY);
    println!("A: {}", solve_a(&input));
    println!("B: {}", solve_b(&input));
}

fn find_first_unique_block(input:&PuzzleInput, length: usize) -> Option<usize> {
    for i in 0..input.raw_input.len() - length - 1 {
        let mut chars = input.raw_input.chars().skip(i).take(length).collect::<Vec<_>>();
        chars.sort();
        chars.dedup(); // dedup requires the chars to be sorted
        if chars.len() == length {
            return Some(i + length);
        }
    }
    None
}

fn solve_a(input: &PuzzleInput) -> usize {
    find_first_unique_block(input, 4)
    .expect("No start-of-packet marker found")
}

fn solve_b(input: &PuzzleInput) -> usize {
    find_first_unique_block(input, 14)
    .expect("No start-of-packet marker found")
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
        assert_eq!(solve_a(&PuzzleInput::new("bvwbjplbgvbhsrlpgdmjqwftvncz")), 5);
        assert_eq!(solve_a(&PuzzleInput::new("nppdvjthqldpwncqszvftbrmjlhg")), 6);
        assert_eq!(solve_a(&PuzzleInput::new("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg")), 10);
        assert_eq!(solve_a(&PuzzleInput::new("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw")), 11);
    }

    #[test]
    fn test_solve_b() {
        assert_eq!(solve_b(&PuzzleInput::new("mjqjpqmgbljsphdztnvjfqwrcgsmlb")), 19);
        assert_eq!(solve_b(&PuzzleInput::new("bvwbjplbgvbhsrlpgdmjqwftvncz")), 23);
        assert_eq!(solve_b(&PuzzleInput::new("nppdvjthqldpwncqszvftbrmjlhg")), 23);
        assert_eq!(solve_b(&PuzzleInput::new("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg")), 29);
        assert_eq!(solve_b(&PuzzleInput::new("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw")), 26);
    }
}
