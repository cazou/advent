use anyhow::Result;

mod day1;
mod day2;
mod day3;

fn main() -> Result<()> {
    day1::run(include_str!("../inputs/2022-01.txt"))?;
    day2::run(include_str!("../inputs/2022-02.txt"))?;
    day3::run(include_str!("../inputs/2022-03.txt"))?;

    Ok(())
}
