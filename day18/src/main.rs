use utils::files;
mod snailmath;
use snailmath::*;

fn main() {
    let filename = "input";
    let lines = files::read_in_lines(filename);
    let numbers: Vec<SnailNumber> = lines.iter().map(|s| s.parse::<SnailNumber>().unwrap()).collect();

    println!("----- PART 1 -----");
    let mut n = numbers[0].clone();
    n = numbers[1..].iter().fold(n, |mut x, y| {
        x += y.clone();
        x.reduce();
        x
    });

    println!("Part 1 Answer: {}", n.magnitude());

    println!("\n\n----- PART 2 -----");

    let mut max_sum = 0;
    for (i, x) in numbers.iter().enumerate() {
        for (j, y) in numbers.iter().enumerate() {
            if i == j {
                continue;
            }
            let mut x1 = x.clone();
            let mut y1 = y.clone();
            x1 += y.clone();
            y1 += x.clone();
            x1.reduce();
            y1.reduce();
            if x1.magnitude() > max_sum {
                max_sum = x1.magnitude();
                // println!("{:?} + {:?} = {:?}", x, y, x1);
            }
            if y1.magnitude() > max_sum {
                max_sum = y1.magnitude();
                // println!("{:?} + {:?} = {:?}", y, x, y1);
            }
        }
    }

    println!("Part 2 Answer: {}", max_sum);

}
