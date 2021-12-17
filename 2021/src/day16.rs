enum Packet {
    LiteralValue(u64, Vec<char>),
    Operation(u64, u64, Vec<Packet>),
}

impl Packet {
    fn versions_sum(&self) -> u64 {
        match self {
            Packet::LiteralValue(version, _) => *version,
            Packet::Operation(version, _, packets) => {
                *version + packets.iter().map(|p| p.versions_sum()).sum::<u64>()
            }
        }
    }

    fn nth_sub_packet(&self, n: usize) -> u128 {
        match self {
            Packet::Operation(_, _, packets) => {
                packets.iter().nth(n).expect("No nth packet").value()
            }
            _ => panic!("No sub packets for Literal packet"),
        }
    }

    fn value(&self) -> u128 {
        match self {
            Packet::LiteralValue(_, bits) => bit_str_to_number(bits) as u128,
            Packet::Operation(_, type_id, packets) => match type_id {
                0 => packets.iter().map(|p| p.value()).sum::<u128>(),
                1 => packets.iter().fold(1, |acc, p| acc * (p.value() as u128)),
                2 => packets.iter().map(|p| p.value()).min().expect("No min"),
                3 => packets.iter().map(|p| p.value()).max().expect("No max"),
                5 => {
                    if self.nth_sub_packet(0) > self.nth_sub_packet(1) {
                        1
                    } else {
                        0
                    }
                }
                6 => {
                    if self.nth_sub_packet(0) < self.nth_sub_packet(1) {
                        1
                    } else {
                        0
                    }
                }
                7 => {
                    if self.nth_sub_packet(0) == self.nth_sub_packet(1) {
                        1
                    } else {
                        0
                    }
                }
                _ => panic!("Invalid operation type id {}", type_id),
            },
        }
    }
}

fn to_bin_str(c: char) -> Vec<char> {
    match c {
        '0' => vec!['0', '0', '0', '0'],
        '1' => vec!['0', '0', '0', '1'],
        '2' => vec!['0', '0', '1', '0'],
        '3' => vec!['0', '0', '1', '1'],
        '4' => vec!['0', '1', '0', '0'],
        '5' => vec!['0', '1', '0', '1'],
        '6' => vec!['0', '1', '1', '0'],
        '7' => vec!['0', '1', '1', '1'],
        '8' => vec!['1', '0', '0', '0'],
        '9' => vec!['1', '0', '0', '1'],
        'A' => vec!['1', '0', '1', '0'],
        'B' => vec!['1', '0', '1', '1'],
        'C' => vec!['1', '1', '0', '0'],
        'D' => vec!['1', '1', '0', '1'],
        'E' => vec!['1', '1', '1', '0'],
        'F' => vec!['1', '1', '1', '1'],
        _ => panic!("Invalid input char {}", c),
    }
}

fn parse_input(input: &str) -> Vec<char> {
    input
        .lines()
        .take(1)
        .nth(0)
        .expect("No input line")
        .chars()
        .flat_map(to_bin_str)
        .collect()
}

fn bit_str_to_number(bit_str: &Vec<char>) -> u64 {
    let bits: String = bit_str.iter().collect();
    isize::from_str_radix(&bits, 2).unwrap() as u64
}

fn read_literal(bit_str: &Vec<char>, start_inx: &mut usize, version: u64) -> Packet {
    let mut nums = vec![];
    loop {
        let leading_bit = bit_str[*start_inx];
        *start_inx += 1;
        for n in bit_str[*start_inx..(*start_inx + 4)].to_vec() {
            nums.push(n);
        }
        *start_inx += 4;
        if leading_bit == '0' {
            break;
        }
    }
    Packet::LiteralValue(version, nums)
}

fn read_operation(
    bit_str: &Vec<char>,
    start_inx: &mut usize,
    version: u64,
    type_id: u64,
) -> Packet {
    let length_type_id = bit_str[*start_inx];
    *start_inx += 1;
    let mut packets = vec![];
    if length_type_id == '0' {
        let num_bits_of_sub_packets =
            bit_str_to_number(&bit_str[*start_inx..(*start_inx + 15)].to_vec());
        *start_inx += 15;
        let started_at = *start_inx;
        while *start_inx < (started_at + num_bits_of_sub_packets as usize) {
            packets.push(parse_packet(bit_str, start_inx));
        }
    } else {
        let num_sub_packets = bit_str_to_number(&bit_str[*start_inx..(*start_inx + 11)].to_vec());
        *start_inx += 11;
        for _ in 0..num_sub_packets {
            packets.push(parse_packet(bit_str, start_inx));
        }
    }
    Packet::Operation(version, type_id, packets)
}

fn parse_packet(bit_str: &Vec<char>, start_inx: &mut usize) -> Packet {
    let version = bit_str_to_number(&bit_str[*start_inx..(*start_inx + 3)].to_vec());
    let type_id = bit_str_to_number(&bit_str[(*start_inx + 3)..(*start_inx + 6)].to_vec());
    *start_inx += 6;
    match type_id {
        4 => read_literal(bit_str, start_inx, version),
        _ => read_operation(bit_str, start_inx, version, type_id),
    }
}

fn parse_msg(msg_bits: &Vec<char>) -> Packet {
    let mut start_inx = 0;
    parse_packet(msg_bits, &mut start_inx)
}

pub fn day16_part1(input: &str) -> String {
    let bin_str = parse_input(input);
    let packet = parse_msg(&bin_str);
    format!("{:?}", packet.versions_sum())
}

pub fn day16_part2(input: &str) -> String {
    let bin_str = parse_input(input);
    let packet = parse_msg(&bin_str);
    format!("{:?}", packet.value())
}
