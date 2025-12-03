use crate::traits::AdventOfCode;
use anyhow::Result;
use std::str::FromStr;

pub struct Day3;

impl Day3 {
    fn run(input: &str, batteries: u8) -> usize {
        let mut full_joltage = 0;

        for line in input.lines() {
            let mut maxes = vec![];

            // Find the n batteries ordered maxes
            for i in 0..batteries {
                maxes.push((0, '0'));
                for (position, joltage) in line.chars().enumerate() {
                    if i > 0 && position <= maxes[(i - 1) as usize].0 {
                        continue;
                    }
                    if position == line.len() - (batteries - i - 1) as usize {
                        break;
                    }

                    let (pos, max) = &mut maxes[i as usize];
                    if *max < joltage {
                        *max = joltage;
                        *pos = position;
                    }
                }
            }

            full_joltage += String::from_iter(maxes.iter().map(|a| a.1))
                .parse::<usize>()
                .unwrap();
        }

        full_joltage
    }
}

impl AdventOfCode for Day3 {
    fn day(&self) -> u8 {
        3
    }

    fn run1(&mut self, input: Option<String>) -> Result<String> {
        let input = input.unwrap();
        Ok(Self::run(&input, 2).to_string())
    }

    fn run2(&mut self, input: Option<String>) -> Result<String> {
        let input = input.unwrap();
        Ok(Self::run(&input, 12).to_string())
    }
}
