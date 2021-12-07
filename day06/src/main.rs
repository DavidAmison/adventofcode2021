use utils::{files, parse_field};

fn reproduce(age: u128, days_left: u128) -> u128 {
    // Include day 0
    if days_left >= (age + 1) {
        let new_days_left = days_left - (age + 1);
        reproduce(8, new_days_left) + reproduce(6, new_days_left)
    } else {
        1
    }
}

fn main() {
    let filename = "src/input.txt";
    let ages = files::read_in_sv_as::<u128>(filename, ",");

    // Some slightly optimized brute force!!! (i.e. only calculate once per age)
    println!("----- PART 1 -----");
    let days = 80;
    let known_descendents_80: Vec<u128> = (0..=6).map(|age| descendents(age, days)).collect();
    // let known_descendents_80: Vec<u128> = (0..=6).map(|age| reproduce(age, days)).collect();
    println!("{:?}", known_descendents_80);
    let total_descendents: u128 = ages.iter().map(|age| known_descendents_80[*age as usize]).sum();

    println!("Part 1 Answer: {}", total_descendents);

    println!("\n\n----- PART 2 -----");
    let days = 256;
    let known_descendents_256: Vec<u128> = (0..=6).map(|age| descendents(age, days)).collect();
    // let known_descendents_256: Vec<u128> = (1..=5).map(|age| reproduce(age, days)).collect();
    println!("{:?}", known_descendents_256);
    let total_descendents: u128 = ages.iter().map(|age| known_descendents_256[*age as usize]).sum();
    println!("Part 2 Answer: {}", total_descendents);
}

fn descendents(start_age: u128, days: u128) -> u128 {
    // Get to first reproduction point
    let remaining_days = days - (start_age + 1);
    let mut sixes = remaining_days/7;
    let mut eights = 0;
    let mut descendents = 2 * binomial_coefficient(sixes, eights);
    while sixes > 0 {
        sixes -= 1;
        eights += 1;
        if sixes*7 + eights*9 > remaining_days {
            eights -= 1;
            descendents += binomial_coefficient(sixes, eights);
        } else  {
            descendents += 2 * binomial_coefficient(sixes, eights);
        }
    };
    descendents
}

// (a + b)! / (a! * b!) with some optimization to reduce overflows
fn binomial_coefficient(a: u128, b: u128) -> u128 {
    let t: u128 = a + b;
    if a == 0 {
        1
    } else if b == 0 {
        1
    } else if a > b {
        ((a+1)..=t).product::<u128>() / (1..=b).product::<u128>()
    } else {
        ((b+1)..=t).product::<u128>() / (1..=a).product::<u128>()
    }
}
