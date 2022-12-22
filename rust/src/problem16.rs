use std::cmp::Ordering;
// use std::collections::BinaryHeap;
use std::collections::{HashMap, VecDeque};
use lazy_static::lazy_static;
use regex::Regex;

#[derive(Clone)]
struct Valve {
    rate: i32,
    tunnels: Vec<usize>,
    distances: HashMap<usize, i32>,
}

fn parse_line(line: String) -> (String, i32, Vec<String>) {
    lazy_static! {
        static ref REGEX: Regex = Regex::new(r"^Valve (\w\w) has flow rate=(\d+); tunnels? leads? to valves? (.*)$").unwrap();
    }

    let captures = REGEX.captures(&line).unwrap();
    let name = captures[1].to_string();
    let rate = captures[2].parse::<i32>().unwrap();
    let names = captures[3].split(", ").map(|s| s.to_string()).collect::<Vec<String>>();

    (name, rate, names)
}

fn parse_valves(lines: &mut dyn Iterator<Item=String>) -> Vec<Valve> {
    let descriptors: Vec<_> = lines.map(parse_line).collect();
    let mut indexes = HashMap::new();
    for index in 0..descriptors.len() {
        let name = descriptors[index].0.clone();
        indexes.insert(name, index);
    }

    let mut result = Vec::new();
    for descriptor in descriptors.iter() {
        let rate = descriptor.1;

        let mut tunnels = Vec::new();
        for name in &descriptor.2 {
            let index = indexes[name];
            tunnels.push(index);
        }

        result.push(Valve { rate, tunnels, distances: HashMap::new() });
    }

    for index in 0..result.len() {
        let mut distances = HashMap::new();
        build_distance_matrix(&result, index, 1, &result[index].tunnels, &mut distances);
        result[index].distances = distances;
    }

    result
}

fn build_distance_matrix(valves: &[Valve], index: usize, distance: i32, columns: &[usize], row: &mut HashMap<usize, i32>) {
    let mut next_columns = Vec::new();
    for column in columns {
        if *column == index {
            continue;
        }

        if row.contains_key(&column) {
            continue;
        }

        row.insert(*column, distance);

        for tunnel in &valves[*column].tunnels {
            next_columns.push(*tunnel);
        }
    }

    if next_columns.is_empty() {
        return;
    }

    build_distance_matrix(valves, index, distance + 1, &next_columns, row);
}

#[derive(Copy, Clone, Eq, Ord)]
struct State {
    priority: i32,
    total_pressure: i32,
    current_pressure: i32,
    minute: i32,
    opened: u128,
    visited: u128,
    index: usize,
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.priority.eq(&other.priority)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.priority.partial_cmp(&other.priority)
    }
}

impl State {
    fn new() -> Self {
        State {
            priority: 0,
            total_pressure: 0,
            current_pressure: 0,
            minute: 0,
            opened: 0,
            visited: 0,
            index: 0,
        }
    }

    fn open(&self, rate: i32) -> Self {
        State {
            priority: 0,
            total_pressure: self.total_pressure + self.current_pressure,
            current_pressure: self.current_pressure + rate,
            minute: self.minute + 1,
            opened: self.opened | (1 << self.index),
            visited: 1 << self.index,
            ..*self
        }
    }

    fn visit(&self, _rate: i32, index: usize) -> Self {
        State {
            priority: 0,
            total_pressure: self.total_pressure + self.current_pressure,
            minute: self.minute + 1,
            visited: self.visited | (1 << index),
            index,
            ..*self
        }
    }

    fn is_opened(&self, index: usize) -> bool {
        self.opened & (1 << index) != 0
    }

    fn is_visited(&self, index: usize) -> bool {
        self.visited & (1 << index) != 0
    }
}

fn a_star(valves: &[Valve]) -> i32 {
    if valves.len() > 128 {
        panic!("Too many valves.");
    }

    // let mut priority_queue = BinaryHeap::new();
    let mut queue = VecDeque::new();
    // priority_queue.push(State::new());
    queue.push_back(State::new());

    let mut max_total_pressure = 0;

    // while let Some(state) = priority_queue.pop() {
    while let Some(state) = queue.pop_front() {
        if state.minute == 30 {
            if max_total_pressure < state.total_pressure {
                max_total_pressure = state.total_pressure;
            }

            continue;
        }

        let current_index = state.index;
        let current_valve = &valves[current_index];
        if current_valve.rate > 0 && !state.is_opened(current_index) {
            let next_state = state.open(current_valve.rate);
            // priority_queue.push(next_state);
            queue.push_back(next_state);
            // println!("Open {} (rate {}): priority {}, minute {}, total: {}, current: {}",
            //          current_index,
            //          current_valve.rate,
            //          next_state.priority,
            //          next_state.minute,
            //          next_state.total_pressure,
            //          next_state.current_pressure);
        }

        for &next_index in current_valve.tunnels.iter() {
            if state.is_visited(next_index) {
                continue;
            }

            let next_valve = &valves[next_index];
            let next_state = state.visit(next_valve.rate, next_index);
            // priority_queue.push(next_state);
            queue.push_back(next_state);
            // println!("Visit {} -> {} (rate {}): priority: {}, minute {}, total: {}, current: {}",
            //          current_index,
            //          next_index,
            //          next_valve.rate,
            //          next_state.priority,
            //          next_state.minute,
            //          next_state.total_pressure,
            //          next_state.current_pressure);
        }
    }

    max_total_pressure
}

pub fn solve_a(lines: &mut dyn Iterator<Item=String>) -> i32 {
    let valves = parse_valves(lines);

    a_star(&valves)
}

#[test]
fn solve_a_with_sample_data_returns_1651() {
    let sample = indoc::indoc!("
        Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
        Valve BB has flow rate=13; tunnels lead to valves CC, AA
        Valve CC has flow rate=2; tunnels lead to valves DD, BB
        Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
        Valve EE has flow rate=3; tunnels lead to valves FF, DD
        Valve FF has flow rate=0; tunnels lead to valves EE, GG
        Valve GG has flow rate=0; tunnels lead to valves FF, HH
        Valve HH has flow rate=22; tunnel leads to valve GG
        Valve II has flow rate=0; tunnels lead to valves AA, JJ
        Valve JJ has flow rate=21; tunnel leads to valve II");
    let mut lines = crate::utils::str_to_iter(sample);

    let actual = solve_a(&mut lines);

    assert_eq!(1651, actual);
}

pub fn solve_b(lines: &mut dyn Iterator<Item=String>) -> usize {
    let _valves = parse_valves(lines);
    0
}

#[test]
fn solve_b_with_sample_data_returns_0() {
    let sample = indoc::indoc!("
        Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
        Valve BB has flow rate=13; tunnels lead to valves CC, AA
        Valve CC has flow rate=2; tunnels lead to valves DD, BB
        Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
        Valve EE has flow rate=3; tunnels lead to valves FF, DD
        Valve FF has flow rate=0; tunnels lead to valves EE, GG
        Valve GG has flow rate=0; tunnels lead to valves FF, HH
        Valve HH has flow rate=22; tunnel leads to valve GG
        Valve II has flow rate=0; tunnels lead to valves AA, JJ
        Valve JJ has flow rate=21; tunnel leads to valve II");
    let mut lines = crate::utils::str_to_iter(sample);

    let actual = solve_b(&mut lines);

    assert_eq!(0, actual);
}
