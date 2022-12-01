use anyhow::Result;

mod day1;

fn main() -> Result<()> {
    day1::run(include_str!("../inputs/2022-01.txt"))?;

    Ok(())
}
