use utils::{files, parse_field_unwrap};
use std::collections::HashMap;

fn main() {
    let filename = "input";
    let lines = files::read_in_lines(filename);
    let instructions: Vec<(String, isize, isize, isize, isize, isize, isize)> = lines.iter().map(|l| parse_field_unwrap!(l => String, " x=" | isize, ".." | isize, ",y=" | isize, ".." | isize, ",z=" | isize, ".." | isize, "")).collect();
    let steps: Vec<RebootStep> = instructions.iter().map(|i| {
        let instr = match i.0.as_str() {
            "on" => Instruction::On,
            "off" => Instruction::Off,
            _ => panic!("Failed to parse instruction"),
        };
        RebootStep {
            instr,
            x: (i.1, i.2),
            y: (i.3, i.4),
            z: (i.5, i.6),
        }
    }).collect();


    println!("----- PART 1 -----");
    let mut points = HashMap::new();
    for step in steps {
        execute_step(step, &mut points);
    }

    // Count points in 50x50x50 cube
    let mut count = 0;
    for (point, state) in points {
        count += match state {
            State::On => {
                if point_in_range(point, (-50, 50)) {
                    1
                } else {
                    0
                }
            }
            _ => 0
        }
    }
    println!("Part 1 Answer: {}", count);

    println!("\n\n----- PART 2 -----");

    println!("Part 2 Answer: {}", 0);

}

fn execute_step(s: RebootStep, points: &mut HashMap<XYZ, State>) {
    for x in s.x.0..=s.x.1 {
        if x < -50 || x > 50 {
            continue;
        }
        for y in s.y.0..=s.y.1 {
            if y < -50 || y > 50 {
                continue;
            }
            for z in s.z.0..=s.z.1 {
                if z < -50 || z > 50 {
                    continue;
                }
                let p = points.entry((x, y, z)).or_insert(State::Off);
                match s.instr {
                    Instruction::On => *p = State::On,
                    Instruction::Off => *p = State::Off,
                }
            }
        }
    }
}

fn point_in_range(p: XYZ, r: Range) -> bool {
    p.0 >= r.0 && p.0 <= r.1 &&
    p.1 >= r.0 && p.1 <= r.1 &&
    p.2 >= r.0 && p.2 <= r.1
}

fn clamp_range(r: Range, limits: Range) -> Range {
    (
        if r.0 < limits.0 { limits.0 } else if r.0 > limits.1 { limits.1 } else { r.0 },
        if r.1 < limits.0 { limits.0 } else if r.1 > limits.1 { limits.1 } else { r.1 },
    )
}

type XYZ = (isize, isize, isize);

type Range = (isize, isize);

#[derive(Debug)]
struct RebootStep {
    instr: Instruction,
    x: Range,
    y: Range,
    z: Range,
}

#[derive(Debug, PartialEq)]
enum Instruction {
    On,
    Off,
}

#[derive(Debug, PartialEq)]
enum State {
    On,
    Off,
}
