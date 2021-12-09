use std::collections::BTreeSet;

fn is_min(map: &Vec<Vec<u8>>, pos: [usize; 2]) -> bool {
    let i = pos[0];
    let j = pos[1];
    let val = map[i][j];
    (i == 0 || map[i-1][j] > val) &&
        (j == 0 || map[i][j-1] > val) &&
        (i == map.len()-1 || map[i+1][j] > val) &&
        (j == map[i].len()-1 || map[i][j+1] > val)
}

fn find_connected(map: &mut Vec<Vec<u8>>, pos: [usize; 2], entries: &mut BTreeSet<[usize; 2]>) {
    let i = pos[0];
    let j = pos[1];

    entries.insert(pos);
    if i > 0 && map[i-1][j] < 9 && !entries.contains(&[i-1, j]) {
        find_connected(map, [i-1, j], entries);
    }

    if j > 0 && map[i][j-1] < 9 && !entries.contains(&[i, j-1]) {
        find_connected(map, [i, j-1], entries);
    }

    if i < (map.len()-1) && map[i+1][j] < 9 && !entries.contains(&[i+1, j]) {
        find_connected(map, [i+1, j], entries);
    }

    if j < (map[i].len()-1) && map[i][j+1] < 9 && !entries.contains(&[i, j+1]) {
        find_connected(map, [i, j+1], entries);
    }
}

fn build_basin(map: &mut Vec<Vec<u8>>, pos: [usize; 2]) -> usize {
    let mut entries = BTreeSet::new();

    find_connected(map, pos, &mut entries);

    entries.len()
}

fn make_basins(map: &mut Vec<Vec<u8>>) -> Vec<usize> {
    let mut basins = vec![];
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if is_min(map, [i,j]) {
                basins.push(build_basin(map, [i,j]));
                //println!("{}", build_basin(map, [i,j]));
            }
        }
    }

    basins
}

pub fn run(contents: &str) -> Result<(), String> {
    print!("[Tubes]...       ");

    let mut map: Vec<Vec<u8>> = vec![];

    // Load input data
    for val in contents.lines() {
        if val.is_empty() {
            continue;
        }

        map.push(Vec::from_iter(
            val.chars().map(|x| {
                let t: u8 = String::from(x).parse().unwrap();
                t
            })
        ));
    }

    let mut lows: usize = 0;
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if is_min(&map, [i,j]) {
                lows += (map[i][j] + 1) as usize;
            }
        }
    }

    let mut basins = make_basins(&mut map);

    basins.sort();
    basins.reverse();

    println!("{}, {}", lows, basins[0] * basins[1] * basins[2]);
    Ok(())
}