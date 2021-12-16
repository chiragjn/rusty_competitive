use std::{
    io::{self, BufRead, Stdin},
    num::ParseIntError,
};

struct InputUtils {
    stream: Stdin,
    buffer: String,
}

impl Default for InputUtils {
    fn default() -> Self {
        return Self {
            stream: io::stdin(),
            buffer: String::new(),
        };
    }
}

impl Iterator for InputUtils {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        match self.stream.lock().lines().next() {
            Some(line) => Some(line.unwrap().trim().to_string()),
            None => None,
        }
    }
}

#[derive(Debug)]
enum PacketType {
    OPERATOR(u64),
    LITERAL(u64),
}

fn to_value(bits: &[char]) -> Result<u64, ParseIntError> {
    return u64::from_str_radix(&bits.iter().collect::<String>(), 2);
}

#[derive(Debug)]
struct Packet<'a> {
    bits: &'a [char],
    version: u64,
    packet_type: PacketType,
    subpackets: Vec<Packet<'a>>,
}

impl<'a> Packet<'a> {
    fn version_sum(&self) -> u64 {
        return self.version + self.subpackets.iter().map(|p| p.version_sum()).sum::<u64>();
    }
}

fn to_packets<'a>(bits: &'a [char], num_packets: Option<usize>) -> (Vec<Packet<'a>>, usize) {
    let mut ptr = 0;
    let mut packets = vec![];
    match num_packets {
        Some(n) => {
            for _ in 0..n {
                let (packet, _bits_read) = to_packet(&bits[ptr..]);
                packets.push(packet);
                ptr += _bits_read;
            }
        }
        None => {
            while ptr < bits.len() {
                let (packet, _bits_read) = to_packet(&bits[ptr..]);
                packets.push(packet);
                ptr += _bits_read;
            }
            if ptr != bits.len() {
                unreachable!("Invalid bits sequence, found extra bits!");
            }
        }
    }
    return (packets, ptr);
}

fn to_packet<'a>(bits: &'a [char]) -> (Packet<'a>, usize) {
    let version = to_value(&bits[0..3]).unwrap();
    let packet_type = to_value(&bits[3..6]).unwrap();
    match packet_type {
        4 => {
            let mut literal_bits: Vec<char> = vec![];
            for ptr in (6..bits.len()).step_by(5) {
                match bits[ptr] {
                    '1' => {
                        literal_bits.extend(bits[ptr + 1..ptr + 5].iter());
                    }
                    '0' => {
                        literal_bits.extend(bits[ptr + 1..ptr + 5].iter());
                        let literal = to_value(&literal_bits[..]).unwrap();
                        let packet = Packet {
                            bits: &bits[..ptr + 5],
                            version,
                            packet_type: PacketType::LITERAL(literal),
                            subpackets: vec![],
                        };
                        return (packet, ptr + 5);
                    }
                    _ => {
                        unreachable!("invalid char, should be a bit 0/1");
                    }
                }
            }
            unreachable!("Invalid bit sequence! no last packet for PacketType::LITERAL");
        }
        o => match bits[6] {
            '0' => {
                let num_bits: usize = to_value(&bits[7..22]).unwrap() as _;
                let (subpackets, _bits_read) = to_packets(&bits[22..(22 + num_bits)], None);
                let packet = Packet {
                    bits: &bits[..22 + _bits_read],
                    version,
                    packet_type: PacketType::OPERATOR(o),
                    subpackets,
                };
                return (packet, 22 + _bits_read);
            }
            '1' => {
                let num_packets: usize = to_value(&bits[7..18]).unwrap() as _;
                let (subpackets, _bits_read) = to_packets(&bits[18..], Some(num_packets));
                let packet = Packet {
                    bits: &bits[..18 + _bits_read],
                    version,
                    packet_type: PacketType::OPERATOR(o),
                    subpackets,
                };
                return (packet, 18 + _bits_read);
            }
            _ => {
                unreachable!("invalid char, should be a bit 0/1");
            }
        },
    }
}

fn solve(mut lines: Box<dyn Iterator<Item = String>>) -> u64 {
    let packet_hex = lines.next().unwrap();
    let mut packet: Vec<char> = vec![];
    for c in packet_hex.chars() {
        packet.extend(
            format!(
                "{:04b}",
                c.to_digit(16).expect("failed to parse as a hex digit")
            )
            .chars(),
        );
    }
    let (packet, _) = to_packet(&packet[..]);
    return packet.version_sum();
}

fn main() {
    let iu = InputUtils::default();
    let boxed_iter = Box::new(iu);
    println!("{}", solve(boxed_iter));
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test() {
        let test_inputs = [
            (r#"8A004A801A8002F478"#, 16),
            (r#"620080001611562C8802118E34"#, 12),
            (r#"C0015000016115A2E0802F182340"#, 23),
            (r#"A0016C880162017C3686B18A3D4780"#, 31),
        ];
        for (test_input, answer) in test_inputs {
            let it = test_input
                .split('\n')
                .into_iter()
                .map(|part| part.to_string());
            assert_eq!(solve(Box::new(it)), answer);
        }
    }
}
