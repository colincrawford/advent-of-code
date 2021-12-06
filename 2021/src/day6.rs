use std::collections::HashMap;

type LanternFishTimer = u8;

fn parse_input(input: &str) -> Vec<LanternFishTimer> {
    input
        .lines()
        .next()
        .expect("No input lines")
        .split(',')
        .map(|timer| timer.parse().expect("Invalid timer input value"))
        .collect()
}

fn simulate(days: u32, timer: LanternFishTimer, cache: &mut HashMap<(u32, LanternFishTimer), u64>) -> u64 {
    if days == 0 {
        return 1;
    }

    if !cache.contains_key(&(days, timer)) {
        let count = if timer == 0 {
            simulate(days - 1, 6, cache) + simulate(days - 1, 8, cache)
        } else {
            simulate(days - 1, timer - 1, cache)
        };

        cache.insert((days, timer), count);
    }

    *cache.get(&(days, timer)).unwrap()
}

fn simulation(days: u32, starting_fish: &Vec<LanternFishTimer>) -> u64 {
    let mut count = 0;
    let mut cache = HashMap::new();
    for fish_timer in starting_fish {
        count += simulate(days, *fish_timer, &mut cache);
    }
    count
}

pub fn day6_part1(input: &str) -> String {
    let number_of_days = 80;
    format!("{}", simulation(number_of_days, &parse_input(input)))
}

pub fn day6_part2(input: &str) -> String {
    let number_of_days = 256;
    format!("{}", simulation(number_of_days, &parse_input(input)))
}
