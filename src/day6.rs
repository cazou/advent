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

pub fn run(input: &str) -> Result<()> {
    println!("Packet start: {}", find_marker(input, 4)?);
    println!("Message start: {}", find_marker(input, 14)?);

    Ok(())
}
