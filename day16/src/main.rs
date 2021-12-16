use std::{env, io, io::prelude::*, panic};

fn main() {
    let args: Vec<String> = env::args().collect();
    let part = args.get(1).cloned().unwrap_or(String::from("1"));

    let reader = io::stdin();
    let lines = reader.lock().lines();
    let values = lines.map(|x| x.unwrap());

    match part.as_str() {
        "1" => println!("{}", day16_part1(values)),
        "2" => println!("{}", day16_part2(values)),
        _ => println!("Invalid part {}", part),
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum TypeId {
    Sum = 0,
    Mul = 1,
    Min = 2,
    Max = 3,
    Lit = 4,
    Gt = 5,
    Lt = 6,
    Eq = 7,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Literal {
    ver: usize,
    val: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Operator {
    id: TypeId,
    ver: usize,
    subpackets: Vec<Packet>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Packet {
    Literal(Literal),
    Operator(Operator),
}

fn read_bin(chars: &mut impl Iterator<Item = char>, len: usize, read: &mut usize) -> usize {
    let bin = chars.take(len).collect::<String>();
    *read += len;
    usize::from_str_radix(&bin, 2).unwrap()
}

fn parse_packet(chars: &mut impl Iterator<Item = char>, read: &mut usize) -> Packet {
    let ver = read_bin(chars, 3, read);

    let id: TypeId = match read_bin(chars, 3, read) as u8 {
        0 => TypeId::Sum,
        1 => TypeId::Mul,
        2 => TypeId::Min,
        3 => TypeId::Max,
        4 => TypeId::Lit,
        5 => TypeId::Gt,
        6 => TypeId::Lt,
        7 => TypeId::Eq,
        _ => panic!("Invalid packet type"),
    };

    match id {
        // Literal value
        TypeId::Lit => {
            let mut val = 0;
            loop {
                let next = read_bin(chars, 5, read);

                val = (val << 4) | (next & 0b01111);
                if next & 0b10000 == 0 {
                    break;
                }
            }

            Packet::Literal(Literal { ver, val })
        }
        // Operator packet
        id => {
            let op = read_bin(chars, 1, read);

            match op {
                0 => {
                    let len = read_bin(chars, 15, read);

                    let init = *read;
                    let mut subpackets = vec![];
                    while *read - init < len {
                        subpackets.push(parse_packet(chars, read));
                    }

                    Packet::Operator(Operator {
                        id,
                        ver,
                        subpackets,
                    })
                }
                1 => {
                    let len = read_bin(chars, 11, read);

                    let mut subpackets = Vec::new();
                    for _ in 0..len {
                        subpackets.push(parse_packet(chars, read));
                    }

                    Packet::Operator(Operator {
                        id,
                        ver,
                        subpackets,
                    })
                }
                _ => panic!("Unknown operator type {}", op),
            }
        }
    }
}

fn collect_packets(root: Packet) -> Vec<Packet> {
    match root {
        Packet::Literal(_) => vec![root],
        Packet::Operator(op) => {
            let mut packets: Vec<Packet> = vec![Packet::Operator(op.clone())];
            for subpacket in op.subpackets {
                packets.append(&mut collect_packets(subpacket.clone()));
            }
            packets
        }
    }
}

fn to_bin_str(hex_str: &str) -> String {
    hex_str
        .chars()
        .map(|c| {
            let hex = c.to_digit(16).unwrap();
            format!("{:0>4b}", hex)
        })
        .collect::<Vec<_>>()
        .join("")
}

fn day16_part1(mut v: impl Iterator<Item = String>) -> usize {
    let line = v.next().unwrap();
    let bin_string = to_bin_str(&line);

    let mut chars = bin_string.chars();
    let mut read = 0;

    let root = parse_packet(&mut chars, &mut read);
    let packets = collect_packets(root);

    packets
        .iter()
        .map(|p| match p {
            Packet::Literal(lit) => lit.ver,
            Packet::Operator(op) => op.ver,
        })
        .sum()
}

#[test]
fn day16_part1_test() {
    let bin_1 = to_bin_str("D2FE28");
    assert_eq!("110100101111111000101000", bin_1);

    let mut chars_1 = bin_1.chars();
    let mut read_1 = 0;
    assert_eq!(
        Packet::Literal(Literal { ver: 6, val: 2021 }),
        parse_packet(&mut chars_1, &mut read_1)
    );

    let bin_2 = to_bin_str("38006F45291200");
    assert_eq!(
        "00111000000000000110111101000101001010010001001000000000",
        bin_2
    );

    assert_eq!(
        16,
        day16_part1(vec![String::from("8A004A801A8002F478")].into_iter())
    );
    assert_eq!(
        12,
        day16_part1(vec![String::from("620080001611562C8802118E34")].into_iter())
    );
    assert_eq!(
        23,
        day16_part1(vec![String::from("C0015000016115A2E0802F182340")].into_iter())
    );
    assert_eq!(
        31,
        day16_part1(vec![String::from("A0016C880162017C3686B18A3D4780")].into_iter())
    );
}

fn packet_value(packet: &Packet) -> usize {
    match packet {
        Packet::Literal(lit) => lit.val,
        Packet::Operator(op) => match op.id {
            TypeId::Sum => op.subpackets.iter().map(|x| packet_value(x)).sum(),
            TypeId::Mul => op.subpackets.iter().map(|x| packet_value(x)).product(),
            TypeId::Min => op.subpackets.iter().map(|x| packet_value(x)).min().unwrap(),
            TypeId::Max => op.subpackets.iter().map(|x| packet_value(x)).max().unwrap(),
            TypeId::Gt => {
                if packet_value(&op.subpackets[0]) > packet_value(&op.subpackets[1]) {
                    1
                } else {
                    0
                }
            }
            TypeId::Lt => {
                if packet_value(&op.subpackets[0]) < packet_value(&op.subpackets[1]) {
                    1
                } else {
                    0
                }
            }
            TypeId::Eq => {
                if packet_value(&op.subpackets[0]) == packet_value(&op.subpackets[1]) {
                    1
                } else {
                    0
                }
            }
            _ => panic!("Unknown operator type"),
        },
    }
}

fn day16_part2(mut v: impl Iterator<Item = String>) -> usize {
    let line = v.next().unwrap();
    let bin_string = to_bin_str(&line);

    let mut chars = bin_string.chars();
    let mut read = 0;

    let root = parse_packet(&mut chars, &mut read);
    packet_value(&root)
}

#[test]
fn day16_part2_test() {
    assert_eq!(3, day16_part2(vec![String::from("C200B40A82")].into_iter()));
    assert_eq!(
        54,
        day16_part2(vec![String::from("04005AC33890")].into_iter())
    );
    assert_eq!(
        7,
        day16_part2(vec![String::from("880086C3E88112")].into_iter())
    );
    assert_eq!(
        9,
        day16_part2(vec![String::from("CE00C43D881120")].into_iter())
    );

    assert_eq!(
        1,
        day16_part2(vec![String::from("D8005AC2A8F0")].into_iter())
    );

    assert_eq!(0, day16_part2(vec![String::from("F600BC2D8F")].into_iter()));

    assert_eq!(
        0,
        day16_part2(vec![String::from("9C005AC2F8F0")].into_iter())
    );

    assert_eq!(
        1,
        day16_part2(vec![String::from("9C0141080250320F1802104A08")].into_iter())
    );
}
