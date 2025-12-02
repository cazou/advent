use crate::traits::AdventOfCode;
use anyhow::Result;
use std::ops::RangeInclusive;
use std::str::FromStr;

pub struct Day2;

struct Database {
    ranges: Vec<RangeInclusive<usize>>,
}

impl FromStr for Database {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let input = s.trim_end();
        let ranges: Vec<RangeInclusive<usize>> = input
            .split(',')
            .map(|r| {
                let vals: Vec<&str> = r.split('-').collect();
                let a: usize = vals[0].parse().unwrap();
                let b: usize = vals[1].parse().unwrap();
                a..=b
            })
            .collect();

        Ok(Database { ranges })
    }
}

impl Database {
    fn dividers(val: u32) -> Vec<u32> {
        let mut dividers = vec![];
        for d in 2..=val {
            if val % d == 0 {
                dividers.push(d);
            }
        }

        dividers
    }

    fn is_invalid(id: usize) -> bool {
        let len = id.to_string().len() as u32;
        let divs = Self::dividers(len);

        for div in divs {
            let mut invalid_div = true;
            let divider = 10usize.pow(len / div);
            let mut id_parts = id;
            let mut part = None;
            for _ in 0..div {
                let new_part = id_parts % divider;
                if let Some(p) = part {
                    if new_part != p {
                        invalid_div = false;
                        break;
                    }
                } else {
                    part = Some(new_part);
                }
                id_parts /= divider;
            }

            if invalid_div {
                return true;
            }
        }

        return false;
    }
}

impl AdventOfCode for Day2 {
    fn day(&self) -> u8 {
        2
    }

    fn run1(&mut self, input: Option<String>) -> Result<String> {
        let db: Database = input.unwrap().parse().unwrap();
        let mut id_sum = 0;

        for range in db.ranges {
            for val in range {
                let len = val.to_string().len() as u32;
                if len & 1 == 1 {
                    continue;
                }

                let divider = 10usize.pow(len / 2);
                let p1 = val / divider;
                let p2 = val % divider;
                if p1 == p2 {
                    id_sum += val;
                }
            }
        }

        Ok(id_sum.to_string())
    }

    fn run2(&mut self, input: Option<String>) -> Result<String> {
        let db: Database = input.unwrap().parse().unwrap();
        let mut id_sum = 0;

        for range in db.ranges {
            for val in range {
                if Database::is_invalid(val) {
                    id_sum += val;
                }
            }
        }

        Ok(id_sum.to_string())
    }
}
