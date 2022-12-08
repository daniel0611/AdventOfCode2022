use aoc_utils::PuzzleInput;
const DAY: u8 = 8;

fn main() {
    let input = PuzzleInput::get_input(DAY);
    println!("A: {}", solve_a(&input));
    println!("B: {}", solve_b(&input));
}

fn parse_input(input: &PuzzleInput) -> Vec<Vec<u8>> {
    input
        .lines()
        .iter()
        .map(|line| {
            line.chars()
                .map(|c| c.to_string().parse().unwrap())
                .collect()
        })
        .collect()
}

#[derive(PartialEq)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

fn view_distance(tree_map: &Vec<Vec<u8>>, x: usize, y: usize, d: Direction) -> (usize, bool) {
    let height = tree_map[y][x];
    let mut trees_in_line: Vec<_> = match d {
        Direction::Left => tree_map[y].iter().take(x).collect(),
        Direction::Right => tree_map[y].iter().skip(x + 1).collect(),
        Direction::Up => tree_map.iter().take(y).map(|v| &v[x]).collect(),
        Direction::Down => tree_map.iter().skip(y + 1).map(|v| &v[x]).collect(),
    };

    if d == Direction::Left || d == Direction::Up {
        trees_in_line.reverse();
    }

    let mut view_distance = trees_in_line.iter().take_while(|v| ***v < height).count();
    let can_see_edge = view_distance == trees_in_line.len();
    if !can_see_edge {
        view_distance += 1;
    }

    (view_distance, can_see_edge)
}

fn is_visible(tree_map: &Vec<Vec<u8>>, x: usize, y: usize) -> bool {
    let (_, left) = view_distance(tree_map, x, y, Direction::Left);
    let (_, right) = view_distance(tree_map, x, y, Direction::Right);
    let (_, top) = view_distance(tree_map, x, y, Direction::Up);
    let (_, bottom) = view_distance(tree_map, x, y, Direction::Down);

    left || right || top || bottom
}

fn solve_a(input: &PuzzleInput) -> usize {
    let tree_map = parse_input(input);

    let outer_visible = 2 * tree_map.len() + 2 * tree_map[0].len() - 4;
    let inner_visible: usize = (1..tree_map.len() - 1)
        .map(|y| {
            (1..tree_map[0].len() - 1)
                .filter(|x| is_visible(&tree_map, *x, y))
                .count()
        })
        .sum();

    inner_visible + outer_visible
}

fn calculate_scenic_core(tree_map: &Vec<Vec<u8>>, x: usize, y: usize) -> usize {
    let (left_view, _) = view_distance(tree_map, x, y, Direction::Left);
    let (right_view, _) = view_distance(tree_map, x, y, Direction::Right);
    let (top_view, _) = view_distance(tree_map, x, y, Direction::Up);
    let (bottom_view, _) = view_distance(tree_map, x, y, Direction::Down);

    left_view * right_view * top_view * bottom_view
}

fn solve_b(input: &PuzzleInput) -> usize {
    let tree_map = parse_input(input);

    (1..tree_map.len() - 1)
        .map(|y| {
            (1..tree_map[0].len() - 1)
                .map(|x| calculate_scenic_core(&tree_map, x, y))
                .max()
                .unwrap()
        })
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "30373
25512
65332
33549
35390";

    #[test]
    fn test_no_panic() {
        let input = PuzzleInput::get_input(DAY);
        solve_a(&input);
        solve_b(&input);
    }

    #[test]
    fn test_solve_a() {
        assert_eq!(solve_a(&PuzzleInput::new(TEST_INPUT)), 21);
    }

    #[test]
    fn test_solve_b() {
        assert_eq!(solve_b(&PuzzleInput::new(TEST_INPUT)), 8);
    }
}
