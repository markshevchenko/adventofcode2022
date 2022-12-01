use std::io::{BufRead, stdin};

pub fn stdin_lines() -> impl Iterator<Item = String> {
    stdin().lock().lines().map(|line| line.unwrap())
}

#[cfg(test)]
pub fn str_to_iter<'a>(s: &'a str) -> impl Iterator<Item = String> + 'a {
    s.split('\n').map(|s| s.to_string())
}