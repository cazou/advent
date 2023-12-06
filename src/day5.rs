use crate::traits::AdventOfCode;
use anyhow::Result;
use std::cmp::{max, min};
use std::collections::HashMap;
use std::ops::Range;
use std::str::FromStr;

const MAP_ORDER: [&str; 7] = [
    "seed-to-soil",
    "soil-to-fertilizer",
    "fertilizer-to-water",
    "water-to-light",
    "light-to-temperature",
    "temperature-to-humidity",
    "humidity-to-location",
];

#[derive(Clone, Debug)]
struct Mapping {
    source: Range<usize>,
    dst: Range<usize>,
}

impl Mapping {
    pub fn map_range(&self, range: &Range<usize>) -> (Option<Range<usize>>, Vec<Range<usize>>) {
        if range.start > self.source.end || range.end < self.source.start {
            return (None, vec![range.clone()]);
        }

        let delta = self.dst.start as isize - self.source.start as isize;
        let mut rest = vec![];
        if range.start < self.source.start {
            rest.push(range.start..self.source.start);
        }
        if range.end > self.source.end {
            rest.push(self.source.end..range.end);
        }

        (
            Some(
                (max(range.start, self.source.start) as isize + delta) as usize
                    ..(min(range.end, self.source.end) as isize + delta) as usize,
            ),
            rest,
        )
    }
}

impl FromStr for Mapping {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let vals: Vec<usize> = s.split(" ").map(|v| v.parse().unwrap()).collect();
        Ok(Mapping {
            source: vals[1]..vals[1] + vals[2],
            dst: vals[0]..vals[0] + vals[2],
        })
    }
}

#[derive(Clone, Debug)]
struct Map {
    mappings: Vec<Mapping>,
}

impl Map {
    pub fn map(&self, val: usize) -> usize {
        for mapping in &self.mappings {
            if mapping.source.contains(&val) {
                return mapping.dst.start + (val - mapping.source.start);
            }
        }

        val
    }

    pub fn map_range(&self, range: &Range<usize>) -> Vec<Range<usize>> {
        let mut to_map = vec![range.clone()];
        let mut ranges = vec![];
        for mapping in &self.mappings {
            let to_map_current = to_map.clone();
            to_map.clear();
            for r in &to_map_current {
                let (mapped, mut rest) = mapping.map_range(r);

                if let Some(r) = mapped {
                    ranges.push(r);
                }
                to_map.append(&mut rest);
            }
        }

        ranges.append(&mut to_map);

        ranges
    }
}

impl FromStr for Map {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut mappings = vec![];
        for line in s.lines() {
            mappings.push(line.parse().unwrap());
        }

        Ok(Map { mappings })
    }
}

#[derive(Debug)]
struct Almanac {
    seeds: Vec<usize>,
    maps: HashMap<String, Map>,
}

impl Almanac {
    fn seed_to_location(&self, seed: usize) -> usize {
        let mut current_val = seed;
        for map in MAP_ORDER {
            current_val = self.maps[map].map(current_val);
        }

        current_val
    }

    fn seeds_to_location(&self, start: usize, to: usize) -> Vec<Range<usize>> {
        let mut ranges = vec![start..to];
        for map in MAP_ORDER {
            let map = &self.maps[map];
            let mut new_ranges = vec![];
            for range in &ranges {
                let mut mapped = map.map_range(range);
                new_ranges.append(&mut mapped);
            }
            ranges = new_ranges;
        }

        ranges
    }

    pub fn locations(&self) -> usize {
        let mut min = usize::MAX;
        for seed in &self.seeds {
            let val = self.seed_to_location(*seed);
            if val < min {
                min = val
            }
        }

        min
    }

    pub fn locations2(&self) -> usize {
        let mut min = usize::MAX;
        let mut start: Option<usize> = None;

        for val in &self.seeds {
            if let Some(s) = start {
                let v = self
                    .seeds_to_location(s, s + *val)
                    .iter()
                    .map(|v| v.start)
                    .min()
                    .unwrap_or(0);
                if v < min {
                    min = v;
                }
                start = None;
            } else {
                start = Some(*val);
            }
        }

        min
    }
}

impl FromStr for Almanac {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut maps: HashMap<String, Map> = HashMap::new();
        let mut current_name: Option<String> = None;
        let mut current_map = Map { mappings: vec![] };
        let mut seeds = vec![];

        for line in s.lines() {
            if line.starts_with("seeds: ") {
                seeds = line
                    .split_once(" ")
                    .unwrap()
                    .1
                    .split(" ")
                    .map(|s| s.parse().unwrap())
                    .collect();
                continue;
            } else if line.is_empty() && current_name.is_some() {
                maps.insert(current_name.unwrap(), current_map.clone());

                current_name = None;
                current_map.mappings.clear();

                continue;
            } else if line.is_empty() {
                continue;
            } else if MAP_ORDER.contains(&line.split_once(" ").unwrap().0) {
                current_name = Some(line.split_once(" ").unwrap().0.to_string());

                continue;
            }

            current_map.mappings.push(line.parse()?);
        }

        Ok(Almanac { maps, seeds })
    }
}

pub struct Day5;

impl AdventOfCode for Day5 {
    fn day(&self) -> u8 {
        5
    }

    fn run1(&mut self, input: Option<String>) -> Result<String> {
        let almanac: Almanac = input.unwrap().parse()?;
        Ok(almanac.locations().to_string())
    }

    fn run2(&mut self, input: Option<String>) -> Result<String> {
        let almanac: Almanac = input.unwrap().parse()?;
        Ok(almanac.locations2().to_string())
    }
}
