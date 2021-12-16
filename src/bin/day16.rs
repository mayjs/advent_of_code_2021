use anyhow::Result;
use aoc2021::stream_items_from_file;
use itertools::Itertools;
use std::{convert, path::Path};

fn parse_hex_repr(input: &str) -> Vec<bool> {
    input
        .chars()
        .map(|hex| match hex {
            '0' => [false, false, false, false],
            '1' => [false, false, false, true],
            '2' => [false, false, true, false],
            '3' => [false, false, true, true],
            '4' => [false, true, false, false],
            '5' => [false, true, false, true],
            '6' => [false, true, true, false],
            '7' => [false, true, true, true],
            '8' => [true, false, false, false],
            '9' => [true, false, false, true],
            'A' => [true, false, true, false],
            'B' => [true, false, true, true],
            'C' => [true, true, false, false],
            'D' => [true, true, false, true],
            'E' => [true, true, true, false],
            'F' => [true, true, true, true],
            _ => panic!("Invalid input: {}", hex),
        })
        .fold(Vec::with_capacity(input.len() * 4), |mut acc, v| {
            acc.extend(v.iter());
            acc
        })
}

fn read_bit_triple(input: &mut impl Iterator<Item = bool>) -> Option<[bool; 3]> {
    let tuple = input.next_tuple();
    tuple.map(|(v1, v2, v3)| [v1, v2, v3])
}

fn read_bit_quintuple(input: &mut impl Iterator<Item = bool>) -> Option<[bool; 5]> {
    let tuple = input.next_tuple();
    tuple.map(|(v1, v2, v3, v4, v5)| [v1, v2, v3, v4, v5])
}

fn read_n_bits(input: &mut impl Iterator<Item = bool>, n: usize) -> Option<Vec<bool>> {
    (0..n).map(|_| input.next()).collect()
}

fn convert_literal(input: &[bool]) -> u64 {
    input
        .iter()
        .rev()
        .fold((1, 0), |(exp, sum), &bit| {
            (exp * 2, if bit { sum + exp } else { sum })
        })
        .1
}

#[derive(Debug)]
struct Header {
    version: u64,
    typ: u64,
}

fn parse_header(input: &mut impl Iterator<Item = bool>) -> Option<(usize, Header)> {
    read_bit_triple(input).and_then(|version| {
        read_bit_triple(input).map(|typ| {
            (
                6,
                Header {
                    version: convert_literal(&version),
                    typ: convert_literal(&typ),
                },
            )
        })
    })
}

#[derive(Debug)]
enum PacketContents {
    Literal(u64),
    Operator(u64, Vec<Packet>),
}

#[derive(Debug)]
struct Packet {
    version: u64,
    contents: PacketContents,
}

fn parse_packet(input: &mut impl Iterator<Item = bool>) -> Option<(usize, Packet)> {
    parse_header(input).and_then(|(header_len, header)| {
        let packet = match header.typ {
            4 => {
                let mut full_bits = Vec::new();
                loop {
                    let bits = read_bit_quintuple(input)?;
                    full_bits.extend_from_slice(&bits[1..]);
                    if !bits[0] {
                        break;
                    }
                }
                Some((
                    full_bits.len() + full_bits.len() / 4 + header_len,
                    PacketContents::Literal(convert_literal(&full_bits)),
                ))
            }
            _ => {
                let mut children = Vec::new();
                let length_type_id = input.next()?;
                let mut read_bits = 0;
                if !length_type_id {
                    // Length type ID is 0, so we get 15 bits for the number of sub-packets
                    let total_subpacket_bits = convert_literal(&read_n_bits(input, 15)?) as usize;
                    while read_bits < total_subpacket_bits {
                        // println!("Expecting {}", total_subpacket_bits - read_bits);
                        let (subpacket_bits, packet) = parse_packet(input)?;
                        children.push(packet);
                        read_bits += subpacket_bits;
                    }
                    read_bits += 15;
                } else {
                    // Length type ID is 1, so we get 11 bits for the number of bits in the sub packets
                    let total_subpackets = convert_literal(&read_n_bits(input, 11)?);
                    for _ in 0..total_subpackets {
                        let (subpacket_bits, packet) = parse_packet(input)?;
                        children.push(packet);
                        read_bits += subpacket_bits;
                    }
                    read_bits += 11;
                }
                Some((
                    read_bits + 1 + header_len,
                    PacketContents::Operator(header.typ, children),
                ))
            }
        }
        .map(|(len, contents)| {
            (
                len,
                Packet {
                    version: header.version,
                    contents,
                },
            )
        });
        // println!("Intermediate result: {:?}", packet);
        packet
    })
}

