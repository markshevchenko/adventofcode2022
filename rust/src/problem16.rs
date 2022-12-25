use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};
use lazy_static::lazy_static;
use regex::Regex;

#[derive(Clone)]
struct Valve {
    index: usize,
    rate: u32,
    tunnels: Vec<usize>,
}

fn parse_valves(lines: &mut dyn Iterator<Item=String>) -> (Vec<Valve>, usize) {
    lazy_static! {
        static ref REGEX: Regex = Regex::new(r"^Valve (\w\w) has flow rate=(\d+); tunnels? leads? to valves? (.*)$").unwrap();
    }

    let mut accumulator = Vec::new();
    let mut map = HashMap::new();
    
    for (index, line) in lines.enumerate() {
        let captures = REGEX.captures(&line).unwrap();
        
        accumulator.push((captures[1].to_string(), captures[2].to_string(), captures[3].to_string()));
        map.insert(captures[1].to_string(), index);
    }
    
    if accumulator.len() > 64 {
        panic!("Too many valves.");
    }

    let mut valves = Vec::new();
    for (name, rate, tunnels) in accumulator {
        let rate = rate.parse::<u32>().unwrap();
        let tunnels = tunnels.split(", ").map(|name| map[name]).collect::<Vec<_>>();
        
        valves.push(Valve { index: map[&name], rate, tunnels });
    }

    (valves, map[&"AA".to_string()])
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct Frame {
    valve_index: usize,
    total_pressure: u32,
    current_pressure: u32,
    minute: u32,
    opened: u64,
}

impl Hash for Frame {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.valve_index.hash(state);
        self.total_pressure.hash(state);
        self.current_pressure.hash(state);
        self.minute.hash(state);
        self.opened.hash(state);
    }
}

impl Frame {
    fn from_valve(valve: &Valve) -> Self {
        Frame {
            valve_index: valve.index,
            total_pressure: 0,
            current_pressure: 0,
            minute: 0,
            opened: 0,
        }
    }

    fn open(&self, valve: &Valve) -> Self {
        Frame {
            total_pressure: self.total_pressure + self.current_pressure,
            current_pressure: self.current_pressure + valve.rate,
            minute: self.minute + 1,
            opened: self.opened | (1 << valve.index),
            ..*self
        }
    }

    fn visit(&self, valve: &Valve) -> Self {
        Frame {
            valve_index: valve.index,
            total_pressure: self.total_pressure + self.current_pressure,
            minute: self.minute + 1,
            ..*self
        }
    }

    fn is_opened(&self) -> bool {
        self.opened & (1 << self.valve_index) != 0
    }
}

fn accelerated_bfs(valves: &[Valve], initial_index: usize) -> u32 {
    let mut queue = vec![Frame::from_valve(&valves[initial_index])];

    loop {
        let mut new_queue = HashSet::new();

        for frame in queue {
            let current_valve = &valves[frame.valve_index];
            if current_valve.rate > 0 && !frame.is_opened() {
                new_queue.insert(frame.open(current_valve));
            }
            
            for &next_index in &current_valve.tunnels {
                let next_valve = &valves[next_index];
                new_queue.insert(frame.visit(next_valve));
            }
        }

        if new_queue.is_empty() {
            panic!("Path not found.")
        }

        let mut new_queue = new_queue.into_iter().collect::<Vec<_>>();
        new_queue.sort_by(|frame1, frame2| {
            frame1.total_pressure.cmp(&frame2.total_pressure).reverse()
        });
        
        if new_queue[0].minute == 30 {
            return new_queue[0].total_pressure;
        }

        new_queue.truncate(30000);

        queue = new_queue;
    }
}

pub fn solve_a(lines: &mut dyn Iterator<Item=String>) -> u32 {
    let (valves, initial_index) = parse_valves(lines);

    accelerated_bfs(&valves, initial_index)
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

#[derive(Clone, PartialEq, Eq, Debug)]
struct Frame2 {
    valve_index_you: usize,
    valve_index_elephant: usize,
    total_pressure: u32,
    current_pressure: u32,
    minute: u32,
    opened: u64,
}

impl Hash for Frame2 {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.valve_index_you.hash(state);
        self.valve_index_elephant.hash(state);
        self.total_pressure.hash(state);
        self.current_pressure.hash(state);
        self.minute.hash(state);
        self.opened.hash(state);
    }
}

