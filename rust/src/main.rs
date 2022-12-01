mod problem01;
mod utils;

use std::env::{args};
use indoc::indoc;
use crate::utils::{stdin_lines};

fn main() {
    let problem_number = args().nth(1).and_then(|arg| arg.parse::<u32>().ok());

    if let Some(problem_number) = problem_number {
        if problem_number >= 1 && problem_number <= 50 {
            solve(problem_number);

            return;
        }
    }

    help();
}

fn solve(problem_number: u32) {
    match problem_number {
        1 => {
            println!("{}", problem01::solve(&mut stdin_lines()));
        },
        _ => {
            println!("Not yet implemented");
        }
    }
}

fn help() {
    println!(indoc!("
    Advent of Code 2022 (https://adventofcode.com)
    Run: adventofcode2022 problem_number
        problem_number is a number from 1 to 50

    The utility reads input from stdin and prints result to stdout.
    "));
}