use utils::{files};

mod packet;
use packet::*;

type Fields = (String,);

fn main() {
    let filename = "input";
    let packet: String = files::read_in_line(filename);
    // Hex to Binary String
    let packet = packet.chars()
        .map(|c| hex_to_bin(c))
        .fold(String::new(), |mut l, s| {
            l.push_str(&s);
            l
        });

    // println!("----- TEST -----");
    // println!("{:?}", parse_packet("110100101111111000101000"));
    // println!("{:#?}", parse_packet("00111000000000000110111101000101001010010001001000000000"));

    let packets = parse_packet(&packet).unwrap().0;

    println!("----- PART 1 -----");
    let version_sum = sum_versions(&packets);

    println!("Part 1 Answer: {}", version_sum);

    println!("\n\n----- PART 2 -----");
    let answer = evaluate_packet(&packets);

    // println!("\n----- TEST -----");
    // let test_packet = "9C0141080250320F1802104A08";
    // let packet = test_packet.chars()
    //     .map(|c| hex_to_bin(c))
    //     .fold(String::new(), |mut l, s| {
    //         l.push_str(&s);
    //         l
    // });
    // println!("{} -> {}",test_packet, evaluate_packet(&parse_packet(&packet).unwrap().0));

    println!("\nPart 2 Answer: {}", answer);

}

fn hex_to_bin(c: char) -> &'static str {
    match c {
        '0' => "0000",
        '1' => "0001",
        '2' => "0010",
        '3' => "0011",
        '4' => "0100",
        '5' => "0101",
        '6' => "0110",
        '7' => "0111",
        '8' => "1000",
        '9' => "1001",
        'A' => "1010",
        'B' => "1011",
        'C' => "1100",
        'D' => "1101",
        'E' => "1110",
        'F' => "1111",
        _ => "",
    }
}
