use utils::{files, parse_field_unwrap};

fn main() {
    let filename = "src/input.txt";
    let lines = files::read_in_lines(filename);
    let parsed: Vec<(Vec<String>, Vec<String>)> = lines.iter()
        .map( |s| {
            let (l, r) = parse_field_unwrap!(&s => String, " | " | String, "");
            let v1 = l.split_whitespace().map(|s| String::from(s)).collect();
            let v2 = r.split_whitespace().map(|s| String::from(s)).collect();
            (v1, v2)
        }).collect();

    println!("----- PART 1 -----");
    let count_unique = |count, (_, r) : &(_, Vec<String>)| {
        count + r.iter()
            .filter(|s| s.len() == 2 || s.len() == 3 || s.len() == 4 || s.len() == 7)
            .count()
    };
    let count = parsed.iter().fold(0, count_unique);

    println!("Part 1 Answer: {}", count);

    println!("\n\n----- PART 2 -----");
    let sum = parsed.iter().fold(0, |count, input| count + decode_segments(input));

    println!("Part 2 Answer: {}", sum);

}

fn decode_segments((l, r): &(Vec<String>, Vec<String>)) -> u32 {
    let one = l.iter().filter(|s| s.len() == 2).next().unwrap();
    let four = l.iter().filter(|s| s.len() == 4).next().unwrap();
    let seven = l.iter().filter(|s| s.len() == 3).next().unwrap();
    let eight = l.iter().filter(|s| s.len() == 7).next().unwrap();
    // ---- 0, 9 and 6 all light up 6 segments ----
    // 9 contains all segments also lit up by 4
    // 0 in remaining candidates contains all segments lit up by 1
    // 6 is the one remaining
    let nine = l.iter().filter(|s| s.len() == 6)
        .filter(|s| contains_chars(s, four))
        .next().unwrap();
    let zero = l.iter().filter(|s| s.len() == 6)
        .filter(|s| s != &nine && contains_chars(s, one))
        .next().unwrap();
    let six = l.iter().filter(|s| s.len() == 6)
        .filter(|s| s != &nine && s != &zero)
        .next().unwrap();
    // ---- 2, 3 and 5 all light up 5 segments ----
    // 3 contains all segments lit up by 1
    // 5 is contained by segments lit up by 6
    // 2 is remaining
    let three = l.iter().filter(|s| s.len() == 5)
        .filter(|s| contains_chars(s, one))
        .next().unwrap();
    let five = l.iter().filter(|s| s.len() == 5)
        .filter(|s| contains_chars(six, s))
        .next().unwrap();
    let two = l.iter().filter(|s| s.len() == 5)
        .filter(|s| s != &three && s != &five)
        .next().unwrap();
        // println!("0:{} 1:{} 2:{} 3:{} 4:{} 5:{} 6:{} 7:{} 8:{} 9:{}",
        // zero, one, two, three, four, five, six, seven, eight, nine);

    // Now calculate sum of output values
    let mut number = String::new();
    for (i, s) in r.iter().enumerate() {
        number.push_str(if contains_chars(s, zero) && contains_chars(zero, s) { "0" }
        else if contains_chars(s, one) && contains_chars(one, s) { "1" }
        else if contains_chars(s, two) && contains_chars(two, s) { "2" }
        else if contains_chars(s, three) && contains_chars(three, s) { "3" }
        else if contains_chars(s, four) && contains_chars(four, s) { "4" }
        else if contains_chars(s, five) && contains_chars(five, s) { "5" }
        else if contains_chars(s, six) && contains_chars(six, s) { "6" }
        else if contains_chars(s, seven) && contains_chars(seven, s) { "7" }
        else if contains_chars(s, eight) && contains_chars(eight, s) { "8" }
        else if contains_chars(s, nine) && contains_chars(nine, s) { "9" }
        else { "" });
    }
    // println!("OUTPUT {:?} -> {}", r, number);
    return number.parse::<u32>().unwrap();
}

fn contains_chars(s: &str, chars: &str) -> bool {
    chars.chars().fold(true, |result, c| result & s.contains(c))
}
