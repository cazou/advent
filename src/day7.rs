fn align_crabs_to(crabs: &Vec<usize>, pos: usize) -> usize
{
    let mut fuel: usize = 0;
    for crab in crabs {
        let f = (*crab as isize - pos as isize);
        fuel += fuel_to_move(f.abs() as usize);
    }

    fuel
}

fn fuel_to_move(steps: usize) -> usize
{
    // The sum of the N consecutive numbers x..x+N is
    //   (N / 2) * (x + x + N)
    // In this case we want 1..steps, which is
    //   (steps / 2) * (1 + steps)
    // To avoid integer division issues, it can be simplified to:
    //   (steps * (1 + steps))/2

    (steps * (1 + steps))/2
}

pub fn run(contents: &str) -> Result<(), String> {
    print!("[Crabs]...       ");

    let mut crabs = vec![];
    let mut min: usize = 0; //Because there is a 0 in the input (that's a cheat)
    let mut max: usize = 0;

    // Load input data
    for val in contents.split(',') {
        if val.is_empty() {
            continue;
        }

        let val: usize = val.trim().parse().unwrap();

        if val > max {
            max = val;
        }

        crabs.push(val);
    }

    let mut min_pos_fuel: Option<[usize; 2]> = None;

    for i in min..=max {
        let fuel = align_crabs_to(&crabs, i);

        min_pos_fuel = match min_pos_fuel {
            None => Some([i, fuel]),
            Some(p) => {
                if fuel < p[1] {
                    Some([i, fuel])
                } else {
                    Some(p)
                }
            }
        };
    }

    if let Some(p) = min_pos_fuel {
        println!("{}", p[1]);
    }

    Ok(())
}