mod problem01;
mod utils;

use std::env::{args};
use indoc::indoc;
use crate::utils::{stdin_lines};

fn main() {
    let valid_problems = vec!["1a", "1b"];

    if let Some(problem) = args().nth(1) {
        let problem = problem.as_str();
        if valid_problems.contains(&problem) {
            solve(problem);

            return;
        }
    }

    help();
}

fn solve(problem: &str) {
    match problem {
        "1a" => {
            println!("{}", problem01::solve_a(&mut stdin_lines()));
        },
        "1b" => {
            println!("{}", problem01::solve_b(&mut stdin_lines()));
        },
        _ => {
            println!("Not yet implemented");
        }
    }
}

fn help() {
    println!(indoc!("
    Advent of Code 2022 (https://adventofcode.com)
    Run: adventofcode2022 problem
        problem is one of 1a, 1b

    The utility reads input from stdin and prints result to stdout.
    "));
}