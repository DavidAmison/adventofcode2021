use utils::{files};

fn main() {
    let filename = "src/input.txt";
    let matrix = files::read_in_matrix_as::<u32>(filename);
    let report_length = matrix.len();
    let num_length = matrix[0].len();
    let mut sums: Vec<u32> = vec!(0; num_length);

    println!("----- PART 1 -----");
    for number in matrix.iter() {
        for (i, digit) in number.iter().enumerate() {
            sums[i] += *digit;
        }
    }
    let mut gamma_rate = String::new();
    let mut epsilon_rate = String::new();
    for sum in sums.iter() {
        if *sum > (report_length/2) as u32 {
            gamma_rate.push_str("1");
            epsilon_rate.push_str("0");
        } else {
            gamma_rate.push_str("0");
            epsilon_rate.push_str("1");
        }
    }
    let gamma = u32::from_str_radix(&gamma_rate, 2).unwrap();
    let epsilon = u32::from_str_radix(&epsilon_rate, 2).unwrap();
    println!("Part 1 Answer: {}", gamma * epsilon);

    println!("\n\n----- PART 2 -----");

    // Filter for 02 rating
    let mut o2 = matrix.clone();
    for i in 0..num_length {
        // Find the most common bit
        let mut filter = most_common_bit(&o2, i, true);
        o2 = o2.into_iter()
        .filter(|num| num[i] == filter)
        .collect();
        if o2.len() == 1 {
            break;
        }
    }

    let mut co2 = matrix.clone();
    for i in 0..num_length {
        // Find the most common bit
        let mut filter = most_common_bit(&co2, i, true);
        co2 = co2.into_iter()
            .filter(|num| num[i] != filter)
            .collect();
        if co2.len() == 1 {
            break;
        }
    }
    println!("O2: {:?}", o2[0]);
    println!("CO2: {:?}", co2[0]);

    let mut o2_rating = 0;
    for bit in &o2[0] {
        o2_rating <<= 1;
        o2_rating += bit;
    }
    let mut co2_rating = 0;
    for bit in &co2[0] {
        co2_rating <<= 1;
        co2_rating += bit;
    }
    println!("O2: {}, CO2: {}", o2_rating, co2_rating);
    println!("O2 * CO2 = {}", o2_rating * co2_rating);

}

fn most_common_bit(numbers: &Vec<Vec<u32>>, index: usize, favour_1: bool) -> u32 {
    let mut sum = 0;
    let length = numbers.len() as u32;
    for num in numbers {
        sum += num[index];
    }
    if sum * 2 == length {
        if favour_1 { 1 } else { 0 }
    } else if sum * 2 > length {
        1
    } else {
        0
    }
}
