use crate::traits::AdventOfCode;
use anyhow::{bail, Result};
use std::str::FromStr;

#[derive(Debug)]
enum ItemType {
    Symbol(String),
    Number(usize),
}

impl ItemType {
    pub fn is_symbol(&self) -> bool {
        match self {
            ItemType::Symbol(_) => true,
            _ => false,
        }
    }

    pub fn is_gear_symbol(&self) -> bool {
        match self {
            ItemType::Symbol(s) => s == "*",
            _ => false,
        }
    }

    pub fn value(&self) -> Option<usize> {
        match self {
            ItemType::Number(d) => Some(*d),
            _ => None,
        }
    }
}

impl FromStr for ItemType {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.parse::<usize>() {
            Ok(d) => Ok(ItemType::Number(d)),
            Err(_) => Ok(ItemType::Symbol(s.to_string())),
        }
    }
}

#[derive(Debug)]
struct Item {
    x: usize,
    y: usize,
    len: usize,
    item_type: ItemType,
}

impl Item {
    pub fn near(&self, other: &Item) -> bool {
        let x_from = if self.x == 0 { self.x } else { self.x - 1 };
        let y_from = if self.y == 0 { self.y } else { self.y - 1 };
        for x in x_from..=self.x + self.len {
            for y in y_from..=self.y + 1 {
                if other.x == x && other.y == y {
                    return true;
                }
            }
        }
        false
    }
}

pub struct Day3;

impl Day3 {
    fn find_items(line: &str, idx: usize) -> Vec<Item> {
        let mut items = vec![];
        let regex = regex::Regex::new(r"\.*(:?(?<num>[0-9]+|[^.0-9])\.*)?").unwrap();
        for c in regex.captures_iter(line) {
            if let Some(m) = c.name("num") {
                let item_type = m.as_str().parse().unwrap();
                items.push(Item {
                    x: m.start(),
                    y: idx,
                    len: m.len(),
                    item_type,
                })
            }
        }
        items
    }

    fn find_adjacent(items: Vec<Item>) -> usize {
        let mut sum = 0;
        for item in items.iter().filter(|i| !i.item_type.is_symbol()) {
            let val = if let ItemType::Number(v) = item.item_type {
                v
            } else {
                0
            };

            if items
                .iter()
                .filter(|i| i.item_type.is_symbol() && item.near(&i))
                .next()
                .is_some()
            {
                sum += val
            }
        }

        sum
    }

    fn find_gears(items: Vec<Item>) -> usize {
        let mut sum = 0;
        for gear_item in items.iter().filter(|i| i.item_type.is_gear_symbol()) {
            let gears = items
                .iter()
                .filter(|i| !i.item_type.is_symbol() && i.near(&gear_item))
                .collect::<Vec<&Item>>();

            if gears.len() == 2 {
                sum += gears[0].item_type.value().unwrap() * gears[1].item_type.value().unwrap()
            }
        }

        sum
    }
}

impl AdventOfCode for Day3 {
    fn day(&self) -> u8 {
        3
    }

    fn run1(&mut self, input: Option<String>) -> Result<String> {
        let input = input.unwrap();
        let mut items = vec![];
        for (idx, line) in input.lines().enumerate() {
            items.append(&mut Self::find_items(line, idx));
        }

        Ok(Self::find_adjacent(items).to_string())
    }

    fn run2(&mut self, input: Option<String>) -> Result<String> {
        let input = input.unwrap();
        let mut items = vec![];
        for (idx, line) in input.lines().enumerate() {
            items.append(&mut Self::find_items(line, idx));
        }

        Ok(Self::find_gears(items).to_string())
    }
}
