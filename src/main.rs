mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;

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

    match day5::run(include_str!("../inputs/2021-05.txt")) {
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

    match day8::run(include_str!("../inputs/2021-08.txt")) {
        Ok(_) => {},
        Err(e) => return Err(e.to_string())
    }

    match day9::run(include_str!("../inputs/2021-09.txt")) {
        Ok(_) => {},
        Err(e) => return Err(e.to_string())
    }

    match day10::run(include_str!("../inputs/2021-10.txt")) {
        Ok(_) => {},
        Err(e) => return Err(e.to_string())
    }

    match day11::run(include_str!("../inputs/2021-11.txt")) {
        Ok(_) => {},
        Err(e) => return Err(e.to_string())
    }

    match day12::run(include_str!("../inputs/2021-12.txt")) {
        Ok(_) => {},
        Err(e) => return Err(e.to_string())
    }

    match day13::run(include_str!("../inputs/2021-13.txt")) {
        Ok(_) => {},
        Err(e) => return Err(e.to_string())
    }

    match day14::run(include_str!("../inputs/2021-14.txt")) {
        Ok(_) => {},
        Err(e) => return Err(e.to_string())
    }

    match day15::run(include_str!("../inputs/2021-15.txt")) {
        Ok(_) => {},
        Err(e) => return Err(e.to_string())
    }

    match day16::run(include_str!("../inputs/2021-16.txt")) {
        Ok(_) => {},
        Err(e) => return Err(e.to_string())
    }

    match day17::run() {
        Ok(_) => {},
        Err(e) => return Err(e.to_string())
    }

    match day18::run(include_str!("../inputs/2021-18.txt")) {
        Ok(_) => {},
        Err(e) => return Err(e.to_string())
    }

    match day19::run(include_str!("../inputs/2021-19.txt")) {
        Ok(_) => {},
        Err(e) => return Err(e.to_string())
    }

    Ok(())
}
