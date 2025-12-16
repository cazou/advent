use crate::traits::AdventOfCode;
use anyhow::Result;
use regex::Regex;

pub struct Day12;

impl AdventOfCode for Day12 {
    fn day(&self) -> u8 {
        12
    }

    fn run1(&mut self, input: Option<String>) -> Result<String> {
        let re = Regex::new(r"^(?<width>\d*)x(?<height>\d*): (?<quantites>.*)$").unwrap();
        let mut count = 0;

        for line in input.unwrap().lines() {
            let caps = match re.captures(line) {
                Some(c) => c,
                None => continue,
            };
            let width: usize = caps["width"].parse().unwrap();
            let height: usize = caps["height"].parse().unwrap();
            let quantity = caps["quantites"]
                .split(' ')
                .fold(0, |q, a| q + a.parse::<usize>().unwrap());

            if width * height >= quantity * 9 {
                count += 1;
            }
        }

        Ok(count.to_string())
    }

    fn run2(&mut self, _input: Option<String>) -> Result<String> {
        Ok("free".to_string())
    }
}
