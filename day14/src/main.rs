use utils::{files, parse_field_unwrap};
use std::collections::HashMap;

type Fields = (String, char);

fn main() {
    let filename = "input";
    let input = files::read_in_lines(filename);
    let rules: Vec<Fields> = input.iter().map(|l| parse_field_unwrap!(l => String, " -> " | char, "")).collect();
    let template = "FNFPPNKPPHSOKFFHOFOC";

    // Find count of character pairs after 1 steps for each rule
    let rule_output: HashMap<String, HashMap<(char, char), usize>> = rules.iter().map(|r| {
        // Assume no rule of form AA -> A because I am lazy and know it is true
            (String::from(&r.0), HashMap::from(
                [((r.0.chars().nth(0).unwrap(), r.1), 1),
                ((r.1, (r.0.chars().nth(1).unwrap())), 1),]
            ))
    }).collect();

    println!("----- PART 1 -----");
    // Get current pair counts
    let mut pair_count = HashMap::new();
    for pair in template[0..].chars().zip(template[1..].chars()) {
        let v = pair_count.entry(pair).or_insert(0);
        *v += 1;
    }

    for _ in 0..10 {
        let mut next_pair_count = HashMap::new();
        for (pair, count) in pair_count.iter() {
            for (inner_pair, inner_count) in rule_output.get(&format!("{}{}", pair.0, pair.1)).unwrap().iter() {
                let v = next_pair_count.entry(*inner_pair).or_insert(0);
                *v += inner_count * count;
            }
        }
        pair_count = next_pair_count;
    }

    let mut char_count_10 = HashMap::new();
    char_count_10.insert(template.chars().next().unwrap(), 1);
    for ((_, r), count) in pair_count.iter() {
        let v = char_count_10.entry(*r).or_insert(0);
        *v += count;
    }

    let find_most_least = |r: ((char, usize), (char, usize)), (k, v): (&char, &usize)| -> ((char, usize), (char, usize)) {
        if *v > r.0.1 {
            ((*k, *v), (r.1.0, r.1.1))
        } else if *v < r.1.1 {
            ((r.0.0, r.0.1), (*k, *v))
        } else {
            r
        }
    };

    let (most, least) = char_count_10.iter().fold(((' ', 0), (' ', 99999)), find_most_least);

    println!("Part 1 Answer: {:?} - {:?} = {}", most, least, most.1 - least.1);

    println!("\n\n----- PART 2 -----");

    for _ in 10..40 {
        let mut next_pair_count = HashMap::new();
        for (pair, count) in pair_count.iter() {
            for (inner_pair, inner_count) in rule_output.get(&format!("{}{}", pair.0, pair.1)).unwrap().iter() {
                let v = next_pair_count.entry(*inner_pair).or_insert(0);
                *v += inner_count * count;
            }
        }
        pair_count = next_pair_count;
    }

    let mut char_count_40 = HashMap::new();
    char_count_40.insert(template.chars().next().unwrap(), 1);
    for ((_, r), count) in pair_count.iter() {
        let v = char_count_40.entry(*r).or_insert(0);
        *v += count;
    }

    let (most, least) = char_count_40.iter().fold(((' ', 0), (' ', 9999999999999)), find_most_least);
    println!("Part 2 Answer: {:?} - {:?} = {}", most, least, most.1 - least.1);

}
