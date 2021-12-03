use std::fs;

const VALUE_SIZE: usize = 12;

enum Sensor {
    Co2,
    O2,
}

// bit_pos is the bit pos to analyse
fn find_sensor_value(l1: &Vec<usize>, l2: &Vec<usize>, bit_pos: usize, sensor: Sensor) -> Result<usize, String> {
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

    let mut first_bit_0: Vec<usize> = vec![];
    let mut first_bit_1: Vec<usize> = vec![];

    // Generate the 2 lists and call this function again
    for val in list {
        match val & (1 << bit_pos) {
            0 => first_bit_0.push(*val),
            _ => first_bit_1.push(*val),
        };
    }

    if bit_pos == 0 {
        if first_bit_0.len() > 1 || first_bit_1.len() > 1 {
            return Err(String::from("I cannot decide"));
        }
        return if first_bit_0[0] & 1 != 0 {
            match sensor {
                Sensor::O2 => Ok(first_bit_0[0]),
                Sensor::Co2 => Ok(first_bit_1[0]),
            }
        } else {
            match sensor {
                Sensor::O2 => Ok(first_bit_1[0]),
                Sensor::Co2 => Ok(first_bit_0[0]),
            }
        }
    }

    find_sensor_value(&first_bit_0, &first_bit_1, bit_pos - 1, sensor)
}

pub fn run(inputfile: &str) -> Result<(), String> {
    print!("[Binary]...      ");

    let contents = match fs::read_to_string(inputfile) {
        Ok(c) => c,
        Err(e) => {
            return Err(e.to_string());
        }
    };

    // The number of values (lines) in the report
    let mut val_count = 0;

    // The number of bits at 1 in each position
    let mut first_bit_0: Vec<usize> = vec![];
    let mut first_bit_1: Vec<usize> = vec![];

    for line in contents.split('\n') {
        if line.is_empty() {
            continue;
        }

        val_count += 1;

        let cs: Vec<char> = line.chars().collect();
        let mut val = 0;

        for i in 0..VALUE_SIZE {
            if cs[i] == '1' {
                val += 1 << (VALUE_SIZE - 1 - i);
            }
        }

        match val & (1 << (VALUE_SIZE-1)) {
            0 => first_bit_0.push(val),
            _ => first_bit_1.push(val),
        };
    }

    let o2 = find_sensor_value(&first_bit_0, &first_bit_1, (VALUE_SIZE-2), Sensor::O2).unwrap();
    let co2 = find_sensor_value(&first_bit_0, &first_bit_1, (VALUE_SIZE-2), Sensor::Co2).unwrap();

    println!("O2: {}, Co2: {} -> {}", o2, co2, o2 * co2);

    Ok(())
}