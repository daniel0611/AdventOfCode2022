use std::ops::RangeInclusive;

use aoc_utils::PuzzleInput;
const DAY: u8 = 4;

fn main() {
    let input = PuzzleInput::get_input(DAY);
    println!("A: {}", solve_a(&input));
    println!("B: {}", solve_b(&input));
}

fn parse_input(input: &PuzzleInput) -> Vec<(RangeInclusive<usize>, RangeInclusive<usize>)>{
    input
    .lines()
    .map(|line| {
        let mut ranges = line.split(',')
            .map(|range| {
                let mut range = range.split('-');
                let start = range.next().unwrap().parse().unwrap();
                let end = range.next().unwrap().parse().unwrap();
                start..=end
            });
        
        (ranges.next().unwrap(), ranges.next().unwrap())
    })
    .collect()
}

fn solve_a(input: &PuzzleInput) -> usize {
    parse_input(input)
    .iter()
        .filter(|(r1, r2)| {
            // one range fully contains the other
            r1.contains(r2.start()) && r1.contains(r2.end()) || r2.contains(r1.start()) && r2.contains(r1.end())
        })
        .count()
}

fn solve_b(input: &PuzzleInput) -> usize {
    parse_input(input)
    .iter()
    .filter(|(r1, r2)| {
        // one range overlaps with the other
        r1.start() <= r2.end() && r2.start() <= r1.end()
    })
    .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn test_no_panic() {
        let input = PuzzleInput::get_input(DAY);
        solve_a(&input);
        solve_b(&input);
    }

    #[test]
    fn test_solve_a() {
        assert_eq!(solve_a(&PuzzleInput::new(TEST_INPUT)), 2);
    }

    #[test]
    fn test_solve_b() {
        assert_eq!(solve_b(&PuzzleInput::new(TEST_INPUT)), 4);
    }
}
