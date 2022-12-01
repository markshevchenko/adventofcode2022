mod problem01;
mod utils;

use crate::utils::{stdin_lines};
use std::env::{args};

fn main() {
    let problem_number = args().nth(1).and_then(|arg| arg.parse::<u32>().ok());

    match problem_number {
        Some(1) => {
            println!("{}", problem01::solve(&mut stdin_lines()));
        },
        _ => {
            println!("Advent of Code 2022 (https://adventofcode.com)");
            println!("Run: adventofcode2022 problem-number");
            println!("     problem-number is a number from 1 to 50");
            println!();
            println!("The utility reads input from stdin and prints result to stdout.");
        }
    }
}
