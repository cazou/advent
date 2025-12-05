use crate::traits::AdventOfCode;
use anyhow::Result;
use std::{ops::RangeInclusive, str::FromStr};

pub struct Day5;

struct Inventory {
    fresh: Vec<RangeInclusive<usize>>,
    ingredients: Vec<usize>,
}

impl FromStr for Inventory {
    type Err = ();

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut fresh_pass1 = vec![];
        let mut fresh = vec![];
        let mut ingredients = vec![];

        for l in s.lines() {
            if l.is_empty() {
                continue;
            }
            let mut vals = l.split('-');
            let v1 = vals.next();
            let v2 = vals.next();
            if let Some(end) = v2 {
                let new_range = Self::add_range_cleaned(
                    &fresh_pass1,
                    v1.unwrap().parse().unwrap()..=end.parse().unwrap(),
                );
                if !new_range.is_empty() {
                    fresh_pass1.push(new_range);
                }
            } else if let Some(v) = v1 {
                ingredients.push(v.parse().unwrap());
            }
        }

        while !fresh_pass1.is_empty() {
            let range = fresh_pass1.remove(0);
            let new_range = Self::add_range_cleaned(&fresh_pass1, range.clone());
            if !new_range.is_empty() {
                fresh.push(new_range);
            }
        }

        Ok(Inventory { fresh, ingredients })
    }
}

impl Inventory {
    fn add_range_cleaned(
        fresh: &[RangeInclusive<usize>],
        new: RangeInclusive<usize>,
    ) -> RangeInclusive<usize> {
        if new.is_empty() {
            return new;
        }

        let mut new_start = *new.start();
        let mut new_end = *new.end();

        for other in fresh {
            if other.contains(&new_end) {
                new_end = *other.start() - 1;
            }
            if other.contains(&new_start) {
                new_start = *other.end() + 1;
            }
        }

        new_start..=new_end
    }

    fn find_fresh(&self) -> usize {
        let mut fresh = 0;
        for ingredient in &self.ingredients {
            for range in &self.fresh {
                if range.contains(&ingredient) {
                    fresh += 1;
                    break;
                }
            }
        }

        fresh
    }

    fn all_fresh(&self) -> usize {
        self.fresh
            .iter()
            .fold(0, |count, range| count + (range.end() - range.start() + 1))
    }
}

impl AdventOfCode for Day5 {
    fn day(&self) -> u8 {
        5
    }

    fn run1(&mut self, input: Option<String>) -> Result<String> {
        let inventory: Inventory = input.unwrap().parse().unwrap();
        Ok(inventory.find_fresh().to_string())
    }

    fn run2(&mut self, input: Option<String>) -> Result<String> {
        let inventory: Inventory = input.unwrap().parse().unwrap();
        Ok(inventory.all_fresh().to_string())
    }
}
