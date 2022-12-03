#[derive(Copy, Clone)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl Shape {
    fn score(&self) -> u32 {
        match self {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        }
    }

    fn from_char1(c: u8) -> Self {
        match c {
            b'A' => Shape::Rock,
            b'B' => Shape::Paper,
            b'C' => Shape::Scissors,
            _ => panic!("Unknown letter for player 1"),
        }
    }

    fn from_char2(c: u8) -> Self {
        match c {
            b'X' => Shape::Rock,
            b'Y' => Shape::Paper,
            b'Z' => Shape::Scissors,
            _ => panic!("Unknown letter for player 2"),
        }
    }
}


#[derive(Copy, Clone)]
enum Outcome {
    Defeat,
    Draw,
    Win,
}

impl Outcome {
    fn score(&self) -> u32 {
        match self {
            Outcome::Defeat => 0,
            Outcome::Draw => 3,
            Outcome::Win => 6,
        }
    }

    fn from_char(c: u8) -> Self {
        match c {
            b'X' => Outcome::Defeat,
            b'Y' => Outcome::Draw,
            b'Z' => Outcome::Win,
            _ => panic!("Unknown letter for outcome"),
        }
    }
}


fn player2_outcome(player1: Shape, player2: Shape) -> Outcome {
    match (player1, player2) {
        (Shape::Rock, Shape::Rock) => Outcome::Draw,
        (Shape::Rock, Shape::Paper) => Outcome::Win,
        (Shape::Rock, Shape::Scissors) => Outcome::Defeat,
        (Shape::Paper, Shape::Rock) => Outcome::Defeat,
        (Shape::Paper, Shape::Paper) => Outcome::Draw,
        (Shape::Paper, Shape::Scissors) => Outcome::Win,
        (Shape::Scissors, Shape::Rock) => Outcome::Win,
        (Shape::Scissors, Shape::Paper) => Outcome::Defeat,
        (Shape::Scissors, Shape::Scissors) => Outcome::Draw,
    }
}


pub fn solve_a(lines: &mut dyn Iterator<Item = String>) -> u32 {
    let mut result = 0;

    for line in lines {
        let line_as_bytes = line.as_bytes();
        let player1 = Shape::from_char1(line_as_bytes[0]);
        let player2 = Shape::from_char2(line_as_bytes[2]);
        let outcome = player2_outcome(player1, player2);

        result += outcome.score() + player2.score();
    }

    result
}

#[test]
fn solve_a_with_sample_data_returns_15() {
    let sample = indoc::indoc!("
        A Y
        B X
        C Z");
    let mut lines = crate::utils::str_to_iter(sample);

    let actual = solve_a(&mut lines);

    assert_eq!(15, actual);
}


fn player2_shape(player1: Shape, outcome: Outcome) -> Shape {
    match (player1, outcome) {
        (Shape::Rock, Outcome::Defeat) => Shape::Scissors,
        (Shape::Rock, Outcome::Draw) => Shape::Rock,
        (Shape::Rock, Outcome::Win) => Shape::Paper,
        (Shape::Paper, Outcome::Defeat) => Shape::Rock,
        (Shape::Paper, Outcome::Draw) => Shape::Paper,
        (Shape::Paper, Outcome::Win) => Shape::Scissors,
        (Shape::Scissors, Outcome::Defeat) => Shape::Paper,
        (Shape::Scissors, Outcome::Draw) => Shape::Scissors,
        (Shape::Scissors, Outcome::Win) => Shape::Rock,
    }
}

pub fn solve_b(lines: &mut dyn Iterator<Item = String>) -> u32 {
    let mut result = 0;

    for line in lines {
        let line_as_bytes = line.as_bytes();
        let player1 = Shape::from_char1(line_as_bytes[0]);
        let outcome = Outcome::from_char(line_as_bytes[2]);
        let player2 = player2_shape(player1, outcome);

        result += outcome.score() + player2.score();
    }

    result
}

#[test]
fn solve_b_with_sample_data_returns_12() {
    let sample = indoc::indoc!("
        A Y
        B X
        C Z");
    let mut lines = crate::utils::str_to_iter(sample);

    let actual = solve_b(&mut lines);

    assert_eq!(12, actual);
}
