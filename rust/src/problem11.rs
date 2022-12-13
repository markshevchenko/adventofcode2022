use std::collections::vec_deque::VecDeque;
use std::fmt::Debug;
use std::str::FromStr;
use num::integer::lcm;

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

fn parse_items(s: &str) -> VecDeque<u64> {
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
    items: VecDeque<u64>,
    operation: Operation,
    divisible_by: u64,
    if_true: usize,
    if_false: usize,
    total_inspects: usize,
}

impl Monkey {
    fn load(lines: &mut dyn Iterator<Item=String>) -> Self {
        debug_assert!(lines.next().unwrap().starts_with("Monkey"));
        let items = parse_items(&lines.next().unwrap());
        let operation = parse_operation(&lines.next().unwrap());
        let divisible_by = parse_number::<u64>(&lines.next().unwrap(), "  Test: divisible by ");
        let if_true = parse_number::<usize>(&lines.next().unwrap(), "    If true: throw to monkey ");
        let if_false = parse_number::<usize>(&lines.next().unwrap(), "    If false: throw to monkey ");
        let total_inspects = 0;

        Monkey { items, operation, divisible_by, if_true, if_false, total_inspects }
    }

    fn inspect<F>(&mut self, relax: F) -> Vec<(usize, u64)>  where F: Fn(u64) -> u64 {
        let mut result = Vec::new();
        self.total_inspects += self.items.len();
        while self.items.len() > 0 {
            let item = self.items.pop_front().unwrap();
            let worry_level = relax(self.operation.apply(item));
            let whom =
                if worry_level % self.divisible_by == 0 { self.if_true }
                else { self.if_false};

            result.push((whom, worry_level));
        }

        result
    }
}

struct Monkeys {
    monkeys: Vec<Monkey>,
    common_lcm: u64
}

impl Monkeys {
    fn load(lines: &mut dyn Iterator<Item=String>) -> Monkeys {
        let mut monkeys = vec![];

        loop {
            let monkey = Monkey::load(lines);
            monkeys.push(monkey);

            if lines.next() == None {
                break;
            }
        }

        let mut common_lcm = 1;
        for monkey in monkeys.iter() {
            common_lcm = lcm(common_lcm, monkey.divisible_by);
        }

        Monkeys { monkeys, common_lcm }
    }

    fn round1(&mut self) {
        let len = self.monkeys.len();
        for i in 0..len {
            let inspects = (&mut self.monkeys[i]).inspect(|a| a / 3u64);

            for (whom, worry_level) in inspects.iter() {
                (&mut self.monkeys[*whom]).items.push_back(*worry_level);
            }
        }
    }

    fn round2(&mut self) {
        let len = self.monkeys.len();
        let common_lcm = self.common_lcm;
        for i in 0..len {
            let inspects = (&mut self.monkeys[i]).inspect(|a| a % common_lcm);

            for (whom, worry_level) in inspects.iter() {
                (&mut self.monkeys[*whom]).items.push_back(*worry_level);
            }
        }
    }

    fn get_total_inspects(&self) -> Vec<usize> {
        self.monkeys.iter().map(|monkey| monkey.total_inspects).collect()
    }
}

pub fn solve_a(lines: &mut dyn Iterator<Item=String>) -> usize {
    let mut monkeys = Monkeys::load(lines);
    for _ in 0..20 {
        monkeys.round1();
    }

    let mut inspects = monkeys.get_total_inspects();
    inspects.sort_by(|a, b| b.cmp(a));
    inspects.iter()
        .take(2)
        .product()
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

    assert_eq!(10_605, actual);
}

pub fn solve_b(lines: &mut dyn Iterator<Item=String>) -> usize {
    let mut monkeys = Monkeys::load(lines);
    for _ in 0..10000 {
        monkeys.round2();
    }

    let mut inspects = monkeys.get_total_inspects();
    inspects.sort_by(|a, b| b.cmp(a));
    inspects.iter()
        .take(2)
        .product()
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

    assert_eq!(2_713_310_158_usize, actual);
}
