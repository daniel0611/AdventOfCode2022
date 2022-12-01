use aoc_utils::PuzzleInput;
const DAY: u8 = 1;

fn main() {
    let input = PuzzleInput::get_input(DAY);
    println!("A: {}", solve_a(&input));
    println!("B: {}", solve_b(&input));
}

fn create_snack_count_list(input: &PuzzleInput) -> Vec<u32> {
    input
        .raw_input
        .split("\n\n")
        .map(|elf_lines| {
            elf_lines
                .split('\n')
                .filter(|line| !line.is_empty())
                .map(|line| 
                    line.parse::<u32>().unwrap()
        )
                .sum::<u32>()
        })
        .collect()
}

fn solve_a(input: &PuzzleInput) -> u32 {
*create_snack_count_list(input)
        .iter()
        .max()
        .unwrap()
}

fn solve_b(input: &PuzzleInput) -> u32 {
    let mut list = create_snack_count_list(input);
    list.sort();
    list.reverse(); // descending order
    list.iter().take(3).sum::<u32>()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

 #[test]
    fn test_no_panic() {
        let input = PuzzleInput::get_input(DAY);
        solve_a(&input);
        solve_b(&input);
    }

    #[test]
    fn test_solve_a() {
        assert_eq!(solve_a(&PuzzleInput::new(TEST_INPUT)), 24000);
    }

    #[test]
    fn test_solve_b() {
        assert_eq!(solve_b(&PuzzleInput::new(TEST_INPUT)), 45000);
    }
}
