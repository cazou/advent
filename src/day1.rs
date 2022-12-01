use std::cmp::Ordering;
use std::str::FromStr;
use anyhow::{bail, ensure, Result};
use std::collections::binary_heap::BinaryHeap;

#[derive(Eq, PartialEq)]
struct Elf {
    calories: usize
}

impl Elf {
    pub fn new() -> Elf {
        Elf {
            calories:0
        }
    }

    pub fn add(&mut self, calories: usize) {
        self.calories = self.calories + calories;
    }
}

impl Ord for Elf {
    fn cmp(&self, other: &Self) -> Ordering {
        other.calories.cmp(&self.calories)
    }
}

impl PartialOrd for Elf {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(other.cmp(self))
    }
}

struct Inventory {
    elves: BinaryHeap<Elf>
}

impl Inventory {
    pub fn max(&self) -> Result<usize> {
        match self.elves.peek() {
            Some(e) => Ok(e.calories),
            None => bail!("Inventory is empty")
        }
    }

    pub fn max3(&self) -> Result<usize> {
        let mut count = 0;
        let mut cal = 0;

        for elf in &self.elves {
            if count == 3 {
                break;
            }
            count += 1;
            cal = cal + elf.calories;
        }

        ensure!(count == 3, "Not enough Elves");

        Ok(cal)
    }
}

impl FromStr for Inventory {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut elves = BinaryHeap::new();
        let mut current_elf = Elf::new();
        for line in s.split('\n') {
            if line.is_empty() {
                elves.push(current_elf);
                current_elf = Elf::new();
            } else {
                current_elf.add(line.parse().unwrap())
            }
        }

        if current_elf.calories != 0 {
            elves.push(current_elf);
        }

        Ok(Inventory {
            elves
        })
    }
}

pub fn run(input: &str) -> Result<()> {
    let inventory: Inventory = input.parse().unwrap();

    println!("Maximum: {}", inventory.max()?);
    println!("Maximum of 3: {}", inventory.max3()?);

    Ok(())
}