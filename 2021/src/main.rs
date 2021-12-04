use std::fs;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;

const CURRENT_DAY: u32 = 4;

fn get_input(day: u32) -> String {
    if day > CURRENT_DAY {
        return "".to_string()
    }
    let file_path = format!("./inputs/day{}.txt", day);
    fs::read_to_string(file_path).expect("Couldn't read input file")
}

fn print_answer(day: u32, ans1: &str, ans2: &str) {
    if day > CURRENT_DAY {
        return
    }
    println!("---------------------");
    println!("| day {:2}            |", day);
    println!("---------------------");
    println!("| part 1: {:9} |", ans1);
    println!("| part 2: {:9} |", ans2);
    println!("---------------------");
}

fn main() {
    let day1_input = get_input(1);
    print_answer(
        1,
        &day1::day1_part1(&day1_input),
        &day1::day1_part2(&day1_input)
    );

    let day2_input = get_input(2);
    print_answer(
        2,
        &day2::day2_part1(&day2_input),
        &day2::day2_part2(&day2_input)
    );

    let day3_input = get_input(3);
    print_answer(
        3,
        &day3::day3_part1(&day3_input),
        &day3::day3_part2(&day3_input)
    );

    let day4_input = get_input(4);
    print_answer(
        4,
        &day4::day4_part1(&day4_input),
        &day4::day4_part2(&day4_input)
    );

    let day5_input = get_input(5);
    print_answer(
        5,
        &day5::day5_part1(&day5_input),
        &day5::day5_part2(&day5_input)
    );

    let day6_input = get_input(6);
    print_answer(
        6,
        &day6::day6_part1(&day6_input),
        &day6::day6_part2(&day6_input)
    );

    let day7_input = get_input(7);
    print_answer(
        7,
        &day7::day7_part1(&day7_input),
        &day7::day7_part2(&day7_input)
    );

    let day8_input = get_input(8);
    print_answer(
        8,
        &day8::day8_part1(&day8_input),
        &day8::day8_part2(&day8_input)
    );

    let day9_input = get_input(9);
    print_answer(
        9,
        &day9::day9_part1(&day9_input),
        &day9::day9_part2(&day9_input)
    );

    let day10_input = get_input(10);
    print_answer(
        10,
        &day10::day10_part1(&day10_input),
        &day10::day10_part2(&day10_input)
    );

    let day11_input = get_input(11);
    print_answer(
        11,
        &day11::day11_part1(&day11_input),
        &day11::day11_part2(&day11_input)
    );

    let day12_input = get_input(12);
    print_answer(
        12,
        &day12::day12_part1(&day12_input),
        &day12::day12_part2(&day12_input)
    );

    let day13_input = get_input(13);
    print_answer(
        13,
        &day13::day13_part1(&day13_input),
        &day13::day13_part2(&day13_input)
    );

    let day14_input = get_input(14);
    print_answer(
        14,
        &day14::day14_part1(&day14_input),
        &day14::day14_part2(&day14_input)
    );

    let day15_input = get_input(15);
    print_answer(
        15,
        &day15::day15_part1(&day15_input),
        &day15::day15_part2(&day15_input)
    );

    let day16_input = get_input(16);
    print_answer(
        16,
        &day16::day16_part1(&day16_input),
        &day16::day16_part2(&day16_input)
    );

    let day17_input = get_input(17);
    print_answer(
        17,
        &day17::day17_part1(&day17_input),
        &day17::day17_part2(&day17_input)
    );

    let day18_input = get_input(18);
    print_answer(
        18,
        &day18::day18_part1(&day18_input),
        &day18::day18_part2(&day18_input)
    );

    let day19_input = get_input(19);
    print_answer(
        19,
        &day19::day19_part1(&day19_input),
        &day19::day19_part2(&day19_input)
    );

    let day20_input = get_input(20);
    print_answer(
        20,
        &day20::day20_part1(&day20_input),
        &day20::day20_part2(&day20_input)
    );

    let day21_input = get_input(21);
    print_answer(
        21,
        &day21::day21_part1(&day21_input),
        &day21::day21_part2(&day21_input)
    );

    let day22_input = get_input(22);
    print_answer(
        22,
        &day22::day22_part1(&day22_input),
        &day22::day22_part2(&day22_input)
    );

    let day23_input = get_input(23);
    print_answer(
        23,
        &day23::day23_part1(&day23_input),
        &day23::day23_part2(&day23_input)
    );

    let day24_input = get_input(24);
    print_answer(
        24,
        &day24::day24_part1(&day24_input),
        &day24::day24_part2(&day24_input)
    );

    let day25_input = get_input(25);
    print_answer(
        25,
        &day25::day25_part1(&day25_input),
        &day25::day25_part2(&day25_input)
    );
}
