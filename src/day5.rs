fn parse_point(text: &str) -> [usize; 2]
{
    let vals: Vec<&str> = text.split(',').collect();

    [vals[0].parse().unwrap(), vals[1].parse().unwrap()]
}

fn add_points_from_line(line: &str, list: &mut Vec<[u8; 1000]>, intersections: &mut usize)
{
    let points: Vec<&str> = line.split("->").collect();
    let p1 = parse_point(points[0].trim());
    let p2 = parse_point(points[1].trim());

    if p1[0] != p2[0] && p1[1] != p2[1] {
        // Manage diagonal
        let len: isize = p1[0] as isize - p2[0] as isize;
        let len = len.abs();
        let x_step;
        let y_step;

        if p1[0] < p2[0] {
            x_step = 1;
        } else {
            x_step = -1;
        }

        if p1[1] < p2[1] {
            y_step = 1;
        } else {
            y_step = -1;
        }

        for i in 0..=len {
            let x = (p1[0] as isize + i * x_step) as usize;
            let y = (p1[1] as isize + i * y_step) as usize;

            list[x][y] += 1;
            if list[x][y] == 2 {
                *intersections += 1;
            }
        }

        return;
    }


    // Manage horizontal + vertical
    let x_range = match p2[0] < p1[0] {
        true => [p2[0], p1[0]],
        false => [p1[0], p2[0]],
    };

    let y_range = match p2[1] < p1[1] {
        true => [p2[1], p1[1]],
        false => [p1[1], p2[1]],
    };

    for x in x_range[0]..=x_range[1] {
        for y in y_range[0]..=y_range[1] {
            list[x][y] += 1;
            if list[x][y] == 2 {
                *intersections += 1;
            }
        }
    }
}

pub fn run(contents: &str) -> Result<(), String>
{
    print!("[Vents]...       ");

    let mut points: Vec<[u8; 1000]> = vec![[0; 1000]; 1000];
    let mut intersections: usize = 0;

    for line in contents.split('\n') {
        if line.is_empty() {
            continue;
        }

        add_points_from_line(line, &mut points, &mut intersections);
    }

    println!("{}", intersections);

    Ok(())
}