use std::collections::HashSet;

struct Zone {
    x_min: i32,
    x_max: i32,
    y_min: i32,
    y_max: i32,
}

fn parse_number(s: &str) -> i32 {
    if s.chars().nth(0).unwrap() == '-' {
        String::from_iter(s.chars().skip(1)).parse::<i32>().unwrap() * -1
    } else {
        s.parse().unwrap()
    }
}

fn parse_range(s: &str) -> (i32, i32) {
    let mut pieces = s.split("..");
    let min_s = pieces.next().expect("No min");
    let max_s = pieces.next().expect("No max");
    (parse_number(min_s), parse_number(max_s))
}

fn parse_input(input: &str) -> Zone {
    let mut pieces = input.trim_end().split(", ");
    let x_piece = pieces.next().expect("No x input piece");
    let y_piece = pieces.next().expect("No y input piece");
    let (x_min, x_max) = parse_range(&String::from_iter(x_piece.chars().skip(15)));
    let (y_min, y_max) = parse_range(&String::from_iter(y_piece.chars().skip(2)));
    Zone {
        x_min,
        x_max,
        y_min,
        y_max,
    }
}

fn max_y_height(zone: &Zone) -> i32 {
    // when the probe hits the water, it will have negative the initial y velocity - 1
    let starting_velocity = zone.y_min.abs() - 1;
    (starting_velocity * (starting_velocity + 1)) / 2
}

fn possible_x_vels(zone: &Zone, steps: usize) -> Vec<i32> {
    let mut vels = vec![];
    for starting_vel in 1..=zone.x_max {
        let mut vel = starting_vel;
        let mut pos = 0;
        for _ in 0..steps {
            pos += vel;
            if vel > 0 {
                vel -= 1;
            } else {
                break;
            }
        }

        if pos >= zone.x_min && pos <= zone.x_max {
            vels.push(starting_vel);
        }
    }
    vels
}

fn possible_y_vels(zone: &Zone) -> Vec<(i32, usize)> {
    let mut vels = vec![];
    for starting_vel in zone.y_min..zone.y_min.abs() {
        let mut vel = starting_vel;
        let mut steps = 0;
        let mut pos = 0;
        loop {
            steps += 1;
            pos += vel;
            vel -= 1;

            if pos < zone.y_min {
                break;
            } else if pos <= zone.y_max && pos >= zone.y_min {
                vels.push((starting_vel, steps));
            }
        }
    }
    vels
}

fn unique_launches(zone: &Zone) -> usize {
    possible_y_vels(zone)
        .into_iter()
        .flat_map(|(y, steps)| {
            possible_x_vels(zone, steps)
                .into_iter()
                .map(move |x| (x, y))
        })
        .collect::<HashSet<(i32, i32)>>()
        .len()
}

pub fn day17_part1(input: &str) -> String {
    let zone = parse_input(input);
    format!("{}", max_y_height(&zone))
}

pub fn day17_part2(input: &str) -> String {
    let zone = parse_input(input);
    format!("{}", unique_launches(&zone))
}
