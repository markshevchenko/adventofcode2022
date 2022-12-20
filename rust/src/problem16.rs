use std::collections::{HashSet, HashMap};
use std::hash::{Hash, Hasher};
use lazy_static::lazy_static;
use regex::Regex;

struct Graph {
    rates: Vec<u32>,
    leads: Vec<Vec<usize>>,
}

impl Graph {
    fn parse_line(line: &String) -> (String, u32, Vec<String>) {
        lazy_static! {
            static ref REGEX: Regex = Regex::new(r"^Valve (\w\w) has flow rate=(\d+); tunnels? leads? to valves? (.*)$").unwrap();
        }

        let captures = REGEX.captures(line).unwrap();
        let name = captures[1].to_string();
        let rate = captures[2].parse::<u32>().unwrap();
        let names = captures[3].split(", ").map(|s| s.to_string()).collect::<Vec<String>>();

        (name, rate, names)
    }
    
    fn parse(lines: &mut dyn Iterator<Item=String>) -> Self {
        // let mut indexes = HashMap::new();
        let mut rates = Vec::new();
        let mut leads = Vec::new();
        
        // for line in lines {
        //     let (name, rate, names) = Graph::parse_line(&line);
        //     let index = if indexes.contains_key(&name) {
        //         indexes[&name]
        //     } else {
        //         let index = indexes.len();
        //         indexes.insert(&name, index);
        //
        //         index
        //     };
        //
        //     let mut lead_indexes = Vec::new();
        //
        //     for name in names {
        //         let index = if indexes.contains_key(&name) {
        //             indexes[&name]
        //         } else {
        //             let index = indexes.len();
        //             indexes.insert(&name, index);
        //
        //             index
        //         };
        //
        //         lead_indexes.push(index);
        //     }
        //
        //     rates.push(rate);
        //     leads.push(lead_indexes);
        // }

        Graph { rates, leads }
    }
}

pub fn solve_a(lines: &mut dyn Iterator<Item=String>) -> usize {
    let graph = Graph::parse(lines);
    
    
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
    let _graph = Graph::parse(lines);
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
