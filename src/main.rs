mod day1;
mod day2;
mod day3;

fn main() -> Result<(), String> {
    match day1::run("/home/detlev/Sources/advent/2021-01.txt") {
        Ok(_) => {},
        Err(e) => return Err(e.to_string())
    }

    match day2::run("/home/detlev/Sources/advent/2021-02.txt") {
        Ok(_) => {},
        Err(e) => return Err(e.to_string())
    }

    match day3::run("/home/detlev/Sources/advent/2021-03.txt") {
        Ok(_) => {},
        Err(e) => return Err(e.to_string())
    }

    Ok(())
}
