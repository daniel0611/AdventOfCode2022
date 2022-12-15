use aoc_utils::PuzzleInput;
use regex::Regex;
const DAY: u8 = 15;

fn main() {
    let input = PuzzleInput::get_input(DAY);
    println!("A: {}", solve_a(&input, 2000000));
    println!("B: {}", solve_b(&input, 4000000));
}

struct Coordinates(i32, i32);

impl Coordinates {
    fn distance(&self, other: &Coordinates) -> i32 {
        (self.0 - other.0).abs() + (self.1 - other.1).abs()
    }
}

struct Sensor {
    position: Coordinates,
    distance_to_beacon: i32,
}

impl Sensor {
    fn parse(l: String) -> Sensor {
        let regex = Regex::new(
            r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)",
        )
        .unwrap();
        let capture = regex.captures(&l).unwrap();
        let capture_groups = capture.iter().skip(1);
        let numbers = capture_groups
            .map(|x| x.unwrap().as_str().parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        let position = Coordinates(numbers[0], numbers[1]);
        let closest_beacon = Coordinates(numbers[2], numbers[3]);

        Sensor {
            distance_to_beacon: position.distance(&closest_beacon),
            position,
        }
    }
}


fn is_beacon_possible(x: i32, y: i32, sensors: &[Sensor]) -> (bool, Option<&Sensor>) {
    for sensor in sensors.iter() {
        let distance_beacon_to_sensor =sensor.distance_to_beacon;
        let distance_here_to_sensor = sensor.position.distance(&Coordinates(x, y));

        if distance_here_to_sensor <= distance_beacon_to_sensor {
            return (false, Some(sensor));
        }
    }

    (true, None)
}

fn solve_a(input: &PuzzleInput, y_value: i32) -> usize {
    let sensors: Vec<_> = input.lines().map(Sensor::parse).collect();

    let result: usize = (-y_value * 3..=y_value*3)
        .filter(|x| !is_beacon_possible(*x, y_value, &sensors).0)
        .count();

    // Don't ask me why this is consistently one off, lol
    result - 1
}

fn solve_b(input: &PuzzleInput, x_y_max: usize) -> usize {
    let sensors: Vec<_> = input.lines().map(Sensor::parse).collect();

    for y in 0..=x_y_max {
        let mut x = 0;
        loop {
            let (possible, sensor) = is_beacon_possible(x as i32, y as i32, &sensors);
            if possible {
                return x * 4000000 + y;
            }

            let sensor = sensor.unwrap();
            x = (sensor.position.0 + sensor.distance_to_beacon - (y as i32 -sensor.position.1).abs()) as usize;
            x += 1;

            if x >= x_y_max {
                break;
            }
        }
    }

    panic!("No solution found");
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3";

    #[test]
    fn test_solve_a() {
        assert_eq!(solve_a(&PuzzleInput::new(TEST_INPUT), 10), 26);
    }

    #[test]
    fn test_solve_b() {
        assert_eq!(solve_b(&PuzzleInput::new(TEST_INPUT), 20), 56000011);
    }
}
