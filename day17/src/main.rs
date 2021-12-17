use utils::{files, parse_field, parse_field_unwrap};
use std::collections::HashMap;

fn main() {
    let filename = "input";
    let line = files::read_in_line(filename);
    let (_, x1, x2, y1, y2) = parse_field_unwrap!(&line => String , "x=" | i32 , ".." | i32 , ", y=" | i32 , ".." | i32 , "\n");
    println!("{:?} {:?} {:?} {:?}", x1, x2, y1, y2);

    println!("----- PART 1 -----");
    let vy_max = (-y1) - 1;
    let max_height = vy_max * (vy_max + 1) / 2;

    println!("Part 1 Answer: {}", max_height);

    println!("\n\n----- PART 2 -----");
    // HashMap of times of intersection for different y-velocities
    let mut times = HashMap::new();
    for v_y_start in y1..=(-y1) {
        let mut v_y = v_y_start;
        let mut y_pos = 0;
        let mut t = 0;
        while y_pos >= y1 {
            y_pos += v_y;
            v_y -= 1;
            t += 1;
            if (y1..=y2).contains(&y_pos) {
                let velocities = times.entry(t).or_insert(Vec::new());
                velocities.push(v_y_start);
            }
        }
    }
    // for time in times.iter() {
    //     println!("{:?}", time);
    // }

    // Now check all viable x-velocities (i.e. 0..=x2)
    // We keep a vector so that duplicate pairs can be removed (i.e when a set of speeeds
    // would intersect more than once)
    let mut speeds = Vec::new();
    for v_x_start in 0..=x2 {
        let mut v_x = v_x_start;
        let mut x_pos = 0;
        let mut t = 0;
        // Since x_vel can reach 0 and to avoid infinite loops
        // we limit to a maximum time or 2 times the max y-coordinate
        // since this is the maximum possible time (when v_y == -(y1+1))
        // Potentially would have been better to use x_velocities for the
        // original hashmap then find intersections for y???
        while x_pos <= x2 && t < y1*-2 {
            x_pos += v_x;
            if v_x > 0 {
                v_x -= 1;
            }
            t += 1;
            if (x1..=x2).contains(&x_pos) {
                let velocities = times.entry(t).or_insert(Vec::new());
                for v in velocities.iter() {
                    speeds.push((v_x_start, *v));
                }
                // count += velocities.len();
            }
        }
    }

    // Remove deplicates
    speeds.sort();
    speeds.dedup();

    // for input in speeds.iter() {
    //     println!("{:?}", input);
    // }

    println!("Part 2 Answer: {:#?}", speeds.len());

}
