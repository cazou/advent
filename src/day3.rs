use crate::traits::AdventOfCode;
use anyhow::Result;
use std::str::FromStr;

struct Rucksack {
    compartments: String,
}

impl FromStr for Rucksack {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let compartments = s.to_string();

        Ok(Rucksack { compartments })
    }
}

impl Rucksack {
    fn sticker(r1: &Rucksack, r2: &Rucksack, r3: &Rucksack) -> u8 {
        let s1 = &r1.compartments;
        let s2 = &r2.compartments;
        let s3 = &r3.compartments;

        for c1 in s1.as_bytes() {
            for c2 in s2.as_bytes() {
                for c3 in s3.as_bytes() {
                    if c1 == c2 && c2 == c3 {
                        return Self::priority(*c1);
                    }
                }
            }
        }

        0
    }

    fn priority(item: u8) -> u8 {
        if (item as char).is_lowercase() {
            item - 'a' as u8 + 1
        } else if (item as char).is_uppercase() {
            item - 'A' as u8 + 27
        } else {
            0
        }
    }

    fn common(&self) -> u8 {
        let c = self.compartments.split_at(self.compartments.len() / 2);
        for l in c.0.as_bytes() {
            for m in c.1.as_bytes() {
                if l == m {
                    return Self::priority(*l);
                }
            }
        }

        0
    }
}

pub struct Day3;

impl AdventOfCode for Day3 {
    fn day(&self) -> u8 {
        3
    }

    fn run1(&mut self, input: Option<String>) -> Result<String> {
        let mut priorities = 0;
        for line in input.unwrap().lines() {
            let rucksack: Rucksack = line.parse()?;
            priorities += rucksack.common() as usize;
        }

        Ok(priorities.to_string())
    }

    fn run2(&mut self, input: Option<String>) -> Result<String> {
        let mut stickers = 0;
        let mut group = vec![];
        for line in input.unwrap().lines() {
            let rucksack: Rucksack = line.parse()?;
            group.push(rucksack);
            if group.len() == 3 {
                stickers += Rucksack::sticker(&group[0], &group[1], &group[2]) as usize;
                group.clear();
            }
        }

        Ok(stickers.to_string())
    }
}
