use std::cmp::max;

struct Target {
    x: [isize; 2],
    y: [isize; 2]
}

impl Target {
    pub fn contains(&self, p: [isize; 2]) -> bool {
        p[0] >= self.x[0] && p[0] <= self.x[1] &&
            p[1] >= self.y[0] && p[1] <= self.y[1]
    }

    pub fn try_hit(&self, v: [isize; 2]) -> bool {
        let mut pos = [0, 0];
        let mut delta_v = v;
        while pos[0] <= self.x[1] && pos[1] >= self.y[0] {
            if self.contains(pos) {
                return true;
            }

            //Update the position and speed
            pos[0] += delta_v[0];
            pos[1] += delta_v[1];

            delta_v[0] = max(0, delta_v[0] - 1);
            delta_v[1] -= 1;
        }

        false
    }
}

pub fn run() -> Result<(), String> {
    print!("[Shot]...        ");
    let target = Target {
        x: [94, 151],
        y: [-156, -103],
    };

    let mut max_height = 0;
    let mut contact_count = 0;

    for x in 1..=target.x[1] {
        for y in target.y[0]..=-target.y[0] {
            if target.try_hit([x, y]) {
                max_height = max((y * (1 + y))/2, max_height);
                contact_count += 1;
            }
        }
    }

    println!("{} {}", max_height, contact_count);

    Ok(())
}