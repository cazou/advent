use anyhow::Result;
use regex::Regex;

fn parse_stacks(input: &str, stacks: &mut Vec<Vec<char>>) -> Result<usize> {
    let mut l_count = 0;
    for _ in 0..9 {
        stacks.push(vec![]);
    }

    for line in input.lines() {
        if line.is_empty() {
            return Ok(l_count);
        }

        let re = Regex::new(r"([A-Z])").unwrap();
        let caps = re.find_iter(line);
        for c in caps {
            let stack = (c.start() + 3) / 4;
            stacks[stack - 1].insert(0, c.as_str().chars().nth(0).unwrap());
        }

        l_count += 1;
    }

    Ok(0)
}

pub fn run(input: &str) -> Result<()> {
    let re = Regex::new(r"move (?P<num>\d*) from (?P<from>\d*) to (?P<to>\d*)").unwrap();
    for crane_id in [9000, 9001] {
        let mut stacks: Vec<Vec<char>> = vec![];
        let move_line = parse_stacks(input, &mut stacks)?;
        let mut l_count = 0;

        for line in input.lines() {
            if l_count <= move_line {
                l_count += 1;
                continue;
            }

            let caps = re.captures(line).unwrap();
            let num = caps.name("num").unwrap().as_str().parse::<usize>()?;
            let from = caps.name("from").unwrap().as_str().parse::<usize>()?;
            let to = caps.name("to").unwrap().as_str().parse::<usize>()?;

            let mut tmp = vec![];

            for _ in 0..num {
                tmp.push(stacks[from - 1].pop().unwrap());
            }

            if crane_id == 9001 {
                tmp.reverse();
            }

            for e in tmp {
                stacks[to - 1].push(e);
            }
        }

        let empty = ' ';
        print!("CrateMover {}: ", crane_id);
        for s in &stacks {
            print!("{}", s.last().unwrap_or(&empty));
        }
        println!();
    }

    Ok(())
}
