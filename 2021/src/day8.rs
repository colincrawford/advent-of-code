use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};

fn parse_strings(s: &str) -> Vec<String> {
    s.split(' ').map(|s| String::from(s)).collect()
}

fn parse_input(input: &str) -> Vec<(Vec<String>, Vec<String>)> {
    input
        .lines()
        .map(|line| {
            let mut pieces = line.split(" | ");
            let digit_combos = parse_strings(pieces.next().expect("No digit combos in input"));
            let secret_codes = parse_strings(pieces.next().expect("No secret code in input"));
            (digit_combos, secret_codes)
        })
        .collect()
}

fn get_number_of_unique_segments(secret_codes: &Vec<String>) -> u32 {
    secret_codes
        .iter()
        .filter(|code| match code.len() {
            2 | 4 | 3 | 7 => true,
            _ => false,
        })
        .count() as u32
}

// the input chars are mixed, so we alphabetize them to make the strings comparable
fn alphabetize_chars(s: &str) -> String {
    let mut sorted_chars: Vec<char> = s.chars().collect();
    sorted_chars.sort_by(|a, b| b.cmp(a));
    String::from_iter(sorted_chars)
}

fn unique_chars(s: &str) -> HashSet<char> {
    s.chars().collect()
}

// We need the codes with len 5 to be last because we need the other
// length codes to narrow down wich of the len 5 codes match digits
fn compare_codes_for_sorting<'r, 's>(code1: &'r String, code2: &'s String) -> Ordering {
    if code1.len() == 5 {
        Ordering::Greater
    } else if code2.len() == 5 {
        Ordering::Less
    } else {
        code1.len().cmp(&code2.len())
    }
}

fn crack_codes(unique_digit_combos: &Vec<String>, secret_code: &Vec<String>) -> u32 {
    let mut code_to_digit = HashMap::new();
    let mut sorted_combos: Vec<String> = unique_digit_combos.iter().map(|s| s.clone()).collect();
    sorted_combos.sort_by(compare_codes_for_sorting);
    let combos_with_chars: Vec<(String, HashSet<char>)> = sorted_combos
        .iter()
        .map(|combo| (alphabetize_chars(combo), unique_chars(combo)))
        .collect();

    let mut four_chars: Option<&HashSet<char>> = None;
    let mut six_chars: Option<&HashSet<char>> = None;
    let mut seven_chars: Option<&HashSet<char>> = None;
    for (combo, chars) in combos_with_chars.iter() {
        match combo.len() {
            2 => {
                code_to_digit.insert(combo, '1');
            }
            3 => {
                code_to_digit.insert(combo, '7');
                seven_chars = Some(chars);
            }
            4 => {
                code_to_digit.insert(combo, '4');
                four_chars = Some(chars);
            }
            5 => {
                // 2, 3, or 5
                // 3 should contain 7's chars, the 2 / 5 will be missing 1
                if seven_chars.unwrap().difference(chars).count() == 0 {
                    code_to_digit.insert(combo, '3');
                // 6 contains 5, not 2
                } else if chars.difference(six_chars.unwrap()).count() == 0 {
                    code_to_digit.insert(combo, '5');
                } else {
                    code_to_digit.insert(combo, '2');
                }
            }
            6 => {
                // 0, 6, 9
                // 9 contains 4, 6 & 0 do not
                if four_chars.unwrap().difference(&chars).count() == 0 {
                    code_to_digit.insert(combo, '9');
                // 0 contains 7, 6 does not
                } else if seven_chars.unwrap().difference(&chars).count() == 0 {
                    code_to_digit.insert(combo, '0');
                } else {
                    code_to_digit.insert(combo, '6');
                    six_chars = Some(chars);
                }
            }
            7 => {
                code_to_digit.insert(combo, '8');
            }
            _ => panic!("Invalid code length {}", combo.len()),
        }
    }

    secret_code
        .iter()
        .fold(String::new(), |digits, code| {
            let digit = code_to_digit.get(&alphabetize_chars(code));
            format!("{}{}", digits, digit.expect("No digit for code"))
        })
        .parse()
        .expect("Invalid secret code digits")
}

pub fn day8_part1(input: &str) -> String {
    let inputs = parse_input(input);
    let number_of_unique_segments_in_secret_codes: u32 = inputs
        .iter()
        .map(|(_, secret_code)| get_number_of_unique_segments(secret_code))
        .sum();
    format!("{}", number_of_unique_segments_in_secret_codes)
}

pub fn day8_part2(input: &str) -> String {
    let inputs = parse_input(input);
    let secret_codes_sum: u32 = inputs
        .iter()
        .map(|(unique_combos, secret_code)| crack_codes(unique_combos, secret_code))
        .sum();
    format!("{}", secret_codes_sum)
}
