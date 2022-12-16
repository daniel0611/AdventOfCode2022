use std::collections::HashMap;

use aoc_utils::PuzzleInput;
use regex::Regex;
const DAY: u8 = 16;

fn main() {
    let input = PuzzleInput::get_input(DAY);
    println!("A: {}", solve_a(&input));
    println!("B: {}", solve_b(&input));
}

struct Valve {
    name: String,
    tunnels_to_valves: Vec<String>,
    flow_rate: u64,
}

impl Valve {
    fn parse(l: String) -> Self {
        let regex =
            Regex::new("Valve (.+) has flow rate=(.+); tunnels? leads? to valves? (.+)").unwrap();
        let captures = regex.captures(&l).unwrap();

        let name = captures.get(1).unwrap().as_str().to_owned();
        let flow_rate = captures.get(2).unwrap().as_str().parse().unwrap();
        let tunnels_to_valves = captures
            .get(3)
            .unwrap()
            .as_str()
            .split(", ")
            .map(|s| s.to_owned())
            .collect();

        Self {
            name,
            tunnels_to_valves,
            flow_rate,
        }
    }
}

fn get_flow(input: &[Valve], open: u64) -> u64 {
    input
        .iter()
        .enumerate()
        .map(|(i, valve)| {
            if open & (1 << i) != 0 {
                valve.flow_rate
            } else {
                0
            }
        })
        .sum::<u64>()
}

fn update<'a>(
    open: u64,
    flow: u64,
    new_open: u64,
    valve: &'a Valve,
    time: u64,
    flow_rate: &mut HashMap<(u64, &'a str, u64), u64>,
    input: &[Valve],
) {
    let new_flow = flow + get_flow(input, open);

    if let Some(old_flow) = flow_rate.get(&(time, &valve.name, new_open)) {
        if new_flow > *old_flow {
            flow_rate.insert((time, &valve.name, new_open), new_flow);
        }
    } else {
        flow_rate.insert((time, &valve.name, new_open), new_flow);
    }
}

fn solve_a(input: &PuzzleInput) -> u64 {
    let input: Vec<_> = input.lines().map(Valve::parse).collect();

    let idx: HashMap<_, _> = input
        .iter()
        .enumerate()
        .map(|(i, valve)| (valve.name.to_owned(), i))
        .collect();

    let mut flow_rate: HashMap<(u64, &str, u64), u64> = Default::default();
    flow_rate.insert((0, "AA", 0), 0);

    for time in 1..=30 {
        for valve in input.iter() {
            // move
            for old_valve in input
                .iter()
                .filter(|l| l.tunnels_to_valves.contains(&valve.name))
            {
                let prev: Vec<_> = flow_rate
                    .iter()
                    .filter(|((t, ol, _), _)| *t == time - 1 && **ol == old_valve.name)
                    .map(|((_t, _ol, open), flow)| (*open, *flow))
                    .collect();
                for (open, flow) in prev {
                    update(open, flow, open, valve, time, &mut flow_rate, &input);
                }
            }

            // open
            if valve.flow_rate > 0 {
                let prev: Vec<_> = flow_rate
                    .iter()
                    .filter(|((t, ol, open), _)| {
                        *t == time - 1 && **ol == valve.name && (open & (1 << idx[&valve.name]) == 0)
                    })
                    .map(|((_t, _ol, open), flow)| (*open, *flow))
                    .collect();
                for (open, flow) in prev {
                    let new_open: u64 = open | (1 << idx[&valve.name]);
                    update(open, flow, new_open, valve, time, &mut flow_rate, &input)
                }
            }
        }

        // clean past entries
        flow_rate.retain(|(t, _, _), _| *t == time);
    }

    *flow_rate.values().max().unwrap()
}

fn solve_b(input: &PuzzleInput) -> u64 {
    let input: Vec<_> = input.lines().map(Valve::parse).collect();

    let idx: HashMap<_, _> = input
        .iter()
        .enumerate()
        .map(|(i, valve)| (valve.name.to_owned(), i))
        .collect();

    let mut flow_rate: HashMap<(u64, &str, u64), u64> = Default::default();
    flow_rate.insert((0, "AA", 0), 0);

    for time in 1..=26 {
        for valve in input.iter() {
            // move
            for old_valve in input
                .iter()
                .filter(|l| l.tunnels_to_valves.contains(&valve.name))
            {
                let prev: Vec<_> = flow_rate
                    .iter()
                    .filter(|((t, ol, _), _)| *t == time - 1 && **ol == old_valve.name)
                    .map(|((_t, _ol, open), flow)| (*open, *flow))
                    .collect();
                for (open, flow) in prev {
                    update(open, flow, open, valve, time, &mut flow_rate, &input);
                }
            }

            // open
            if valve.flow_rate > 0 {
                let prev: Vec<_> = flow_rate
                    .iter()
                    .filter(|((t, ol, open), _)| {
                        *t == time - 1 && **ol == valve.name && (open & (1 << idx[&valve.name]) == 0)
                    })
                    .map(|((_t, _ol, open), flow)| (*open, *flow))
                    .collect();
                for (open, flow) in prev {
                    let new_open: u64 = open | (1 << idx[&valve.name]);
                    update(open, flow, new_open, valve, time, &mut flow_rate, &input)
                }
            }
        }

        // clean past entries
        flow_rate.retain(|(t, _, _), _| *t == time);
    }

    let mut max_flow = HashMap::new();
    for ((_t, _l, open), flow) in flow_rate.iter() {
            let s: &mut u64 = max_flow.entry(*open).or_default();
            *s = (*s).max(*flow);
    }

    let mut part2 = 0;
    for (&open1, &flow1) in max_flow.iter() {
        for (&open2, &flow2) in max_flow.iter() {
            if open1 & open2 == 0 {
                part2 = part2.max(flow1 + flow2);
            }
        }
    }

    part2
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II";

    #[test]
    fn test_solve_a() {
        assert_eq!(solve_a(&PuzzleInput::new(TEST_INPUT)), 1651);
    }

    #[test]
    fn test_solve_b() {
        assert_eq!(solve_b(&PuzzleInput::new(TEST_INPUT)), 1707);
    }
}
