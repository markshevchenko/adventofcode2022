use std::collections::HashSet;
use text_io::scan;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    row: i32,
    column: i32,
}

#[derive(Clone, Copy)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Position {
    fn move_to(&self, direction: Direction) -> Self {
        match direction {
            Direction::Up => Position { row: self.row + 1, column: self.column },
            Direction::Right => Position { row: self.row, column: self.column + 1 },
            Direction::Down => Position { row: self.row - 1, column: self.column },
            Direction::Left => Position { row: self.row, column: self.column - 1 },
        }
    }

    fn move_tail(&self, tail: &Position) -> Position {
        match (tail.row - self.row, tail.column - self.column) {
            (2, 2) => self.move_to(Direction::Up).move_to(Direction::Right),
            (2, -2) => self.move_to(Direction::Up).move_to(Direction::Left),
            (-2, 2) => self.move_to(Direction::Down).move_to(Direction::Right),
            (-2, -2) => self.move_to(Direction::Down).move_to(Direction::Left),
            (2, _) => self.move_to(Direction::Up),
            (-2, _) => self.move_to(Direction::Down),
            (_, 2) => self.move_to(Direction::Right),
            (_, -2) => self.move_to(Direction::Left),
            _ => *tail
        }
    }
}

fn parse(line: &String) -> (Direction, u32) {
    let c: char;
    let steps: u32;

    scan!(line.bytes() => "{} {}", c, steps);
    let direction = match c {
        'U' => Direction::Up,
        'R' => Direction::Right,
        'D' => Direction::Down,
        'L' => Direction::Left,
        _ => panic!("Unrecognized direction")
    };

    (direction, steps)
}

fn solve(lines: &mut dyn Iterator<Item=String>, rope_size: usize) -> usize {
    let mut rope = vec![Position { row: 0, column: 0}; rope_size];
    let mut tails = HashSet::new();
    tails.insert(rope.last().unwrap().clone());

    for line in lines {
        let (direction, steps) = parse(&line);
        for _ in 1..=steps {
            rope[0] = rope[0].move_to(direction);
            for i in 1..rope.len() {
                rope[i] = rope[i - 1].move_tail(&rope[i]);
            }

            tails.insert(rope.last().unwrap().clone());
        }
    }

    tails.len()
}

pub fn solve_a(lines: &mut dyn Iterator<Item=String>) -> usize {
    solve(lines, 2)
}

#[test]
fn solve_a_with_sample_data_returns_13() {
    let sample = indoc::indoc!("
        R 4
        U 4
        L 3
        D 1
        R 4
        D 1
        L 5
        R 2");
    let mut lines = crate::utils::str_to_iter(sample);

    let actual = solve_a(&mut lines);

    assert_eq!(13, actual);
}

pub fn solve_b(lines: &mut dyn Iterator<Item=String>) -> usize {
    solve(lines, 10)
}

#[test]
fn solve_b_with_sample_data_returns_1() {
    let sample = indoc::indoc!("
        R 4
        U 4
        L 3
        D 1
        R 4
        D 1
        L 5
        R 2");
    let mut lines = crate::utils::str_to_iter(sample);

    let actual = solve_b(&mut lines);

    assert_eq!(1, actual);
}

#[test]
fn solve_b_with_second_sample_data_returns_36() {
    let sample = indoc::indoc!("
        R 5
        U 8
        L 8
        D 3
        R 17
        D 10
        L 25
        U 20");
    let mut lines = crate::utils::str_to_iter(sample);

    let actual = solve_b(&mut lines);

    assert_eq!(36, actual);
}
