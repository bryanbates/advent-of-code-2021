use std::cmp::{max, min};
use std::collections::VecDeque;

fn to_bits(ch: u8) -> [u8; 4] {
    // println!("Decoding {:?}", ch);
    match ch {
        b'0' => [0, 0, 0, 0],
        b'1' => [0, 0, 0, 1],
        b'2' => [0, 0, 1, 0],
        b'3' => [0, 0, 1, 1],
        b'4' => [0, 1, 0, 0],
        b'5' => [0, 1, 0, 1],
        b'6' => [0, 1, 1, 0],
        b'7' => [0, 1, 1, 1],
        b'8' => [1, 0, 0, 0],
        b'9' => [1, 0, 0, 1],
        b'A' => [1, 0, 1, 0],
        b'B' => [1, 0, 1, 1],
        b'C' => [1, 1, 0, 0],
        b'D' => [1, 1, 0, 1],
        b'E' => [1, 1, 1, 0],
        b'F' => [1, 1, 1, 1],
        _ => unreachable!(),
    }
}

type Header = (u8, u8);

#[derive(Debug, Default)]
struct Packet {
    version: u8,
    type_id: u8,
    value: u64, // Either literal value or length
    sub_packets: Option<Vec<Packet>>,
}

fn read_bits(bits: &mut VecDeque<u8>, n: usize) -> u64 {
    let mut val: u64 = 0;
    for i in 1..=n {
        val += (bits.pop_front().unwrap() as u64) << (n - i);
    }
    val
}

fn read_header(bits: &mut VecDeque<u8>) -> Header {
    let version = read_bits(bits, 3) as u8;
    let type_id = read_bits(bits, 3) as u8;

    (version, type_id)
}

fn read_all_packets(bits: &mut VecDeque<u8>) -> Vec<Packet> {
    let mut packets: Vec<Packet> = Vec::new();

    while bits.len() >= 11 {
        packets.push(read_packet(bits));
    }

    packets
}

fn read_packet(bits: &mut VecDeque<u8>) -> Packet {
    let header = read_header(bits);

    // println!("HEADER: {:?}", header);

    match header {
        (_, 4) => {
            // Literal value, start reading chunks
            let mut chunks: Vec<u8> = Vec::new();
            loop {
                // println!("bits: {:?}", bits);
                let chunk = read_bits(bits, 5);
                chunks.push((chunk & 0xF_u64) as u8);
                if chunk & 0x10 == 0 {
                    break;
                }
            }
            let mut val: u64 = 0;
            let mut mult: u64 = 0;
            while let Some(chunk) = chunks.pop() {
                val += (chunk as u64) << (mult * 4);
                mult += 1;
            }

            // println!("LITERAL: {:?}", val);

            Packet {
                version: header.0,
                type_id: header.1,
                value: val,
                sub_packets: None,
            }
        }
        (_, _) => {
            // Operator of some sort
            let length_type_id = bits.pop_front().unwrap();
            // println!("LEN_TYPE: {}", length_type_id);
            match length_type_id {
                0 => {
                    // println!("bits: {:?}", bits);
                    // Length in bits
                    let lenbits = read_bits(bits, 15);

                    // println!("OP-0: {:?}", lenbits);

                    let mut rem = bits.split_off(lenbits as usize);
                    std::mem::swap(bits, &mut rem);

                    let mut subs: Vec<Packet> = Vec::new();

                    while rem.len() >= 11 {
                        // println!("BITS: {:?}", rem);
                        // println!("Reading sub packet");
                        let packet = read_packet(&mut rem);
                        subs.push(packet);
                    }

                    Packet {
                        version: header.0,
                        type_id: header.1,
                        value: lenbits,
                        sub_packets: Some(subs),
                    }
                }
                1 => {
                    let npackets = read_bits(bits, 11);

                    // println!("OP-1: {:?}", npackets);

                    let mut subs: Vec<Packet> = Vec::new();

                    for p in 0..npackets {
                        // println!("BITS: {:?}", bits);
                        // println!("Reading sub packet {} of {}", p, npackets);

                        let packet = read_packet(bits);
                        subs.push(packet);
                    }

                    Packet {
                        version: header.0,
                        type_id: header.1,
                        value: npackets,
                        sub_packets: Some(subs),
                    }
                }
                _ => unreachable!(),
            }
        }
    }
}

