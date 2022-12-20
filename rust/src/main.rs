mod problem01;
mod problem02;
mod problem03;
mod problem04;
mod problem05;
mod problem06;
mod problem07;
mod problem08;
mod problem09;
mod problem10;
mod problem11;
mod problem12;
mod problem14;
mod problem16;
mod problem18;
mod problem20;
mod utils;

use std::env::{args};
use indoc::indoc;
use crate::utils::{stdin_lines};

fn main() {
    if let Some(problem) = args().nth(1) {
        if solve(problem.as_str()) {
            return;
        }
    }

    help();
}

fn solve(problem: &str) -> bool {
    match problem {
        "1a" => {
            println!("{}", problem01::solve_a(&mut stdin_lines()));
        },
        "1b" => {
            println!("{}", problem01::solve_b(&mut stdin_lines()));
        },
        "2a" => {
            println!("{}", problem02::solve_a(&mut stdin_lines()));
        },
        "2b" => {
            println!("{}", problem02::solve_b(&mut stdin_lines()));
        },
        "3a" => {
            println!("{}", problem03::solve_a(&mut stdin_lines()));
        },
        "3b" => {
            println!("{}", problem03::solve_b(&mut stdin_lines()));
        },
        "4a" => {
            println!("{}", problem04::solve_a(&mut stdin_lines()));
        },
        "4b" => {
            println!("{}", problem04::solve_b(&mut stdin_lines()));
        },
        "5a" => {
            println!("{}", problem05::solve_a(&mut stdin_lines()));
        },
        "5b" => {
            println!("{}", problem05::solve_b(&mut stdin_lines()));
        },
        "6a" => {
            println!("{}", problem06::solve_a(&mut stdin_lines()));
        },
        "6b" => {
            println!("{}", problem06::solve_b(&mut stdin_lines()));
        },
        "7a" => {
            println!("{}", problem07::solve_a(&mut stdin_lines()));
        },
        "7b" => {
            println!("{}", problem07::solve_b(&mut stdin_lines()));
        },
        "8a" => {
            println!("{}", problem08::solve_a(&mut stdin_lines()));
        },
        "8b" => {
            println!("{}", problem08::solve_b(&mut stdin_lines()));
        },
        "9a" => {
            println!("{}", problem09::solve_a(&mut stdin_lines()));
        },
        "9b" => {
            println!("{}", problem09::solve_b(&mut stdin_lines()));
        },
        "10a" => {
            println!("{}", problem10::solve_a(&mut stdin_lines()));
        },
        "10b" => {
            println!("{}", problem10::solve_b(&mut stdin_lines()));
        },
        "11a" => {
            println!("{}", problem11::solve_a(&mut stdin_lines()));
        },
        "11b" => {
            println!("{}", problem11::solve_b(&mut stdin_lines()));
        },
        "12a" => {
            println!("{}", problem12::solve_a(&mut stdin_lines()));
        },
        "12b" => {
            println!("{}", problem12::solve_b(&mut stdin_lines()));
        },
        "14a" => {
            println!("{}", problem14::solve_a(&mut stdin_lines()));
        },
        "14b" => {
            println!("{}", problem14::solve_b(&mut stdin_lines()));
        },
        "16a" => {
            println!("{}", problem16::solve_a(&mut stdin_lines()));
        },
        "16b" => {
            println!("{}", problem16::solve_b(&mut stdin_lines()));
        },
        "18a" => {
            println!("{}", problem18::solve_a(&mut stdin_lines()));
        },
        "18b" => {
            println!("{}", problem18::solve_b(&mut stdin_lines()));
        },
        "20a" => {
            println!("{}", problem20::solve_a(&mut stdin_lines()));
        },
        "20b" => {
            println!("{}", problem20::solve_b(&mut stdin_lines()));
        },
        _ => {
            return false;
        }
    }

    true
}

fn help() {
    println!(indoc!("
        Advent of Code 2022 (https://adventofcode.com)
        Run: adventofcode2022 problem
            problem is one of 1a-12a,14a,16a; 1b-12b,14b,16b
    
        The utility reads input from stdin and prints result to stdout."));
}