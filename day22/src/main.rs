use utils::{files, parse_field_unwrap};
use std::collections::HashMap;

// mod cube;
use cube::*;

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
            x: (i.1, i.2+1),
            y: (i.3, i.4+1),
            z: (i.5, i.6+1),
        }
    }).collect();

    // Add cubes to a list with following rules:
    // - Check for all intersections with previous cubes and 'nullify' that intersection
    //   by adding a negative weighted cube
    // - If we have an ON instruction then add the cube in the instruction

    // Weighted cubes (+1 or -1)
    let mut cubes: Vec<(Cube, isize)> = Vec::new();
    for step in steps.iter() {
        let cube = Cube::new(step.x, step.y, step.z);
        let mut update: Vec<(Cube, isize)> = Vec::new();
        for (c, i) in cubes.iter() {
            if let Some(int) = cube.intersection(&c) {
                update.push((int, -1 * i));  // Insert opposite (i.e. nullify cube)
            }
        }
        match step.instr {
            Instruction::On => update.push((cube, 1)),
            Instruction::Off => (), // Do nothing
        };

        cubes.append(&mut update);
    }

    println!("----- PART 1 -----");

    let count_area = Cube::new((-50, 51), (-50, 51), (-50, 51));
    let volume = cubes.iter().map(|(c, w)| (c.intersection(&count_area), w))   // Restrict to area of interest
        .filter(|(c, _)| c.is_some())                                          // Filter out non-intersecting cubes
        .fold(0, |vol, (c, w)| vol + w * c.unwrap().volume());                 // Calculate weighted volume

    println!("Part 1 Answer: {}", volume);

    println!("\n\n----- PART 2 -----");

    let volume = cubes.iter().fold(0, |vol, (c, w)| vol + w * c.volume());

    println!("Part 2 Answer: {}", volume);
}

mod cube {
    pub type Range = (isize, isize);

    #[derive(Debug)]
    pub struct Cube {
        x: Range,
        y: Range,
        z: Range,
    }

    impl Cube {
        pub fn new(x: Range, y: Range, z: Range) -> Self {
            Cube {
                x,
                y,
                z,
            }
        }

        pub fn volume(&self) -> isize {
            (self.x.1 - self.x.0) * (self.y.1 - self.y.0) * (self.z.1 - self.z.0)
        }

        pub fn intersection(&self, other: &Self) -> Option<Cube> {
            let x = (std::cmp::max(self.x.0, other.x.0), std::cmp::min(self.x.1, other.x.1));
            let y = (std::cmp::max(self.y.0, other.y.0), std::cmp::min(self.y.1, other.y.1));
            let z = (std::cmp::max(self.z.0, other.z.0), std::cmp::min(self.z.1, other.z.1));
            if x.0 < x.1 && y.0 < y.1 && z.0 < z.1 {
                Some(Cube{x, y, z})
            } else {
                None
            }
        }
    }

    #[derive(Debug)]
    pub struct RebootStep {
        pub instr: Instruction,
        pub x: Range,
        pub y: Range,
        pub z: Range,
    }

    #[derive(Debug, PartialEq)]
    pub enum Instruction {
        On,
        Off,
    }
}

