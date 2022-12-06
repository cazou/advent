use anyhow::Result;
use std::str::FromStr;

struct Marker {
    message: String,
}

impl Marker {
    fn find_marker(&self, len: usize) -> usize {
        let mut start = 0;
        let mut end = len;
        let mut not_it = true;
        while not_it {
            not_it = false;
            for i in start..end {
                let l = self.message.chars().nth(i).unwrap();
                for j in i + 1..end {
                    let r = self.message.chars().nth(j).unwrap();
                    if l == r {
                        not_it = true;
                        break;
                    }
                }

                if not_it {
                    break;
                }
            }
            start += 1;
            end += 1;
        }

        end - 1
    }
}

impl FromStr for Marker {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Marker {
            message: s.to_string(),
        })
    }
}

pub fn run(input: &str) -> Result<()> {
    let marker: Marker = input.parse().unwrap();
    println!("Packet start: {}", marker.find_marker(4));
    println!("Message start: {}", marker.find_marker(14));
    Ok(())
}
