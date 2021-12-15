use std::collections::HashMap;

type Instructions = HashMap<(char, char), ((char, char), (char, char))>;

fn parse_input(input: &str) -> (HashMap<(char, char), u128>, Instructions) {
    let mut starting_counts = HashMap::new();
    let start_code = input.lines().take(1).nth(0).expect("No start code");

    let mut prev_char = start_code.chars().nth(0).expect("No first char");
    for c in start_code.chars().skip(1) {
        let count = *starting_counts.get(&(prev_char, c)).unwrap_or(&0);
        starting_counts.insert((prev_char, c), count + 1);
        prev_char = c;
    }

    let mut instructions = HashMap::new();
    for line in input.lines().skip(2) {
        let mut line_pieces = line.split(" -> ");
        let mut pair = line_pieces.next().expect("No instruction pair").chars();
        let p1 = pair.next().expect("No first char");
        let p2 = pair.next().expect("No second char");
        let to_insert = line_pieces.next().expect("No instruction add part");
        let insert = to_insert
            .chars()
            .next()
            .expect("No instruction insert char");
        instructions.insert((p1, p2), ((p1, insert), (insert, p2)));
    }
    (starting_counts, instructions)
}

fn run_instructions(
    starting_counts: &HashMap<(char, char), u128>,
    instructions: &Instructions,
    iterations: u16,
) -> HashMap<(char, char), u128> {
    let mut pair_totals: HashMap<(char, char), u128> = starting_counts.clone();

    for _ in 0..iterations {
        let mut new_totals: HashMap<(char, char), u128> = HashMap::new();
        for (pair, count) in &pair_totals {
            match instructions.get(&pair) {
                None => {
                    new_totals.insert(*pair, *count);
                }
                Some((p1, p2)) => {
                    let current1 = *new_totals.get(p1).unwrap_or(&0);
                    new_totals.insert(*p1, current1 + count);
                    let current2 = *new_totals.get(p2).unwrap_or(&0);
                    new_totals.insert(*p2, current2 + count);
                }
            }
        }
        pair_totals = new_totals
    }

    pair_totals
}

fn count_chars(pair_counts: &HashMap<(char, char), u128>) -> HashMap<char, u128> {
    let mut counts: HashMap<char, u128> = HashMap::new();
    for ((p1, p2), count) in pair_counts {
        let count1 = *counts.get(&p1).unwrap_or(&0);
        counts.insert(*p1, count1 + count);
        let count2 = *counts.get(&p2).unwrap_or(&0);
        counts.insert(*p2, count2 + count);
    }
    counts
}

fn get_max_min_diff_for_iterations(
    starting_counts: &HashMap<(char, char), u128>,
    instructions: &Instructions,
    iterations: u16,
) -> u128 {
    let final_counts = run_instructions(&starting_counts, &instructions, iterations);
    let char_counts = count_chars(&final_counts);
    let mut min = *char_counts.values().min().expect("No min");
    if min % 2 == 1 {
        min = min + 1;
    }
    let mut max = *char_counts.values().max().expect("No max");
    if max % 2 == 1 {
        max = max + 1;
    }
    (max / 2) - (min / 2)
}

pub fn day14_part1(input: &str) -> String {
    let iterations = 10;
    let (starting_counts, instructions) = parse_input(input);
    let diff = get_max_min_diff_for_iterations(&starting_counts, &instructions, iterations);
    format!("{}", diff)
}

pub fn day14_part2(input: &str) -> String {
    let iterations = 40;
    let (starting_counts, instructions) = parse_input(input);
    let diff = get_max_min_diff_for_iterations(&starting_counts, &instructions, iterations);
    format!("{}", diff)
}
