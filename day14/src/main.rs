use aoc_utils::PuzzleInput;
const DAY: u8 = 14;

fn main() {
    let input = PuzzleInput::get_input(DAY);
    println!("A: {}", solve_a(&input));
    println!("B: {}", solve_b(&input));
}

struct Point(usize, usize);

struct RockLines {
    points: Vec<Point>,
}

impl RockLines {
    fn parse(l: String) -> RockLines {
        RockLines {
            points: l
                .split(" -> ")
                .map(|s| {
                    let mut it = s.split(',').map(|s| s.parse::<usize>().unwrap());
                    Point(it.next().unwrap(), it.next().unwrap())
                })
                .collect(),
        }
    }
}
#[derive(Clone, PartialEq)]
enum Block {
    Air,
    Rock,
    Sand,
}

fn parse_input(input: &PuzzleInput) -> Vec<Vec<Block>> {
    let rocks: Vec<_> = input
        .lines()
        .map(RockLines::parse)
        .collect();

    let mut grid = vec![vec![Block::Air; 1000]; 1000];

    for rock_line in rocks.iter() {
        let mut start_point = &rock_line.points[0];

        for end_point in &rock_line.points[1..] {
            let Point(start_x, start_y) = *start_point;
            let Point(end_x, end_y) = *end_point;
            let (min_x, max_x) = (start_x.min(end_x), start_x.max(end_x));
            let (min_y, max_y) = (start_y.min(end_y), start_y.max(end_y));

            if start_x == end_x {
                for row in grid.iter_mut().take(max_y + 1).skip(min_y) {
                    row[start_x] = Block::Rock;
                }
            } else {
                for x in min_x..=max_x {
                    grid[start_y][x] = Block::Rock;
                }
            }
            start_point = end_point;
        }
    }

    grid
}

fn place_sand(grid: &mut [Vec<Block>]) -> bool {
    let mut sand_pos = Point(500, 0);
    if grid[0][500] != Block::Air {
        return false;
    }

    for _ in 0..grid.len() - 1 {
        let Point(x, y) = sand_pos;
        if grid[y + 1][x] == Block::Air {
            sand_pos = Point(x, y + 1);
            continue;
        }

        if grid[y + 1][x - 1] == Block::Air {
            sand_pos = Point(x - 1, y + 1);
            continue;
        }

        if grid[y + 1][x + 1] == Block::Air {
            sand_pos = Point(x + 1, y + 1);
            continue;
        }

        assert!(grid[y + 1][x] == Block::Rock || grid[y + 1][x] == Block::Sand);
        grid[y][x] = Block::Sand;
        return true;
    }

    false
}

fn solve_a(input: &PuzzleInput) -> usize {
    let mut grid = parse_input(input);

    let mut sand_count = 0;
    while place_sand(&mut grid) {
        sand_count += 1;
    }

    sand_count
}

fn solve_b(input: &PuzzleInput) -> usize {
    let mut grid = parse_input(input);

    let highest_sand_y = grid
        .iter()
        .enumerate()
        .filter(|(_, row)| row.iter().any(|b| *b == Block::Rock))
        .map(|(y, _)| y)
        .max()
        .unwrap();

    for x in 0..grid[highest_sand_y].len() {
        grid[highest_sand_y + 2][x] = Block::Rock;
    }

    let mut sand_count = 0;
    while place_sand(&mut grid) {
        sand_count += 1;
    }

    sand_count
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "498,4 -> 498,6 -> 496,6\n503,4 -> 502,4 -> 502,9 -> 494,9";

    #[test]
    fn test_no_panic() {
        let input = PuzzleInput::get_input(DAY);
        solve_a(&input);
        solve_b(&input);
    }

    #[test]
    fn test_solve_a() {
        assert_eq!(solve_a(&PuzzleInput::new(TEST_INPUT)), 24);
    }

    #[test]
    fn test_solve_b() {
        assert_eq!(solve_b(&PuzzleInput::new(TEST_INPUT)), 93);
    }
}
