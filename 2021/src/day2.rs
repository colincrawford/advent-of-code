enum Movement {
    Up(i32),
    Down(i32),
    Forward(i32)
}

fn input_line_to_movement(line: &str) -> Movement {
    let mut pieces = line.split_whitespace();
    let direction = pieces.next().expect("No direction piece found");
    let amount = pieces.next().expect("No amount piece foound").parse().expect("Could not parse movement amount");
    match direction {
        "up" => Movement::Up(amount),
        "down" => Movement::Down(amount),
        "forward" => Movement::Forward(amount),
        _ => panic!("Could not parse direction {}", direction)
    }
}

fn parse_input(input: &str) -> Vec<Movement> {
    input.lines().map(input_line_to_movement).collect()
}

pub fn day2_part1(input: &str) -> String {
    let mut x = 0;
    let mut y = 0;
    for movement in parse_input(input).iter() {
        match movement {
            Movement::Up(amt) => { y += amt },
            Movement::Down(amt) => { y -= amt },
            Movement::Forward(amt) => { x += amt }
        }
    }
    format!("{}", -y * x)
}

pub fn day2_part2(input: &str) -> String {
    let mut aim = 0;
    let mut x = 0;
    let mut y = 0;
    for movement in parse_input(input).iter() {
        match movement {
            Movement::Up(amt) => { aim += amt },
            Movement::Down(amt) => { aim -= amt },
            Movement::Forward(amt) => {
                x += amt;
                y += aim * amt
            }
        }
    }
    format!("{}", -y * x)
}
