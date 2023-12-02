use crate::traits::AdventOfCode;
use anyhow::Result;
use std::str::FromStr;

pub struct Day{{ $day }};

impl AdventOfCode for Day{{ $day }} {
    fn day(&self) -> u8 {
        {{ $day }}
    }

    fn run1(&mut self, input: Option<String>) -> Result<String> {
        todo!()
    }

    fn run2(&mut self, input: Option<String>) -> Result<String> {
        todo!()
    }
}
