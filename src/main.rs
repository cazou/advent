mod day1;
mod day2;
mod day3;
mod day4;
mod day6;
mod day7;

fn main() -> Result<(), String> {
    match day1::run(include_str!("../inputs/2021-01.txt")) {
        Ok(_) => {},
        Err(e) => return Err(e.to_string())
    }

    match day2::run(include_str!("../inputs/2021-02.txt")) {
        Ok(_) => {},
        Err(e) => return Err(e.to_string())
    }

    match day3::run(include_str!("../inputs/2021-03.txt")) {
        Ok(_) => {},
        Err(e) => return Err(e.to_string())
    }

    match day4::run(include_str!("../inputs/2021-04.txt")) {
        Ok(_) => {},
        Err(e) => return Err(e.to_string())
    }

    match day6::run(include_str!("../inputs/2021-06.txt")) {
        Ok(_) => {},
        Err(e) => return Err(e.to_string())
    }

    match day6::run2(include_str!("../inputs/2021-06.txt")) {
        Ok(_) => {},
        Err(e) => return Err(e.to_string())
    }

    match day7::run(include_str!("../inputs/2021-07.txt")) {
        Ok(_) => {},
        Err(e) => return Err(e.to_string())
    }

    Ok(())
}
