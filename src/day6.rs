use crate::traits::AdventOfCode;
use anyhow::Result;

const RACES: [(usize, usize); 4] = [(48, 296), (93, 1928), (85, 1236), (95, 1391)];
const RACE: (usize, usize) = (48938595, 296192812361391);

fn valid_push_times_count(time: usize, dist: usize) -> usize {
    let mut records = 0;
    for push_time in 0..time {
        let speed = push_time;
        let left_time = time - push_time;
        if left_time * speed > dist {
            records += 1;
        }
    }
    records
}

pub struct Day6;

impl AdventOfCode for Day6 {
    fn day(&self) -> u8 {
        6
    }

    fn run1(&mut self, _input: Option<String>) -> Result<String> {
        Ok(RACES
            .iter()
            .fold(1, |record, (time, dist)| {
                record * valid_push_times_count(*time, *dist)
            })
            .to_string())
    }

    fn run2(&mut self, _input: Option<String>) -> Result<String> {
        Ok(valid_push_times_count(RACE.0, RACE.1).to_string())
    }
}
