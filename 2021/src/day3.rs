#[derive(PartialEq)]
enum MostCommonBit {
    Zero,
    One,
    Tied,
}

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn get_most_common_bit(bit_seqs: &Vec<Vec<char>>, inx: usize) -> MostCommonBit {
    let mut zeros = 0;
    let mut ones = 0;
    for bit_seq in bit_seqs {
        match bit_seq[inx] {
            '0' => zeros += 1,
            '1' => ones += 1,
            _ => panic!("Invalid bit char {}", bit_seq[inx])
        }
    }
    if zeros == ones { MostCommonBit::Tied }
    else if ones > zeros { MostCommonBit::One }
    else { MostCommonBit::Zero }
}

fn get_most_common_bits(bit_seqs: Vec<Vec<char>>) -> Vec<MostCommonBit> {
    bit_seqs[0].iter().enumerate().map(|(inx, _)| get_most_common_bit(&bit_seqs, inx)).collect()
}

fn bits_to_int(bits: &Vec<char>) -> u32 {
    let mut num = 0;
    for bit in bits.iter() {
        num = num << 1;
        num += if *bit == '1' { 1 } else { 0 }
    }
    num
}

fn get_rates(most_common_bits: Vec<MostCommonBit>) -> (u32, u32) {
    let mut gamma = vec![];
    let mut epsilon = vec![];
    for bit in most_common_bits.iter() {
        match bit {
            MostCommonBit::Zero => {
                gamma.push('0');
                epsilon.push('1')
            },
            MostCommonBit::One | MostCommonBit::Tied => {
                gamma.push('1');
                epsilon.push('0')
            },
        }
    }
    (bits_to_int(&gamma), bits_to_int(&epsilon))
}

pub fn day3_part1(input: &str) -> String {
    let bit_seqs = parse_input(input);
    let most_common_bits = get_most_common_bits(bit_seqs);
    let (gamma, epsilon) = get_rates(most_common_bits);
    format!("{}", gamma * epsilon)
}

fn get_rating(bit_seqs: &Vec<Vec<char>>, get_char_to_match: impl Fn(MostCommonBit) -> char) -> u32 {
    let mut valid_seqs: Vec<Vec<char>> = bit_seqs.iter().map(|bits| bits.clone()).collect();
    let mut inx = 0;
    while valid_seqs.len() > 1 {
        let bit_to_match = get_char_to_match(get_most_common_bit(&valid_seqs, inx));
        valid_seqs = valid_seqs.iter()
            .filter(|seq| seq[inx] == bit_to_match)
            .map(|seq| seq.clone())
            .collect();
        inx += 1;
    }
    bits_to_int(&valid_seqs[0])
}

pub fn day3_part2(input: &str) -> String {
    let bit_seqs = parse_input(input);
    let oxygen_generator_rating = get_rating(&bit_seqs, |most_common_bit| {
        match most_common_bit { MostCommonBit::Zero => '0', _ => '1' }
    });
    let co2_scrubber_rating = get_rating(&bit_seqs, |most_common_bit| {
        match most_common_bit { MostCommonBit::Zero => '1', _ => '0' }
    });
    format!("{}", oxygen_generator_rating * co2_scrubber_rating)
}
