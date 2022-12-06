use anyhow::Result;
use regex::Regex;

pub fn run(input: &str) -> Result<()> {
    let mut contained = 0;
    let mut overlap = 0;
    let re = Regex::new(r"(?P<from1>\d*)-(?P<to1>\d*),(?P<from2>\d*)-(?P<to2>\d*)").unwrap();

    for line in input.lines() {
        let caps = re.captures(line).unwrap();
        let r0 = caps.name("from1").unwrap().as_str().parse::<usize>()?
            ..=caps.name("to1").unwrap().as_str().parse::<usize>()?;
        let r1 = caps.name("from2").unwrap().as_str().parse::<usize>()?
            ..=caps.name("to2").unwrap().as_str().parse::<usize>()?;

        if r1.contains(&r0.start()) && r1.contains(&r0.end())
            || r0.contains(&r1.start()) && r0.contains(&r1.end())
        {
            contained += 1;
        }

        if r0.start() <= r1.end() && r0.end() >= r1.start() {
            overlap += 1;
        }
    }

    println!("Contained = {}", contained);
    println!("Overlap = {}", overlap);

    Ok(())
}
