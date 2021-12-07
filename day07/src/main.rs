use utils::{files, parse_field};

fn main() {
    let filename = "src/test_input.txt";
    let positions = files::read_in_sv_as::<i32>(filename, ",");

    println!("----- PART 1 -----");
    let mut minimum = positions.iter().sum();

    let consumption = |x: i32, position: i32| {
        (x - position).abs()
    };

    let mut position = 0;
    loop {
        position += 1;
        let fuel: i32 = positions.iter()
            .map(|x| consumption(*x, position))
            .sum();
        // println!("Offset {} - Fuel {}", offset, fuel);
        if fuel < minimum {
            minimum = fuel
        } else {
            break
        };
    }
    println!("Part 1 Answer: {}", minimum);

    println!("\n\n----- PART 2 -----");
    let mut minimum = positions.iter()
        .map(|x| triangle(*x))
        .sum();

    let triangle_consumption = |x: i32, position: i32| {
        triangle((x - position).abs())
    };

    let mut position = 0;
    loop {
        position += 1;
        let fuel: i32 = positions.iter()
            .map(|x| triangle_consumption(*x, position))
            .sum();
        // println!("Offset {} - Fuel {}", offset, fuel);
        if fuel < minimum {
            minimum = fuel
        } else {
            break
        };
    }
    println!("Part 2 Answer: {}", minimum);

}

fn triangle(x: i32) -> i32 {
    x * (x+1) / 2
}
