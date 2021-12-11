use utils::{files};

mod octopi;
use octopi::{OctopiMatrix, Octopus};

fn main() {
    let filename = "input";
    let input = files::read_in_matrix_as::<u32>(filename);
    let mut octopi = OctopiMatrix::new();
    octopi.octopi = input.iter()
        .map(
            |r| r.iter()
                .map(|energy| Octopus::Charging(*energy))
                .collect()
        ).collect();

    println!("----- PART 1 -----");
    let mut flashed = 0;
    for _ in 0..100 {
        flashed += octopi.step();
    }

    println!("Part 1 Answer: {} flashes", flashed);

    println!("\n\n----- PART 2 -----");
    let count = octopi.octopi.iter().flatten().count();
    let mut day = 101;  // We've already has 100 days from part 1
    while octopi.step() as usize != count {
        day += 1;
    };

    println!("Part 2 Answer: Day {}", day);

}
