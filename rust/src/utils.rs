use std::io::{BufRead, stdin};

pub fn stdin_lines() -> impl Iterator<Item = String> {
    stdin().lock().lines().map(|line| line.unwrap())
}

#[cfg(test)]
pub fn strvec_to_iter<'a>(string_vec: &'a Vec<&'a str>) -> impl Iterator<Item = String> + 'a {
    string_vec.iter().map(|s| s.to_string())
}