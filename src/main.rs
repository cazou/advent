use anyhow::Result;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;

fn main() -> Result<()> {
    day1::run(include_str!("../inputs/2022-01.txt"))?;
    day2::run(include_str!("../inputs/2022-02.txt"))?;
    day3::run(include_str!("../inputs/2022-03.txt"))?;
    day4::run(include_str!("../inputs/2022-04.txt"))?;
    day5::run(include_str!("../inputs/2022-05.txt"))?;
    day6::run(include_str!("../inputs/2022-06.txt"))?;
    day7::run(include_str!("../inputs/2022-07.txt"))?;
    day8::run(include_str!("../inputs/2022-08.txt"))?;

    Ok(())
}
