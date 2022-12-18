use std::collections::{HashSet};
use std::hash::{Hash, Hasher};
use lazy_static::lazy_static;
use regex::Regex;

#[derive(Eq)]
struct Valve {
    #[allow(dead_code)]
    name: String,
    #[allow(dead_code)]
    rate: u32,
    #[allow(dead_code)]
    leads: Vec<String>,
}

impl PartialEq<Self> for Valve {
    fn eq(&self, other: &Self) -> bool {
        self.name.eq(&other.name)
    }
}

impl Hash for Valve {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

impl Valve {
    fn parse(s: &str) -> Self {
        lazy_static! {
            static ref REGEX: Regex = Regex::new(r"^Valve (\w\w) has flow rate=(\d+); tunnels? leads? to valves? (.*)$").unwrap();
        }
        
        let captures = REGEX.captures(s).unwrap();
        let name = captures[1].to_string();
        let rate = captures[2].parse::<u32>().unwrap();
        let leads = captures[3].split(", ").map(|s| s.to_string()).collect::<Vec<String>>();
        
        Valve { name, rate, leads }
    }
}

struct Graph {
    #[allow(dead_code)]
    start: String,
    #[allow(dead_code)]
    valves: HashSet<Valve>
}

impl Graph {
    fn parse(lines: &mut dyn Iterator<Item=String>) -> Self {
        let mut valves = HashSet::new();
        let mut start = "".to_string();
        for line in lines {
            let valve = Valve::parse(&line);
            if start == "" {
                start = valve.name.clone();
            }
            valves.insert(valve);
        }

        Graph { start, valves }
    }
}

pub fn solve_a(lines: &mut dyn Iterator<Item=String>) -> usize {
    let _graph = Graph::parse(lines);
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