impl Frame2 {
    fn from_valve(valve: &Valve) -> Self {
        Frame2 {
            valve_index_you: valve.index,
            valve_index_elephant: valve.index,
            total_pressure: 0,
            current_pressure: 0,
            minute: 0,
            opened: 0,
        }
    }

    fn open(&self, valve: &Valve) -> Self {
        Frame2 {
            current_pressure: self.current_pressure + valve.rate,
            opened: self.opened | (1 << valve.index),
            ..*self
        }
    }

    fn visit_you(&self, valve: &Valve) -> Self {
        Frame2 {
            valve_index_you: valve.index,
            ..*self
        }
    }

    fn visit_elephant(&self, valve: &Valve) -> Self {
        Frame2 {
            valve_index_elephant: valve.index,
            ..*self
        }
    }
    
    fn next(&self) -> Self {
        Frame2 {
            total_pressure: self.total_pressure + self.current_pressure,
            minute: self.minute + 1,
            ..*self
        }
    }

    fn is_you_opened(&self) -> bool {
        self.opened & (1 << self.valve_index_you) != 0
    }

    fn is_elephant_opened(&self) -> bool {
        self.opened & (1 << self.valve_index_elephant) != 0
    }
}

fn accelerated_bfs2(valves: &[Valve], initial_index: usize) -> u32 {
    let mut queue = vec![Frame2::from_valve(&valves[initial_index])];

    loop {
        let mut new_queue = HashSet::new();

        for frame in queue {
            if frame.valve_index_you == frame.valve_index_elephant {
                let current_valve = &valves[frame.valve_index_you];
                if current_valve.rate > 0 && !frame.is_you_opened() {
                    for &next_index in &current_valve.tunnels {
                        let next_valve = &valves[next_index];
                        
                        new_queue.insert(frame.next().open(current_valve).visit_you(next_valve));
                        new_queue.insert(frame.next().open(current_valve).visit_elephant(next_valve));
                    }
                } else {
                    for &next_index1 in &current_valve.tunnels {
                        for &next_index2 in &current_valve.tunnels {
                            if next_index1 == next_index2 {
                                continue;
                            }

                            let next_valve1 = &valves[next_index1];
                            let next_valve2 = &valves[next_index2];

                            new_queue.insert(frame.next().visit_you(next_valve1).visit_elephant(next_valve2));
                            new_queue.insert(frame.next().visit_you(next_valve2).visit_elephant(next_valve1));
                        }
                    }
                }
            } else {
                let you_valve = &valves[frame.valve_index_you];
                let elephant_valve = &valves[frame.valve_index_elephant];

                let base = frame.next();
                let mut bases = Vec::new();

                if you_valve.rate > 0 && !frame.is_you_opened() {
                    bases.push(base.open(you_valve));
                }

                for &next_index in &you_valve.tunnels {
                    let next_valve = &valves[next_index];
                    bases.push(base.visit_you(next_valve));
                }
                
                for base in bases {
                    if elephant_valve.rate > 0 && !frame.is_elephant_opened() {
                        new_queue.insert(base.open(elephant_valve));
                    }

                    for &next_index in &elephant_valve.tunnels {
                        let next_valve = &valves[next_index];
                        new_queue.insert(base.visit_elephant(next_valve));
                    }
                }
            }
        }

        if new_queue.is_empty() {
            panic!("Path not found.")
        }

        let mut new_queue = new_queue.into_iter().collect::<Vec<_>>();
        new_queue.sort_by(|frame1, frame2| {
            frame1.total_pressure.cmp(&frame2.total_pressure).reverse()
        });

        if new_queue[0].minute == 26 {
            return new_queue[0].total_pressure;
        }

        new_queue.truncate(3000);

        queue = new_queue;
    }
}

pub fn solve_b(lines: &mut dyn Iterator<Item=String>) -> u32 {
    let (valves, initial_index) = parse_valves(lines);
    
    accelerated_bfs2(&valves, initial_index)
}

#[test]
fn solve_b_with_sample_data_returns_1707() {
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

    assert_eq!(1707, actual);
}