fn sum_versions(packet: Packet) -> u64 {
    let mut sum = 0;
    let mut stack = Vec::new();
    stack.push(packet);

    while let Some(packet) = stack.pop() {
        sum += packet.version;
        match packet.contents {
            PacketContents::Literal(_) => (),
            PacketContents::Operator(_, mut children) => stack.append(&mut children),
        }
    }
    sum
}

impl Packet {
    fn evaluate(&self) -> u64 {
        match &self.contents {
            PacketContents::Literal(v) => *v,
            PacketContents::Operator(op, children) => {
                let mut child_values = children.iter().map(Packet::evaluate);
                match op {
                    0 => child_values.sum(),
                    1 => child_values.product(),
                    2 => child_values.min().unwrap(),
                    3 => child_values.max().unwrap(),
                    5 | 6 | 7 => {
                        let first = child_values.next().unwrap();
                        let second = child_values.next().unwrap();
                        match op {
                            5 => {
                                if first > second {
                                    1
                                } else {
                                    0
                                }
                            }
                            6 => {
                                if first < second {
                                    1
                                } else {
                                    0
                                }
                            }
                            7 => {
                                if first == second {
                                    1
                                } else {
                                    0
                                }
                            }
                            _ => panic!("Should never get here"),
                        }
                    }
                    _ => panic!("Unexpected op: {}", op),
                }
            }
        }
    }
}

fn part1<P: AsRef<Path>>(input: P) -> Result<u64> {
    let hex: String = stream_items_from_file(input)?.next().unwrap();
    let bin = parse_hex_repr(&hex);
    let mut iter = bin.into_iter();
    let packet = parse_packet(&mut iter).unwrap();
    Ok(sum_versions(packet.1))
}

fn part2<P: AsRef<Path>>(input: P) -> Result<u64> {
    let hex: String = stream_items_from_file(input)?.next().unwrap();
    let bin = parse_hex_repr(&hex);
    let mut iter = bin.into_iter();
    let packet = parse_packet(&mut iter).unwrap();
    Ok(packet.1.evaluate())
}

const INPUT: &str = "input/day16.txt";

fn main() -> Result<()> {
    println!("Answer for part 1: {}", part1(INPUT)?);
    println!("Answer for part 2: {}", part2(INPUT)?);
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use aoc2021::test_helpers::create_line_file;
    use tempfile::TempDir;

    use super::*;

    fn example_file1() -> (TempDir, impl AsRef<Path>) {
        create_line_file(["8A004A801A8002F478"].iter(), None)
    }

    fn example_file2() -> (TempDir, impl AsRef<Path>) {
        create_line_file(["620080001611562C8802118E34"].iter(), None)
    }

    fn example_file3() -> (TempDir, impl AsRef<Path>) {
        create_line_file(["C0015000016115A2E0802F182340"].iter(), None)
    }

    fn example_file4() -> (TempDir, impl AsRef<Path>) {
        create_line_file(["A0016C880162017C3686B18A3D4780"].iter(), None)
    }

    fn example_file5() -> (TempDir, impl AsRef<Path>) {
        create_line_file(["C200B40A82"].iter(), None)
    }

    #[test]
    fn test_part1() {
        let (dir, file) = example_file1();
        assert_eq!(part1(file).unwrap(), 16);
        drop(dir);
        let (dir, file) = example_file2();
        assert_eq!(part1(file).unwrap(), 12);
        drop(dir);
        let (dir, file) = example_file3();
        assert_eq!(part1(file).unwrap(), 23);
        drop(dir);
        let (dir, file) = example_file4();
        assert_eq!(part1(file).unwrap(), 31);
        drop(dir);
    }

    #[test]
    fn test_part2() {
        let (dir, file) = example_file5();
        assert_eq!(part2(file).unwrap(), 3);
        drop(dir);
    }
}
