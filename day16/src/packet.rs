#[derive(Debug)]
pub enum ParsePacketError {
    ParsePacketVersionError,
    ParsePacketTypeError,
    ParsePacketLengthError,
    ParseLiteralError,
}

#[derive(Debug)]
pub enum Operation {
    Sum,
    Product,
    Min,
    Max,
    Greater,
    Less,
    Equal,
}
#[derive(Debug)]
pub enum PacketType {
    Literal,
    Operator(Operation),
}


impl std::str::FromStr for PacketType {
    type Err = ParsePacketError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // println!("Parsing {} to PacketType", s);
        match s {
            "000" => Ok(Self::Operator(Operation::Sum)),
            "001" => Ok(Self::Operator(Operation::Product)),
            "010" => Ok(Self::Operator(Operation::Min)),
            "011" => Ok(Self::Operator(Operation::Max)),
            "100" => Ok(Self::Literal),
            "101" => Ok(Self::Operator(Operation::Greater)),
            "110" => Ok(Self::Operator(Operation::Less)),
            "111" => Ok(Self::Operator(Operation::Equal)),
            _ => Err(Self::Err::ParsePacketTypeError),
        }
    }
}

#[derive(Debug)]
pub enum PacketLength {
    Bits(u64),
    SubPackets(u64),
    None,
}

impl std::str::FromStr for PacketLength {
    type Err = ParsePacketError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // println!("Parsing {} to PacketLength", s);

        match s.chars().nth(0).unwrap() {
            '0' => {
                if let Ok(l) = u64::from_str_radix(&s[1..16], 2) {
                    Ok(Self::Bits(l))
                } else {
                    Err(Self::Err::ParsePacketLengthError)
                }
            }
            '1' => {
                if let Ok(l) = u64::from_str_radix(&s[1..12], 2) {
                    Ok(Self::SubPackets(l))
                } else {
                    Err(Self::Err::ParsePacketLengthError)
                }
            }
            _ => {
                Err(Self::Err::ParsePacketLengthError)
            }
        }
    }
}

#[derive(Debug)]
pub struct Packet {
    pub version: u64,
    pub ty: PacketType,
    pub length: PacketLength,

    pub literal: Option<u64>,
    pub sub_packets: Vec<Packet>,
}

impl Packet {
    pub fn operation(version: u64, ty: PacketType, length: PacketLength, sub_packets: Vec<Packet>) -> Packet {
        Self {
            version,
            ty,
            length,
            literal: None,
            sub_packets,
        }
    }

    pub fn literal(version: u64, ty: PacketType, literal: u64) -> Packet {
        Self {
            version,
            ty,
            length: PacketLength::None,
            literal: Some(literal),
            sub_packets: Vec::new(),
        }
    }

}

/// Parse the next packet - return the Packet and number of 'bits' read
pub fn parse_packet(s: &str) -> Result<(Packet, usize), ParsePacketError> {
    // Recursively parse the packets:
    // LITERAL PACKET:
    //  000 000 [00000]+
    //  VVV TTT [NNNNN]+
    //
    // OPERATOR PACKET:
    //   000 000 0 00000000000[0000] ...
    //   VVV TTT I LLLLLLLLLLL[LLLL] ...

    // println!("PACKET: {}", s);
    let mut p: usize = 0;  // pointer to the current index
    let mut len: usize = 0;  // length of the field being read

    // Parse Version
    len = 3;
    let str_version = &s[p..(p+len)];
    p += len;

    let version = if let Ok(v) = u64::from_str_radix(str_version, 2) {
        v
    } else {
        return Err(ParsePacketError::ParsePacketVersionError)
    };
    // println!("VERSION: {}", version);

    // Parse Type
    len = 3;
    let str_type = &s[p..(p+len)];
    p += len;

    let ty = str_type.parse::<PacketType>()?;
    // println!("TYPE: {:?}", ty);

    // Different Approach for Operator or Literal Packets
    match ty {
        PacketType::Literal => {
            let mut str_literal: String = String::new();
            let len = 5;
            while s.chars().nth(p) != Some('0') {
                str_literal += &s[(p+1)..(p+5)];
                p += len;
            }
            str_literal += &s[(p+1)..(p+5)];
            p += len;
            let literal = u64::from_str_radix(&str_literal, 2).unwrap();
            // println!("LITERAL: {:?}", literal);

            // Create Packet
            let packet = Packet::literal(
                version,
                ty,
                literal
            );
            Ok((packet, p))
        }
        PacketType::Operator(_) => {
            // Parse Length
            let str_length = &s[6..];
            let length = str_length.parse::<PacketLength>()?;
            // println!("LENGTH: {:?}", length);
            // Bit Length or number of sub-packets
            let mut sub_packets = Vec::new();
            match length {
                PacketLength::Bits(n) => {
                    p += 16;
                    while p < (n as usize + 22) {
                        let str_sub_packets = &s[p..];
                        let (sub_packet, read)  = parse_packet(str_sub_packets)?;
                        sub_packets.push(sub_packet);
                        p += read;
                    }

                }
                PacketLength::SubPackets(n) => {
                    p += 12;
                    for _ in 0..n {
                        let str_sub_packets = &s[p..];
                        let (sub_packet, read)  = parse_packet(str_sub_packets)?;
                        sub_packets.push(sub_packet);
                        p += read;
                    }
                }
                _ => (),
            }
            let packet = Packet::operation(version, ty, length, sub_packets);
            Ok((packet, p))
        }
    }
}

