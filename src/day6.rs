use crate::traits::AdventOfCode;
use anyhow::{Context, Result};
use std::collections::HashSet;

fn find_marker(input: &str, len: usize) -> Result<usize> {
    input
        .as_bytes()
        .windows(len)
        .position(|w| w.iter().collect::<HashSet<_>>().len() == len)
        .map(|p| p + len)
        .context(format!("Cannot find marker in '{}'", input))
}

pub struct Day6;

impl AdventOfCode for Day6 {
    fn day(&self) -> u8 {
        6
    }

    fn run1(&mut self, input: Option<String>) -> Result<String> {
        match find_marker(&input.unwrap(), 4) {
            Ok(t) => Ok(t.to_string()),
            Err(e) => Err(e),
        }
    }

    fn run2(&mut self, input: Option<String>) -> Result<String> {
        match find_marker(&input.unwrap(), 14) {
            Ok(t) => Ok(t.to_string()),
            Err(e) => Err(e),
        }
    }
}
