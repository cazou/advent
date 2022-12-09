use anyhow::Result;

pub trait AdventOfCode {
    fn day(&self) -> u8;
    fn run1(&mut self, input: Option<String>) -> Result<String>;
    fn run2(&mut self, input: Option<String>) -> Result<String>;
}