pub fn sum_versions(p: &Packet) -> u64 {
    p.version + p.sub_packets.iter().fold(0, |acc, sp| acc + sum_versions(sp))
}

pub fn evaluate_packet(p: &Packet) -> u64 {
    match p.ty {
        PacketType::Operator(Operation::Sum) =>
            p.sub_packets.iter().fold(0, |sum, p| sum + evaluate_packet(p)),
        PacketType::Operator(Operation::Product) =>
            p.sub_packets.iter().fold(1, |prod, p| prod * evaluate_packet(p)),
        PacketType::Operator(Operation::Min) =>
            p.sub_packets.iter().fold(u64::MAX, |min, p| std::cmp::min(min, evaluate_packet(p))),
        PacketType::Operator(Operation::Max) =>
            p.sub_packets.iter().fold(0, |max, p| std::cmp::max(max, evaluate_packet(p))),
        PacketType::Operator(Operation::Less) =>
            if evaluate_packet(&p.sub_packets[0]) < evaluate_packet(&p.sub_packets[1]) { 1 } else { 0 }
        PacketType::Operator(Operation::Greater) =>
            if evaluate_packet(&p.sub_packets[0]) > evaluate_packet(&p.sub_packets[1]) { 1 } else { 0 }
        PacketType::Operator(Operation::Equal) =>
            if evaluate_packet(&p.sub_packets[0]) == evaluate_packet(&p.sub_packets[1]) { 1 } else { 0 }
        PacketType::Literal =>
            p.literal.unwrap(),
        _ => 0,
    }
}
// impl std::str::FromStr for Packet {
//     type Err = ParsePacketError;

//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         // Recursively parse the packets:
//         // LITERAL PACKET:
//         //  000 000 [00000]+
//         //  VVV TTT [NNNNN]+
//         //
//         // OPERATOR PACKET:
//         //   000 000 0 00000000000[0000] ...
//         //   VVV TTT I LLLLLLLLLLL[LLLL] ...

//         // Parse Version
//         let str_version = &s[0..3];
//         let version = if let Ok(v) = str_version.parse::<u64>() {
//             v
//         } else {
//             return Err(Self::Err::ParsePacketVersionError)
//         };

//         // Parse Type
//         let str_type = &s[3..6];
//         let ty = str_type.parse::<PacketType>()?;

//         // Different Approach for Operator or Literal Packets
//         match ty {
//             PacketType::Literal => {
//                 ()
//             }
//             PacketType::Operator(_) => {
//                 // Parse Length
//                 let str_length = &s[6..];
//                 let length = str_length.parse::<PacketLength>()?
//                 // Bit Length vs n-subpackets
//                 match length {
//                     PacketLength::Bits(n) => {
//                         let sub_packets = &s[22..(22 + n.into())];
//                         // How do we know where the next packets start/end?

//                     }
//                 }

//             }
//         }

//         Err(Self::Err::ParsePacketVersionError)
//     }
// }