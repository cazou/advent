type Image = Vec<Vec<u8>>;
type Code = Vec<u8>;

fn get_pixel_at(image: &Image, x: isize, y: isize, step: u8) -> u8 {
    return if x < 0 || y < 0 || x >= image.len() as isize || y >= image.len() as isize {
        step % 2
    } else {
        image[x as usize][y as usize]
    }
}

fn encode_pixel_at(image: &Image, code: &Code, x: isize, y: isize, step: u8) -> u8 {
    let mut i = 8;
    let mut idx = 0;
    let s = if code[0] == 0 {
        0
    } else {
        step
    };

    for a in x-1..=x+1 {
        for b in y-1..=y+1 {
            if get_pixel_at(image, a,b, s) == 1 {
                idx += 1 << i;
            }
            i -= 1;
        }
    }

    code[idx]
}

fn encode(image: &Image, code: &Code, step: u8) -> Image {
    let mut ret = vec![];
    for x in -1..=image.len() as isize {
        ret.push(vec![]);
        for y in -1..=image.len() as isize {
            ret.last_mut().unwrap().push(encode_pixel_at(image, code, x, y, step));
        }
    }

    ret
}

fn make_code(line: &str) -> Result<Vec<u8>, String> {
    let ret: Vec<u8> = line.chars().filter_map(|c| match c {
        '.' => Some(0),
        '#' => Some(1),
        _ => None
    }).collect();

    Ok(ret)
}

fn lit_pixels(image: &Image) -> u32 {
    image.iter().map(|i| i.iter().sum::<u8>() as u32).sum()
}

pub fn run(contents: &str) -> Result<(), String> {
    print!("[Images]...    ");

    let mut code: Option<Vec<u8>> = None;
    let mut image: Vec<Vec<u8>> = vec![];

    for line in contents.lines() {
        if line.is_empty() {
            continue;
        }

        if let None = code {
            code = match make_code(line) {
                Ok(c) => Some(c),
                Err(e) => return Err(e)
            };
            continue;
        }

        image.push(make_code(line).unwrap());
    }

    let code = match code {
        Some(c) => c,
        None => return Err("Could not get code".to_string())
    };

    for step in 0..2 {
        image = encode(&image, &code, step);
    }
    let l2 = lit_pixels(&image);

    for step in 2..50 {
        image = encode(&image, &code, step);
    }
    let l50 = lit_pixels(&image);

    println!("{} {}", l2, l50);

    Ok(())
}