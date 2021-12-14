use utils::{parse_field};
mod sub;
use sub::{Direction, Position1, Position2};

fn main() {
    let lines = utils::files::read_in_lines("src/instructions.txt");


    println!("----- PART 1 -----");
    // let p = lines.iter()
    //     .map(|l| parse_field!(l => String, " " | i32, ""))
    //     .fold((0, 0, 0), |p, (s, x)| match s.unwrap().as_str() {
    //         "forward" => (p.0 + x.unwrap(), p.1 + x.unwrap()*p.2, p.2),
    //         "up" => (p.0, p.1, p.2 - x.unwrap()),
    //         "down" => (p.0, p.1, p.2 + x.unwrap()),
    //         _ => (p.0, p.1, p.2),
    //     });
    // println!("p=>{:?}", p);

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
