fn parse_input(input: &str) -> Vec<u32> {
    input.lines()
        .into_iter()
        .map(|line| line.parse().expect("Unable to parse input line"))
        .collect()
}

fn count_increasing_windows(data: Vec<u32>, window_size: u32) -> u32 {
    let mut window_start_inx: usize = 0;
    let mut count = 0;
    let mut window_sum: i32 = 0;
    for i in 0..window_size {
        window_sum += data[i as usize] as i32
    }
    let last_inx: u32 = data.len() as u32;
    for i in window_size..last_inx {
        let new_window_sum = window_sum - (data[window_start_inx] as i32) + (data[i as usize] as i32);
        if new_window_sum > window_sum {
            count += 1
        }
        window_start_inx += 1;
        window_sum += data[i as usize] as i32 - data[window_start_inx] as i32
    }
    count
}

pub fn day1_part1(input: &str) -> String {
    let input_data = parse_input(input);
    let number_increasing = count_increasing_windows(input_data, 1);
    format!("{}", number_increasing)
}

pub fn day1_part2(input: &str) -> String {
    let input_data = parse_input(input);
    let number_increasing = count_increasing_windows(input_data, 3);
    format!("{}", number_increasing)
}

