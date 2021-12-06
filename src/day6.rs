
/*
This contains 2 implementations.
The first one creates a Fish object for each fish. It works for 80 days but is not scalable for
256 (memory and CPU wise). The second implementation just keeps a number of fish at each stage and
update it for each day.
*/

struct Fish {
    age: usize
}

impl Fish {
    pub fn new_born() -> Fish
    {
        Fish::from_age(8)
    }

    pub fn from_age(age: usize) -> Fish
    {
        Fish {
            age
        }
    }

    pub fn new_day(&mut self) -> Option<Fish>
    {
        if self.age == 0 {
            self.age = 6;
            return Some(Fish::new_born());
        }

        self.age -= 1;

        None
    }
}

pub fn run(contents: &str) -> Result<(), String> {
    print!("[Fish]...        ");

    let mut list: Vec<Fish> = vec![];

    // Load input data
    for val in contents.split(',') {
        if val.is_empty() {
            continue;
        }

        let val: usize = val.trim().parse().unwrap();

        list.push(Fish::from_age(val));
    }

    for day in 0..80 {
        let fish_count = list.len();

        for i in 0..fish_count {
            match list[i].new_day() {
                None => continue,
                Some(f) => list.push(f),
            }
        }
    }

    println!("Day 80: {}", list.len());

    Ok(())
}

fn new_day(fishes: &[u64; 9]) -> [u64; 9]
{
    let mut ret = [0; 9];

    ret[8] = fishes[0];
    ret[7] = fishes[8];
    ret[6] = fishes[7] + fishes[0];
    ret[5] = fishes[6];
    ret[4] = fishes[5];
    ret[3] = fishes[4];
    ret[2] = fishes[3];
    ret[1] = fishes[2];
    ret[0] = fishes[1];

    ret
}

pub fn run2(contents: &str) -> Result<(), String> {
    print!("                 ");

    let mut fishes = [0; 9];

    // Load input data
    for val in contents.split(',') {
        if val.is_empty() {
            continue;
        }

        let val: usize = val.trim().parse().unwrap();

        fishes[val] += 1;
    }

    for day in 0..256 {
        fishes = new_day(&fishes);
    }

    let mut sum = 0;
    for i in 0..9 {
        sum += fishes[i];
    }

    println!("Day 256: {}", sum);

    Ok(())
}