use aoc_utils::PuzzleInput;
use pathfinding::prelude::dijkstra;
const DAY: u8 = 12;
const START_VALUE: u32 = 0;
const END_VALUE: u32 = 27;

fn main() {
    let input = PuzzleInput::get_input(DAY);
    println!("A: {}", solve_a(&input));
    println!("B: {}", solve_b(&input));
}

fn parse_input(input: &PuzzleInput) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    'S' => START_VALUE,
                    'E' => END_VALUE,
                    c => c as u32 - 'a' as u32 + 1,
                })
                .collect()
        })
        .collect()
}

fn find_shortest_path(map: &[Vec<u32>], start_points: &[(usize, usize)]) -> u32  {
    let is_at_end = |(x, y): &(usize, usize)| map[*y][*x] == END_VALUE;

    start_points.iter()
    .map(|start| {
        let result = dijkstra(
            start,
            |&(x, y)| {
                let mut neighbors = vec![];
                let current_weight = map[y][x];
    
                // Left
                if x > 0 && map[y][x-1] <= current_weight + 1 {
                    neighbors.push(((x-1, y), 1));
                }
    
                // Right
                if x < map[y].len() - 1 && map[y][x+1] <= current_weight + 1 {
                    neighbors.push(((x+1, y), 1));
                }
    
                // Up
                if y > 0 && map[y-1][x] <= current_weight + 1 {
                    neighbors.push(((x, y-1), 1));
                }
    
                // Down
                if y < map.len() - 1 && map[y+1][x] <= current_weight + 1 {
                    neighbors.push(((x, y+1), 1));
                }
    
                neighbors
            },
            is_at_end,
        );
    
        result.map(|(_, cost)| cost).unwrap_or(u32::MAX)
    })
    .min()
    .unwrap()
}

fn solve_a(input: &PuzzleInput) -> u32 {
    let map = parse_input(input);

    let start = map
        .iter()
        .enumerate()
        .find(|(_, row)| row.contains(&START_VALUE))
        .map(|(y, row)| (row.iter().position(|&c| c == START_VALUE).unwrap(), y))
        .unwrap();

    find_shortest_path(&map, &[start])
}

fn solve_b(input: &PuzzleInput) -> u32 {
    let map = parse_input(input);

    let starting_points = map
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, &c)| c == 0 || c == 1)
                .map(move |(x, _)| (x, y))
        })
        .collect::<Vec<_>>();
    
    find_shortest_path(&map, &starting_points)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

    #[test]
    fn test_no_panic() {
        let input = PuzzleInput::get_input(DAY);
        solve_a(&input);
        //solve_b(&input); //takes too long (about 11 seconds)
    }

    #[test]
    fn test_solve_a() {
        assert_eq!(solve_a(&PuzzleInput::new(TEST_INPUT)), 31);
    }

    #[test]
    fn test_solve_b() {
        assert_eq!(solve_b(&PuzzleInput::new(TEST_INPUT)), 29);
    }
}
