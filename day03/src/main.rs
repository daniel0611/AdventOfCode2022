use aoc_utils::PuzzleInput;
const DAY: u8 = 3;

fn main() {
    let input = PuzzleInput::get_input(DAY);
    println!("A: {}", solve_a(&input));
    println!("B: {}", solve_b(&input));
}

fn calculate_priority_points(c: &char) -> u32 {
    match c {
        'a'..='z' => *c as u32 - 96,
        'A'..='Z' => *c as u32 - 38,
        _ => panic!("Invalid character"),
    }
}

#[derive(Clone)]
struct Rucksack {
    content: String,
}

impl Rucksack {
    fn new(content: String) -> Self {
        Self {
            content: content,
        }
    }

    fn calculate_priority(&self) -> u32 {
        let first_compartment: Vec<_> = self.content.chars().take(self.content.len() / 2).collect();
        let second_compartment: Vec<_> =
            self.content.chars().skip(self.content.len() / 2).collect();

        let duplicate = first_compartment
            .iter()
            .find(|c| second_compartment.contains(c))
            .unwrap();

        calculate_priority_points(duplicate)
    }
}

struct ElvGroup {
    rucksacks: [Rucksack; 3],
}

impl ElvGroup {
    fn new(rucksacks: Vec<&Rucksack>) -> Self {
        Self {
            rucksacks: [
                rucksacks[0].clone(),
                rucksacks[1].clone(),
                rucksacks[2].clone(),
            ],
        }
    }

    fn calculate_priority(&self) -> u32 {
        let badge = self.rucksacks[0]
            .content
            .chars()
            .find(|c| {
                self.rucksacks[1].content.contains(*c) && self.rucksacks[2].content.contains(*c)
            })
            .unwrap();

        calculate_priority_points(&badge)
    }
}

fn solve_a(input: &PuzzleInput) -> u32 {
    input
        .lines()
        .map(|line| Rucksack::new(line))
        .map(|rucksack| rucksack.calculate_priority())
        .sum()
}

fn solve_b(input: &PuzzleInput) -> u32 {
    let rucksacks: Vec<_> = input
        .lines()
        .map(|line| Rucksack::new(line))
        .collect();

    let mut count = 0;
    for i in 0..rucksacks.len() {
        if i % 3 != 0 {
            continue;
        }

        let elv_group = ElvGroup::new(rucksacks.iter().skip(i).take(3).collect());
        count += elv_group.calculate_priority();
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn test_no_panic() {
        let input = PuzzleInput::get_input(DAY);
        solve_a(&input);
        solve_b(&input);
    }

    #[test]
    fn test_solve_a() {
        assert_eq!(solve_a(&PuzzleInput::new(TEST_INPUT)), 157);
    }

    #[test]
    fn test_solve_b() {
        assert_eq!(solve_b(&PuzzleInput::new(TEST_INPUT)), 70);
    }
}
