use utils::{files, parse_field_unwrap};

fn main() {
    let filename = "input";
    let instruction_blocks = files::read_in_chunks(filename);

    println!("----- PART 1 -----");
    let mut input: Vec<isize> = vec!(3,9,4,9,4,1,9,5,7,9,9,9,7,9);
    let mut r = Registers::new(input.remove(0));
    for block in instruction_blocks.iter() {
        for i in block.iter() {
            r.execute_instruction(i);
        }
        println!("{:?}", r);
        if input.len() != 0 {
            r.set_input(input.remove(0));
        }
    }

    // Split program into 'sections' on input points
    // Calculate all outputs from all possible inputs at each section
    let mut registers: Vec<Registers> = Vec::new();
    for i in 1..10 {
        registers.push(Registers::new(i));
    }

    // for block in instruction_blocks.iter() {
    //     println!("----- NEW BLOCK -----");
    //     for i in block.iter() {
    //         println!("{}", i)
    //     }
    // }

    for (i, block) in instruction_blocks.iter().enumerate() {
        // Calculate all outputs for inputs 1-9
        for r in registers.iter_mut() {
            for i in block.iter() {
                r.execute_instruction(i);
            }
        }

        // Sort on z values
        // Remove duplicate z outputs
        registers.sort();
        let mut z = registers[0].z;
        let mut max = registers[0].clone();
        let mut update = Vec::new();
        for r in registers.iter() {
            // println!("{:?}", r);
            if r.z == z {
                if r.input_history < max.input_history {
                    max = r.clone();
                }
            } else {
                let mut total = r.z;
                for _ in i..14 {
                    total /= 26
                }
                if total != 0 { break }
                // Generate inputs for next loop
                for i in 1..10 {
                    let mut copy = max.clone();
                    copy.set_input(i);
                    update.push(copy);
                }
                z = r.z;
                max = r.clone();
            }
        }


        for i in 1..10 {
            let mut copy = max.clone();
            copy.set_input(i);
            update.push(copy);
        }

        if i == 13 {
            break;
        }

        registers = update;

        println!("CHUNK {} PROCESSED - INPUT REGISTERS FOR NEXT CHUNK: {}", i, registers.len());

    }

    for r in registers {
        if r.z == 0 {
            println!("{:?}", r);
        }
    }

    println!("Part 1 Answer: {}", 0);

    println!("\n\n----- PART 2 -----");

    println!("Part 2 Answer: {}", 0);

}

fn decrement_input(x: &mut Vec<isize>) {
    let mut index = x.len() - 1;
    while x[index] == 1 {
        x[index] = 9;
        index -= 1;
    }
    x[index] -= 1;

    if index == 7 {
        println!("INPUT: {:?}", x);
    }
}

#[derive(Debug, Clone)]
struct Registers {
    input: isize,
    input_history: String,
    w: isize,
    x: isize,
    y: isize,
    z: isize,
}

impl std::cmp::PartialOrd for Registers {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.z.cmp(&other.z))
    }
}

impl std::cmp::Ord for Registers {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.z.cmp(&other.z)
    }
}

impl std::cmp::PartialEq for Registers {
    fn eq(&self, other: &Self) -> bool {
        self.z == other.z
    }
}

impl std::cmp::Eq for Registers { }

impl Registers {
    pub fn new(input: isize) -> Self {
        Self {
            input,
            input_history: format!("{}", input),
            w: 0,
            x: 0,
            y: 0,
            z: 0,
        }
    }

    pub fn set_input(&mut self, w: isize) {
        self.input_history = format!("{}{}", self.input_history, w);
        self.input = w;
    }

    pub fn execute_instruction(&mut self, s: &str) {
        let (i, arg1, arg2): (String, String , String) = parse_field_unwrap!(s => String, " " | String, " " | String, "");
        match i.as_str() {
            "inp" => self.inp(&arg1),
            "add" => self.add(&arg1, &arg2),
            "mul" => self.mul(&arg1, &arg2),
            "div" => self.div(&arg1, &arg2),
            "mod" => self.modulo(&arg1, &arg2),
            "eql" => self.eql(&arg1, &arg2),
            _ => (),
        }
    }

    fn inp(&mut self, register: &str) {
        match register {
            "w" => self.w = self.input,
            "x" => self.x = self.input,
            "y" => self.y = self.input,
            "z" => self.z = self.input,
            _ => (),
        }
    }

    fn add(&mut self, arg1: &str, arg2: &str) {
        // First arg mut be register, second could be register or number
        let v = if let Ok(v) = arg2.parse::<isize>() {
            v
        } else {
            match arg2 {
                "w" => self.w,
                "x" => self.x,
                "y" => self.y,
                "z" => self.z,
                _ => 0,
            }
        };

        match arg1 {
            "w" => self.w += v,
            "x" => self.x += v,
            "y" => self.y += v,
            "z" => self.z += v,
            _ => (),
        }
    }

    fn mul(&mut self, arg1: &str, arg2: &str) {
        // First arg mut be register, second could be register or number
        let v = if let Ok(v) = arg2.parse::<isize>() {
            v
        } else {
            match arg2 {
                "w" => self.w,
                "x" => self.x,
                "y" => self.y,
                "z" => self.z,
                _ => 0,
            }
        };

        match arg1 {
            "w" => self.w *= v,
            "x" => self.x *= v,
            "y" => self.y *= v,
            "z" => self.z *= v,
            _ => (),
        }
    }

    fn div(&mut self, arg1: &str, arg2: &str) {
        // First arg mut be register, second could be register or number
        let v = if let Ok(v) = arg2.parse::<isize>() {
            v
        } else {
            match arg2 {
                "w" => self.w,
                "x" => self.x,
                "y" => self.y,
                "z" => self.z,
                _ => 0,
            }
        };

        match arg1 {
            "w" => self.w /= v,
            "x" => self.x /= v,
            "y" => self.y /= v,
            "z" => self.z /= v,
            _ => (),
        }
    }

    fn modulo(&mut self, arg1: &str, arg2: &str) {
        // First arg mut be register, second could be register or number
        let v = if let Ok(v) = arg2.parse::<isize>() {
            v
        } else {
            match arg2 {
                "w" => self.w,
                "x" => self.x,
                "y" => self.y,
                "z" => self.z,
                _ => 0,
            }
        };

        match arg1 {
            "w" => self.w %= v,
            "x" => self.x %= v,
            "y" => self.y %= v,
            "z" => self.z %= v,
            _ => (),
        }
    }

    fn eql(&mut self, arg1: &str, arg2: &str) {
        // First arg mut be register, second could be register or number
        let v = if let Ok(v) = arg2.parse::<isize>() {
            v
        } else {
            match arg2 {
                "w" => self.w,
                "x" => self.x,
                "y" => self.y,
                "z" => self.z,
                _ => 0,
            }
        };

        match arg1 {
            "w" => self.w = if self.w == v { 1 } else { 0 },
            "x" => self.x = if self.x == v { 1 } else { 0 },
            "y" => self.y = if self.y == v { 1 } else { 0 },
            "z" => self.z = if self.z == v { 1 } else { 0 },
            _ => (),
        }
    }
}

fn execute_chunk(step: usize, z: isize, w: isize, div: isize, x_offset: isize, y_offset: isize) -> isize {
    if z%26 + x_offset == w { z / div } else { w + y_offset + 26 * (z / div)}
}