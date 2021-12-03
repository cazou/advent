use std::fs;

enum Operation {
    Forward(usize),
    Up(usize),
    Down(usize),
}

impl Operation {
    pub fn from_line(line: &str) -> Result<Operation, String> {
        let splits: Vec<&str> = line.split(' ').collect();

        let value: usize = match splits[1].parse() {
            Ok(v) => v,
            Err(e) => return Err(e.to_string())
        };

        match splits[0] {
            "forward" => Ok(Operation::Forward(value)),
            "up" => Ok(Operation::Up(value)),
            "down" => Ok(Operation::Down(value)),
            _ => Err(String::from("Cannot parse ") + splits[0]),
        }

    }
}

pub fn run(inputfile: &str) -> Result<(), String> {
    print!("[Dive]...        ");

    let contents = match fs::read_to_string(inputfile) {
        Ok(c) => c,
        Err(e) => {
            return Err(e.to_string());
        }
    };

    let mut forward = 0;
    let mut depth = 0;
    let mut aim = 0;

    for line in contents.split('\n') {
        if line.is_empty() {
            continue;
        }

        let op = match Operation::from_line(line) {
            Ok(o) => o,
            Err(e) => return Err(e),
        };

        match op {
            Operation::Forward(f) => {
                forward += f;
                depth += f * aim;
            },
            Operation::Up(v) => aim -= v,
            Operation::Down(v) => aim += v,
        }
    }

    println!("{}", forward * depth);

    Ok(())
}
