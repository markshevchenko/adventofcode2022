use std::collections::{HashMap};
use lazy_static::lazy_static;
use regex::Regex;

#[derive(Clone)]
struct Valve {
    rate: u32,
    tunnels: Vec<usize>,
    distances: HashMap<usize, usize>,
}

fn parse_line(line: String) -> (String, u32, Vec<String>) {
    lazy_static! {
        static ref REGEX: Regex = Regex::new(r"^Valve (\w\w) has flow rate=(\d+); tunnels? leads? to valves? (.*)$").unwrap();
    }

    let captures = REGEX.captures(&line).unwrap();
    let name = captures[1].to_string();
    let rate = captures[2].parse::<u32>().unwrap();
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

fn build_distance_matrix(valves: &[Valve], index: usize, distance: usize, columns: &[usize], row: &mut HashMap<usize, usize>) {
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

pub fn solve_a(lines: &mut dyn Iterator<Item=String>) -> usize {
    let valves = parse_valves(lines);

    for i in 0..valves.len() {
        print!("Index {}, Rate {},", i, valves[i].rate);

        for distance in valves[i].distances.iter() {
            print!("[{}, {}]", distance.0, distance.1);
        }

        println!("");
    }

    0
}

#[ignore]
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
