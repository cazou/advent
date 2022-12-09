use anyhow::Result;
use std::fs::OpenOptions;
use std::io::Read;
use std::time::Instant;
use traits::AdventOfCode;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod traits;

fn get_input(year: u16, day: u8) -> Result<String> {
    let mut input = String::new();
    OpenOptions::new()
        .read(true)
        .open(format!("inputs/{}-{:02}.txt", year, day))?
        .read_to_string(&mut input)?;

    Ok(input)
}

fn main() -> Result<()> {
    let mut exercises: Vec<Box<dyn AdventOfCode>> = vec![
        Box::new(day1::Day1),
        Box::new(day2::Day2),
        Box::new(day3::Day3),
        Box::new(day4::Day4),
        Box::new(day5::Day5),
        Box::new(day6::Day6),
        Box::new(day7::Day7),
        Box::new(day8::Day8),
        Box::new(day9::Day9),
    ];

    for e in exercises.iter_mut() {
        let start = Instant::now();
        let result = e.run1(get_input(2022, e.day()).ok())?;
        println!("[{:?}] Day {}a:", start.elapsed(), e.day());
        println!("{}", result);

        let start = Instant::now();
        let result = e.run2(get_input(2022, e.day()).ok())?;
        println!("[{:?}] Day {}b:", start.elapsed(), e.day());
        println!("{}", result);
        println!();
    }

    Ok(())
}
