use utils::{files, parse_field};
mod transform;
use transform::*;
use std::collections::HashMap;

fn main() {
    let filename = "input";
    let scanners = files::read_in_matrix_chunks_with_header_as::<isize>(filename, "scanner");
    let mut scanners = scanners.iter()
        .map(|s| s.iter().map(|p| (p[0], p[1], p[2])).collect::<Vec<XYZ>>())
        .collect::<Vec<Vec<XYZ>>>();

    // for (i, scanner) in scanners.iter().enumerate() {
    //     println!("---- SCANNER {} ----", i);
    //     for point in scanner {
    //         println!("{:?}", point);
    //     }
    // }

    println!("----- PART 1 -----");
    let mut points = scanners.remove(0);

    let mut scanner_offsets = Vec::new();
    // Get matrix of all orientations
    while let Some(scanner) = scanners.pop() {
        // Get all 24 possible orientations
        let current_points = points.clone();
        let mut found_match = false;
        for orientation in rotations(&scanner) {
            let mut distances = HashMap::new();
            for p1 in orientation.iter() {
                for p2 in points.iter() {
                    let offset = ((p1.0 - p2.0), (p1.1 - p2.1), (p1.2 - p2.2));
                    let e = distances.entry(offset).or_insert(0);
                    *e += 1;
                    if *e >= 12 {
                        // MATCH
                        found_match = true;
                        println!("MATCH WITH OFFSET = {:?}", offset);
                        scanner_offsets.push(offset);
                        // transform by distance and push non-duplicated points
                        for new_point in orientation.iter().map(|p| rebase(*p, offset)) {
                            points.push(new_point)
                        }
                        break;
                    }
                }
                if found_match == true {
                    break;
                }
            }
            if found_match == true {
                break;
            }
        }
        if found_match == false {
            scanners.insert(0, scanner);
        } else {
            points.sort();
            points.dedup();
        }
    }

    // for point in points.iter() {
    //     println!("{:?}", point);
    // }

    // println!("{}", points.len());


    println!("Part 1 Answer: {}", points.len());

    println!("\n\n----- PART 2 -----");

    let mut max_distance = 0;
    for o1 in scanner_offsets.iter() {
        for o2 in scanner_offsets.iter() {
            let d = (o2.0 - o1.0).abs() + (o2.1 - o1.1).abs() + (o2.2 - o1.2).abs();
            if d > max_distance {
                max_distance = d;
            }
        }
    }
    println!("Part 2 Answer: {}", max_distance);

}
