use anyhow::Result;
use clap::Parser;
use colored::Colorize;
use std::fs::OpenOptions;
use std::io::Read;
use std::time::{Duration, Instant};
use traits::AdventOfCode;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Specify a day to run
    #[arg(short, long, value_parser = clap::value_parser!(u8).range(1..=25))]
    day: Option<u8>,

    /// Specify a part to run, 1 or 2
    #[arg(short, long, value_parser = clap::value_parser!(u8).range(1..=2))]
    part: Option<u8>,

    /// Setup the boiler plate + input for the given new day
    #[arg(long, value_parser = clap::value_parser!(u8).range(1..=25))]
    new_day: Option<u8>,

    /// Use the example input if available in input/YYY-DD-example.txt
    #[arg(long, short)]
    example: bool,
}

mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
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

fn print_result(day: u8, part: u8, elapsed: &Duration, result: &str) {
    println!(" * Day {}-{}", format!("{:02}", day).bold(), part);
    println!(
        "   Runtime: {}",
        format!("{:?}", elapsed).bold().bright_blue()
    );
    if result.lines().count() > 1 {
        println!("   Result:");
        for line in result.lines() {
            println!("     {}", line.bold().bright_green());
        }
    } else {
        println!("   Result: {}", result.bold().bright_green());
    }

    println!();
}

fn main() -> Result<()> {
    let args = Args::parse();

    if let Some(_) = args.new_day {
        // add src/day{d}.rs from template
        // download input and put it in input/2022-{d}.txt
        // modify this file to add the module (TBC)
        //return Ok(());
        todo!()
    }

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
        Box::new(day10::Day10),
        Box::new(day11::Day11),
        Box::new(day12::Day12),
        Box::new(day13::Day13),
    ];

    for e in exercises.iter_mut() {
        if let Some(d) = args.day {
            if d != e.day() {
                continue;
            }
        }

        if args.part.is_none() || args.part.unwrap() == 1 {
            let start = Instant::now();
            let result = e.run1(get_input(2022, e.day()).ok())?;
            print_result(e.day(), 1, &start.elapsed(), &result);
        }

        if args.part.is_none() || args.part.unwrap() == 2 {
            let start = Instant::now();
            let result = e.run2(get_input(2022, e.day()).ok())?;
            print_result(e.day(), 2, &start.elapsed(), &result);
        }
    }

    Ok(())
}
