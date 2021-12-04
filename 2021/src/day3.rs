type InputNumber = u16;

// Our input is a list of binary strings, we'll parse those strings and convert them to numbers
fn parse_input(input: &str) -> Vec<InputNumber> {
    input
        .lines()
        .map(|line| isize::from_str_radix(line, 2).unwrap() as InputNumber)
        .collect()
}

fn get_most_common_bit(bit_seqs: &Vec<InputNumber>, bit_seq_len: usize, inx: usize) -> u8 {
    let mut zeros = 0;
    let mut ones = 0;
    let compare = 1 << (bit_seq_len - inx - 1);

    for number in bit_seqs {
        if (number & compare) > 0 {
            ones += 1
        } else {
            zeros += 1
        }
    }

    if zeros == ones || ones > zeros {
        1
    } else {
        0
    }
}

fn get_power_consumption(bit_seqs: &Vec<InputNumber>, bit_seq_len: usize) -> u32 {
    let mut gamma = 0;
    let mut epsilon = 0;
    for i in 0..bit_seq_len {
        let most_common_bit = get_most_common_bit(bit_seqs, bit_seq_len, i as usize);
        gamma = gamma << 1;
        epsilon = epsilon << 1;
        if most_common_bit == 1 {
            gamma += 1
        } else {
            epsilon += 1
        }
    }
    gamma * epsilon
}

pub fn day3_part1(input: &str) -> String {
    let bit_seq_len = input.lines().next().expect("No input lines").len();
    let bit_seqs = parse_input(input);
    format!("{}", get_power_consumption(&bit_seqs, bit_seq_len))
}

fn get_rating(bit_seqs: &Vec<InputNumber>, bit_seq_len: usize, take_most_common: bool) -> u32 {
    let mut valid_seqs = bit_seqs.clone();
    for i in 0..bit_seq_len {
        let most_common_bit = get_most_common_bit(&valid_seqs, bit_seq_len, i);
        let one_bit_at_inx = 1 << (bit_seq_len - i - 1);
        valid_seqs = valid_seqs
            .into_iter()
            .filter(|number| {
                if (take_most_common && most_common_bit == 1)
                    || (!take_most_common && most_common_bit == 0)
                {
                    (number & one_bit_at_inx) > 0
                } else {
                    (number & one_bit_at_inx) == 0
                }
            })
            .collect();
        if valid_seqs.len() == 1 {
            break;
        }
    }
    valid_seqs[0] as u32
}

pub fn day3_part2(input: &str) -> String {
    let bit_seq_len = input.lines().next().expect("No input lines").len();
    let bit_seqs = parse_input(input);
    let oxygen_generator_rating = get_rating(&bit_seqs, bit_seq_len, true);
    let co2_scrubber_rating = get_rating(&bit_seqs, bit_seq_len, false);
    format!("{}", oxygen_generator_rating * co2_scrubber_rating)
}
