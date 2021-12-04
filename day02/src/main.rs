use utils::{parse_field};
mod sub;
use sub::{Direction, Position1, Position2};

fn main() {
    let lines = utils::files::read_in_lines("src/instructions.txt");
    println!("----- PART 1 -----");
    let mut sub1 = Position1::new();
    for line in lines.iter() {
        match parse_field!(&line => Direction, " " | i32, "") {
            (Some(d), Some(x)) => sub1.execute_instruction(d, x),
            i => println!("Instuction {:?} not recognized", i),
        };
    }
    println!("FINAL POSITION: {:?}", sub1);
    println!("Part 1 Answer: {}", sub1.depth * sub1.horizontal);

    println!("\n\n----- PART 2 -----");
    let mut sub2 = Position2::new();
    for line in lines.iter() {
        match parse_field!(&line => Direction, " " | i32, "") {
            (Some(d), Some(x)) => sub2.execute_instruction(d, x),
            i => println!("Instuction {:?} not recognized", i),
        };
    }
    println!("FINAL POSITION: {:?}", sub2);
    println!("Part 2 Answer: {}", sub2.depth * sub2.horizontal);

    println!("\n\n\n\n");
}
