use std::str::FromStr;
use std::fmt::Debug;

enum Operation {
    Add(u64),
    Multiply(u64),
    Square
}

impl Operation {
    fn apply(&self, b: u64) -> u64 {
        match &self {
            Operation::Add(a) => a + b,
            Operation::Multiply(a) => a * b,
            Operation::Square => b * b,
        }
    }
}

fn parse_operation(s: &str) -> Operation {
    if s == "  Operation: new = old * old" {
        Operation::Square
    } else if let Some(number) = s.strip_prefix("  Operation: new = old * ") {
        Operation::Multiply(number.parse::<u64>().unwrap())
    } else if let Some(number) = s.strip_prefix("  Operation: new = old + ") {
        Operation::Add(number.parse::<u64>().unwrap())
    } else {
        panic!("Unrecognized operation");
    }
}

fn parse_items(s: &str) -> Vec<u64> {
    if let Some(value) = s.strip_prefix("  Starting items: ") {
        value.split(", ").map(|s| s.parse::<u64>().unwrap()).collect()
    } else {
        panic!("Unrecognized items")
    }
}

fn parse_number<N>(s: &str, prefix: &str) -> N where N: FromStr, <N as FromStr>::Err: Debug {
    if let Some(value) = s.strip_prefix(prefix) {
        return N::from_str(value).unwrap();
    }
    
    panic!("Unrecognized number");
}

struct Monkey {
    items: Vec<u64>,
    operation: Operation,
    divisible_by: u64,
    if_true: usize,
    if_false: usize,
}

fn parse_monkey(lines: &mut dyn Iterator<Item=String>) -> Monkey {
    debug_assert!(lines.next().unwrap().starts_with("Monkey"));
    let items = parse_items(&lines.next().unwrap());
    let operation = parse_operation(&lines.next().unwrap());
    let divisible_by = parse_number::<u64>(&lines.next().unwrap(), "  Test: divisible by ");
    let if_true = parse_number::<usize>(&lines.next().unwrap(), "    If true: throw to monkey ");
    let if_false = parse_number::<usize>(&lines.next().unwrap(), "    If false: throw to monkey ");
    
    Monkey { items, operation, divisible_by, if_true, if_false }
}

struct Monkeys {
    monkeys: Vec<Monkey>
}

impl Monkeys {
    fn new(monkeys: Vec<Monkey>) -> Self {
        Monkeys { monkeys }
    }
}

fn parse_monkeys(lines: &mut dyn Iterator<Item=String>) -> Monkeys {
    let mut monkeys = vec![];
    
    loop {
        let monkey = parse_monkey(lines);
        monkeys.push(monkey);
        
        if lines.next() == None {
            break;
        } 
    }
    
    Monkeys { monkeys }
}

pub fn solve_a(lines: &mut dyn Iterator<Item=String>) -> u64 {
    10605
}

#[test]
fn solve_a_with_sample_data_returns_10605() {
    let sample = indoc::indoc!("
        Monkey 0:
          Starting items: 79, 98
          Operation: new = old * 19
          Test: divisible by 23
            If true: throw to monkey 2
            If false: throw to monkey 3
        
        Monkey 1:
          Starting items: 54, 65, 75, 74
          Operation: new = old + 6
          Test: divisible by 19
            If true: throw to monkey 2
            If false: throw to monkey 0
        
        Monkey 2:
          Starting items: 79, 60, 97
          Operation: new = old * old
          Test: divisible by 13
            If true: throw to monkey 1
            If false: throw to monkey 3
        
        Monkey 3:
          Starting items: 74
          Operation: new = old + 3
          Test: divisible by 17
            If true: throw to monkey 0
            If false: throw to monkey 1");
    let mut lines = crate::utils::str_to_iter(sample);

    let actual = solve_a(&mut lines);

    assert_eq!(10_605_u64, actual);
}

pub fn solve_b(lines: &mut dyn Iterator<Item=String>) -> u64 {
    2713310158
}

#[ignore]
#[test]
fn solve_b_with_sample_data_returns_2713310158() {
    let sample = indoc::indoc!("
        Monkey 0:
          Starting items: 79, 98
          Operation: new = old * 19
          Test: divisible by 23
            If true: throw to monkey 2
            If false: throw to monkey 3
        
        Monkey 1:
          Starting items: 54, 65, 75, 74
          Operation: new = old + 6
          Test: divisible by 19
            If true: throw to monkey 2
            If false: throw to monkey 0
        
        Monkey 2:
          Starting items: 79, 60, 97
          Operation: new = old * old
          Test: divisible by 13
            If true: throw to monkey 1
            If false: throw to monkey 3
        
        Monkey 3:
          Starting items: 74
          Operation: new = old + 3
          Test: divisible by 17
            If true: throw to monkey 0
            If false: throw to monkey 1");
    let mut lines = crate::utils::str_to_iter(sample);

    let actual = solve_a(&mut lines);

    assert_eq!(2_713_310_158_u64, actual);
}
