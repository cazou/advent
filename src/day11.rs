use std::cmp::{max, min};
use std::collections::BTreeSet;

fn get_neighbors_range(i: usize, j: usize) -> [usize; 4] {
    let start_i = max(0, i as isize - 1) as usize;
    let start_j = max(0, j as isize - 1) as usize;

    let end_i = min(i+1, 9);
    let end_j = min(j+1, 9);

    [start_i, end_i, start_j, end_j]
}

fn process_point(octopuses: &mut Vec<Vec<usize>>, i: usize, j: usize, flashed: &mut BTreeSet<[usize; 2]>) {
    if octopuses[i][j] < 10 || flashed.contains(&[i,j]) {
        return;
    }

    flashed.insert([i,j]);
    octopuses[i][j] = 0;

    let r = get_neighbors_range(i, j);

    for x in r[0]..=r[1] {
        for y in r[2]..=r[3] {
            if !flashed.contains(&[x, y]) {
                octopuses[x][y] += 1;
                process_point(octopuses, i, j, flashed);
            }
        }
    }
}

fn run_step(octopuses: &mut Vec<Vec<usize>>) -> usize {
    // Increase all values
    for i in 0..10 {
        for j in 0..10 {
            octopuses[i][j] += 1;
        }
    }

    let mut flashed: BTreeSet<[usize; 2]> = BTreeSet::new();
    loop {
        let f = flashed.len();
        for i in 0..10 {
            for j in 0..10 {
                process_point(octopuses, i, j, &mut flashed);
            }
        }
        if flashed.len() == f {
            break;
        }
    }

    flashed.len()
}

fn all_flash(octopuses: &Vec<Vec<usize>>) -> bool {
    for i in 0..10 {
        if octopuses[i].iter().any(|x| *x != 0) {
            return false
        }
    }
    return true;
}

pub fn run(contents: &str) -> Result<(), String> {
    print!("[Dumbo]...       ");

    let mut flashes = 0;
    let mut octopuses: Vec<Vec<usize>> = vec![];

    // Load input data
    for val in contents.lines() {
        if val.is_empty() {
            continue;
        }

        octopuses.push(Vec::from_iter(val.chars().map(|x| x.to_string().parse().unwrap() )));
    }

    for _ in 0..100 {
        let f = run_step(&mut octopuses);
        flashes += f;
    }

    let mut all = 100;

    loop {
        if all_flash(&octopuses) {
            break;
        }
        run_step(&mut octopuses);
        all += 1;
    }

    println!("{} {}", flashes, all);

    Ok(())
}