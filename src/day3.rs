use std::fs;

const VALUE_SIZE: usize = 12;

enum Sensor {
    Co2,
    O2,
}

// bit_pos is the bit pos to analyse
fn find_sensor_value(list: &Vec<usize>, bit_pos: usize, sensor: Sensor) -> Result<usize, String> {

    let mut l1: Vec<usize> = vec![];
    let mut l2: Vec<usize> = vec![];

    // Generate the 2 lists
    for val in list {
        match val & (1 << bit_pos) {
            0 => l1.push(*val),
            _ => l2.push(*val),
        };
    }

    // Select the lists to generate each sensor
    let &mut o2_list;
    let &mut co2_list;

    if l1.len() > l2.len() {
        o2_list = l1;
        co2_list = l2;
    } else if l1.len() < l2.len() {
        o2_list = l2;
        co2_list = l1;
    } else {
        if l1[0] & (1 << (bit_pos + 1)) == 0 {
            o2_list = l2;
            co2_list = l1;
        } else {
            o2_list = l1;
            co2_list = l2;
        }
    }

    let list = match sensor {
        Sensor::O2 => o2_list,
        Sensor::Co2 => co2_list,
    };

    if list.len() == 1 {
        return Ok(list[0]);
    }

    find_sensor_value(&list, bit_pos - 1, sensor)
}

pub fn run(contents: &str) -> Result<(), String> {
    print!("[Binary]...      ");

    // The number of bits at 1 in each position
    let mut list: Vec<usize> = vec![];

    for line in contents.split('\n') {
        if line.is_empty() {
            continue;
        }

        let cs: Vec<char> = line.chars().collect();
        let mut val = 0;

        for i in 0..VALUE_SIZE {
            if cs[i] == '1' {
                val += 1 << (VALUE_SIZE - 1 - i);
            }
        }

        list.push(val);
    }

    let o2 = find_sensor_value(&list, VALUE_SIZE - 1, Sensor::O2).unwrap();
    let co2 = find_sensor_value(&list, VALUE_SIZE - 1, Sensor::Co2).unwrap();

    println!("O2: {}, Co2: {} -> {}", o2, co2, o2 * co2);

    Ok(())
}