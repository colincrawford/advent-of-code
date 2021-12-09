type CrabHorizontalPosition = u16;

fn parse_input(input: &str) -> Vec<CrabHorizontalPosition> {
    input
        .lines()
        .next()
        .expect("No input lines")
        .split(',')
        .map(|pos| pos.parse().expect("Invalid input position"))
        .collect()
}

fn get_least_fuel_position(
    starting_positions: &Vec<CrabHorizontalPosition>,
    get_move_cost: impl Fn(u32) -> u32,
) -> u32 {
    let min = *starting_positions
        .iter()
        .min()
        .expect("No min starting position");
    let max = *starting_positions
        .iter()
        .max()
        .expect("No max starting position");

    // start with the cost of moving everything to the 0 position
    let mut best_seen: u32 = starting_positions.iter().map(|v| get_cost(*v as u32)).sum();
    let mut running_diff: u32 = 0;
    // evaluate the cost of moving everything to each position between min and max
    for pos in min..=max {
        for &starting_pos in starting_positions.iter() {
            let move_distance = (pos as i32 - starting_pos as i32).abs() as u32;
            running_diff += get_move_cost(move_distance);
        }

        if running_diff < best_seen {
            best_seen = running_diff;
        }

        running_diff = 0;
    }
    best_seen
}

fn get_cost(distance: u32) -> u32 {
    (distance * (distance + 1)) / 2
}

pub fn day7_part1(input: &str) -> String {
    let starting_positions: Vec<CrabHorizontalPosition> = parse_input(input);
    format!("{}", get_least_fuel_position(&starting_positions, |v| v))
}

pub fn day7_part2(input: &str) -> String {
    let starting_positions: Vec<CrabHorizontalPosition> = parse_input(input);
    format!("{}", get_least_fuel_position(&starting_positions, get_cost))
}
