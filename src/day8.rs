use std::str::FromStr;

struct Observation {
    patterns: Vec<String>,
    outputs: Vec<String>,
}

impl Observation {
    pub fn base_numbers(&self) -> usize {
        let mut count = 0;
        for v in &self.outputs {
            count += match v.len() {
                2 | 3 | 4 | 7 => 1,
                _ => 0,
            }
        }

        count
    }
}

impl FromStr for Observation {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let obs: Vec<&str> = s.split('|').collect();
        let patterns: Vec<String> = obs[0].split(' ').map(|x| x.to_string()).collect();
        let outputs: Vec<String> = obs[1].split(' ').map(|x| x.to_string()).collect();

        Ok(Observation {
            patterns,
            outputs

        })
    }
}

pub fn run(contents: &str) -> Result<(), String> {
    print!("[Display]...     ");

    let mut count = 0;

    // Load input data
    for val in contents.lines() {
        if val.is_empty() {
            continue;
        }

        count += Observation::from_str(val).unwrap().base_numbers();
    }

    println!("{}, ?", count);
    Ok(())
}