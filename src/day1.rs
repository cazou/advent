use crate::traits::AdventOfCode;
use anyhow::Result;
use std::str::FromStr;

struct IntValue {
    value: usize,
}

impl FromStr for IntValue {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "one" => Ok(IntValue { value: 1 }),
            "two" => Ok(IntValue { value: 2 }),
            "three" => Ok(IntValue { value: 3 }),
            "four" => Ok(IntValue { value: 4 }),
            "five" => Ok(IntValue { value: 5 }),
            "six" => Ok(IntValue { value: 6 }),
            "seven" => Ok(IntValue { value: 7 }),
            "eight" => Ok(IntValue { value: 8 }),
            "nine" => Ok(IntValue { value: 9 }),
            _ => {
                let value = s.parse()?;
                Ok(IntValue { value })
            }
        }
    }
}

struct Calibration {
    values: Vec<usize>,
}

impl Calibration {
    pub fn compute(&self) -> usize {
        self.values.iter().sum()
    }

    fn from_input(s: &str, include_ascii_values: bool) -> Result<Calibration> {
        let mut values = vec![];
        for line in s.lines() {
            if line.is_empty() {
                continue;
            }

            let nums = Self::extract_numbers(line, include_ascii_values);

            values.push(nums.first().unwrap().1 * 10 + nums.last().unwrap().1);
        }

        Ok(Calibration { values })
    }

    fn extract_numbers(line: &str, include_ascii_values: bool) -> Vec<(usize, usize)> {
        let mut digits = vec!["1", "2", "3", "4", "5", "6", "7", "8", "9"];

        let mut asciis = vec![
            "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
        ];

        if include_ascii_values {
            digits.append(&mut asciis)
        }

        let mut matches: Vec<(usize, &str)> =
            digits.iter().flat_map(|e| line.match_indices(e)).collect();
        matches.sort_by(|(a, _), (b, _)| a.cmp(b));

        matches
            .iter()
            .map(|(i1, s1)| (*i1, s1.parse::<IntValue>().unwrap().value))
            .collect()
    }
}

pub struct Day1;

impl AdventOfCode for Day1 {
    fn day(&self) -> u8 {
        1
    }

    fn run1(&mut self, input: Option<String>) -> Result<String> {
        let c = Calibration::from_input(input.unwrap().as_str(), false)?;
        Ok(c.compute().to_string())
    }

    fn run2(&mut self, input: Option<String>) -> Result<String> {
        let c = Calibration::from_input(input.unwrap().as_str(), true)?;
        Ok(c.compute().to_string())
    }
}
