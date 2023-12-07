use anyhow::{bail, Result};
use clap::Parser;
use colored::Colorize;
use reqwest;
use reqwest::header::{HeaderMap, HeaderValue};
use std::fs::{read_to_string, File, OpenOptions};
use std::io::{Read, Write};
use std::path::PathBuf;
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

    /// Cookie for the adventofcode.com input
    #[arg(long, short)]
    cookie: Option<String>,

    /// Use the example input if available in input/YYY-DD-example.txt
    #[arg(long, short)]
    example: bool,
}

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod traits;

fn get_input(year: u16, day: u8, example: bool) -> Result<String> {
    let mut input = String::new();
    OpenOptions::new()
        .read(true)
        .open(format!(
            "inputs/{}-{:02}{}.txt",
            year,
            day,
            if example { "-example" } else { "" }
        ))?
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

    if let Some(day) = args.new_day {
        // Create Source file from template
        let dst = PathBuf::from(format!("src/day{}.rs", day));
        if dst.exists() {
            bail!("{dst:?} already exists");
        }

        let source = read_to_string(PathBuf::from("templates/dayN.tpl"))?;

        let mut dst_file = File::create(dst)?;
        dst_file.write_all(
            source
                .replace("{{ $day }}", day.to_string().as_str())
                .as_bytes(),
        )?;

        // Import module in main.rs and add in list
        let main_src = read_to_string("src/main.rs")?;
        let new_src = main_src
            .replace(
                format!("Box::new(day{}::Day{})", day - 1, day - 1).as_str(),
                format!(
                    "Box::new(day{}::Day{}), Box::new(day{}::Day{})",
                    day - 1,
                    day - 1,
                    day,
                    day
                )
                .as_str(),
            )
            .replace(
                format!("mod day{};", day - 1).as_str(),
                format!("mod day{};\nmod day{};", day - 1, day).as_str(),
            );

        // This will truncate the file
        let mut main_file = File::create("src/main.rs")?;
        main_file.write_all(new_src.as_bytes())?;

        // Download exercise input
        if let Some(cookie) = args.cookie {
            let url = format!("https://adventofcode.com/2023/day/{}/input", day);
            let client = reqwest::blocking::Client::new();
            let mut headers = HeaderMap::new();
            headers.insert("Cookie", HeaderValue::from_str(cookie.as_str()).unwrap());

            let res = client.get(url).headers(headers).send()?;
            if !res.status().is_success() {
                bail!(
                    "Could not retrieve the input: {} ({})",
                    res.status(),
                    res.text()?
                )
            }
            let mut input_file = File::create(format!("inputs/2023-{day:02}.txt"))?;

            input_file.write_all(res.bytes()?.as_ref())?;
        }

        return Ok(());
    }

    let mut exercises: Vec<Box<dyn AdventOfCode>> = vec![
        Box::new(day1::Day1),
        Box::new(day2::Day2),
        Box::new(day3::Day3), Box::new(day4::Day4), Box::new(day5::Day5), Box::new(day6::Day6), Box::new(day7::Day7),
    ];

    for e in exercises.iter_mut() {
        if let Some(d) = args.day {
            if d != e.day() {
                continue;
            }
        }

        if args.part.is_none() || args.part.unwrap() == 1 {
            let start = Instant::now();
            let result = e.run1(get_input(2023, e.day(), args.example).ok())?;
            print_result(e.day(), 1, &start.elapsed(), &result);
        }

        if args.part.is_none() || args.part.unwrap() == 2 {
            let start = Instant::now();
            let result = e.run2(get_input(2023, e.day(), args.example).ok())?;
            print_result(e.day(), 2, &start.elapsed(), &result);
        }
    }

    Ok(())
}