fn sum_versions(p: &Packet) -> u64 {
    let mut sum = p.version as u64;

    if let Some(subs) = &p.sub_packets {
        for sub in subs {
            sum += sum_versions(sub);
        }
    }

    sum
}

fn interpret(p: &Packet) -> u64 {
    if let Some(subs) = &p.sub_packets {
        let mut subvals = subs.iter().map(interpret);

        match p.type_id {
            0 => subvals.sum(),
            1 => subvals.product(),
            2 => subvals.min().unwrap(),
            3 => subvals.max().unwrap(),
            4 => p.value, // Shouldn't be here, but here we are...
            5 => {
                if subvals.next().unwrap() > subvals.next().unwrap() {
                    1
                } else {
                    0
                }
            }
            6 => {
                if subvals.next().unwrap() < subvals.next().unwrap() {
                    1
                } else {
                    0
                }
            }
            7 => {
                if subvals.next().unwrap() == subvals.next().unwrap() {
                    1
                } else {
                    0
                }
            }
            _ => unreachable!(),
        }
    } else {
        assert_eq!(p.type_id, 4);
        p.value
    }
}

#[aoc(day16, part1)]
pub fn part1(input: &str) -> u64 {
    // println!("INPUT: {:?}", input);
    let mut bits = input
        .trim()
        .bytes()
        .flat_map(to_bits)
        .collect::<VecDeque<_>>();
    // println!("BITS: {:?}", bits);
    assert_eq!(input.trim().len() * 4, bits.len());
    let packets = read_all_packets(&mut bits);

    // println!("PACKETS: {:?}", packets);
    packets.iter().map(sum_versions).sum()
}

#[aoc(day16, part2)]
pub fn part2(input: &str) -> u64 {
    let mut bits = input
        .trim()
        .bytes()
        .flat_map(to_bits)
        .collect::<VecDeque<_>>();
    let packets = read_all_packets(&mut bits);

    assert_eq!(packets.len(), 1);

    interpret(&packets[0])
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = r#"38006F45291200
"#;
    const EXAMPLE_2: &str = r#"EE00D40C823060
"#;
    const EXAMPLE_3: &str = r#"8A004A801A8002F478
"#;
    const EXAMPLE_4: &str = r#"620080001611562C8802118E34
"#;
    const EXAMPLE_5: &str = r#"C0015000016115A2E0802F182340
"#;
    const EXAMPLE_6: &str = r#"A0016C880162017C3686B18A3D4780
"#;

    #[test]
    fn part1_ex1() {
        assert_eq!(part1(EXAMPLE_1), 1 + 6 + 2);
    }

    #[test]
    fn part1_ex0() {
        assert_eq!(part1("D2FE28"), 6);
    }

    #[test]
    fn part1_ex2() {
        assert_eq!(part1(EXAMPLE_2), 7 + 2 + 4 + 1);
    }

    #[test]
    fn part1_ex3() {
        assert_eq!(part1(EXAMPLE_3), 16);
    }

    #[test]
    fn part1_ex4() {
        assert_eq!(part1(EXAMPLE_4), 12);
    }

    #[test]
    fn part1_ex5() {
        assert_eq!(part1(EXAMPLE_5), 23);
    }

    #[test]
    fn part1_ex6() {
        assert_eq!(part1(EXAMPLE_6), 31);
    }

    #[test]
    fn part2_ex1() {
        assert_eq!(part2("C200B40A82"), 3);
        assert_eq!(part2("04005AC33890"), 54);
        assert_eq!(part2("880086C3E88112"), 7);
        assert_eq!(part2("CE00C43D881120"), 9);
        assert_eq!(part2("D8005AC2A8F0"), 1);
        assert_eq!(part2("F600BC2D8F"), 0);
        assert_eq!(part2("9C005AC2F8F0"), 0);
        assert_eq!(part2("9C0141080250320F1802104A08"), 1);
    }
}
