use crate::traits::AdventOfCode;
use anyhow::Result;
use regex::Regex;

pub struct Day4;

impl AdventOfCode for Day4 {
    fn day(&self) -> u8 {
        4
    }

    fn run1(&mut self, input: Option<String>) -> Result<String> {
        let mut contained = 0;
        let re = Regex::new(r"(?P<from1>\d*)-(?P<to1>\d*),(?P<from2>\d*)-(?P<to2>\d*)").unwrap();

        for line in input.unwrap().lines() {
            let caps = re.captures(line).unwrap();
            let r0 = caps.name("from1").unwrap().as_str().parse::<usize>()?
                ..=caps.name("to1").unwrap().as_str().parse::<usize>()?;
            let r1 = caps.name("from2").unwrap().as_str().parse::<usize>()?
                ..=caps.name("to2").unwrap().as_str().parse::<usize>()?;

            if r1.contains(r0.start()) && r1.contains(r0.end())
                || r0.contains(r1.start()) && r0.contains(r1.end())
            {
                contained += 1;
            }
        }

        Ok(contained.to_string())
    }

    fn run2(&mut self, input: Option<String>) -> Result<String> {
        let mut overlap = 0;
        let re = Regex::new(r"(?P<from1>\d*)-(?P<to1>\d*),(?P<from2>\d*)-(?P<to2>\d*)").unwrap();

        for line in input.unwrap().lines() {
            let caps = re.captures(line).unwrap();
            let r0 = caps.name("from1").unwrap().as_str().parse::<usize>()?
                ..=caps.name("to1").unwrap().as_str().parse::<usize>()?;
            let r1 = caps.name("from2").unwrap().as_str().parse::<usize>()?
                ..=caps.name("to2").unwrap().as_str().parse::<usize>()?;

            if r0.start() <= r1.end() && r0.end() >= r1.start() {
                overlap += 1;
            }
        }

        Ok(overlap.to_string())
    }
}
