use std::cmp::{max, min};

enum Direction {
    On,
    Off,
}

#[derive(Eq, Clone)]
struct Range {
    min: i128,
    max: i128,
}

impl PartialEq for Range {
    fn eq(&self, other: &Range) -> bool {
        self.min == other.min && self.max == other.max
    }
}

#[derive(Eq, Clone)]
struct Cube {
    x: Range,
    y: Range,
    z: Range,
}

impl PartialEq for Cube {
    fn eq(&self, other: &Cube) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}

struct Instruction {
    direction: Direction,
    cube: Cube,
}

impl Cube {
    fn volume(&self) -> u128 {
        ((self.x.max - self.x.min + 1)
            * (self.y.max - self.y.min + 1)
            * (self.z.max - self.z.min + 1)) as u128
    }

    fn subtract(&self, other: &Cube) -> Vec<Cube> {
        let intersect = self.intersection(other);
        let mut cubes = vec![];
        match intersect {
            None => vec![self.clone()],
            Some(c) => {
                if *self == c {
                    return cubes;
                }
                if self.x.min < c.x.min {
                    let max = c.x.min - 1;
                    let x = Range { max, ..self.x };
                    cubes.push(Cube { x, ..self.clone() });
                }
                if self.x.max > c.x.max {
                    let min = c.x.max + 1;
                    let x = Range { min, ..self.x };
                    cubes.push(Cube { x, ..self.clone() });
                }
                if self.y.min < c.y.min {
                    let x = Range { ..c.x };
                    let y = Range {
                        max: c.y.min - 1,
                        ..self.y
                    };
                    cubes.push(Cube { x, y, ..self.clone() });
                }
                if self.y.max > c.y.max {
                    let x = Range { ..c.x };
                    let y = Range {
                        min: c.y.max + 1,
                        ..self.y
                    };
                    cubes.push(Cube { x, y, ..self.clone() });
                }
                if self.z.min < c.z.min {
                    let z = Range {
                        max: c.z.min - 1,
                        ..self.z
                    };
                    cubes.push(Cube { z, ..c.clone() });
                }
                if self.z.max > c.z.min {
                    let z = Range {
                        min: c.z.max + 1,
                        ..self.z
                    };
                    cubes.push(Cube { z, ..c.clone() });
                }
                cubes
            }
        }
    }

    fn intersection(&self, other: &Cube) -> Option<Cube> {
        let x = Range {
            min: max(self.x.min, other.x.min),
            max: min(self.x.max, other.x.max),
        };
        let y = Range {
            min: max(self.y.min, other.y.min),
            max: min(self.y.max, other.y.max),
        };
        let z = Range {
            min: max(self.z.min, other.z.min),
            max: min(self.z.max, other.z.max),
        };
        if x.max < x.min || y.max < y.min || z.max < z.min {
            return None;
        }
        Some(Cube { x, y, z })
    }
}

fn intersect_with(cubes: &Vec<Cube>, cube: &Cube) -> Vec<Cube> {
    cubes
        .iter()
        .map(|c| c.intersection(cube))
        .filter(|c| c.is_some())
        .map(|c| c.unwrap())
        .collect()
}

fn subtract_from(cubes: &Vec<Cube>, cube: &Cube) -> Vec<Cube> {
    cubes.iter().flat_map(|c| c.subtract(cube)).collect()
}

fn total_volume(cubes: &Vec<Cube>) -> u128 {
    cubes.iter().map(|c| c.volume()).sum()
}

fn parse_coord_range(coord: &str) -> Range {
    let s = String::from_iter(coord.chars().skip(2));
    let mut pieces = s.split("..");
    let min = pieces.next().unwrap().parse::<i128>().unwrap();
    let max = pieces.next().unwrap().parse::<i128>().unwrap();
    Range { min, max }
}

fn parse_instruction(line: &str) -> Instruction {
    let mut pieces = line.split(' ');
    let direction = match pieces.next().unwrap() {
        "on" => Direction::On,
        "off" => Direction::Off,
        _ => panic!("Invalid direction"),
    };
    let coords_piece = pieces.next().unwrap();
    let mut coord_pieces = coords_piece.split(',');
    let x_piece = coord_pieces.next().unwrap();
    let y_piece = coord_pieces.next().unwrap();
    let z_piece = coord_pieces.next().unwrap();
    let x = parse_coord_range(x_piece);
    let y = parse_coord_range(y_piece);
    let z = parse_coord_range(z_piece);
    let cube = Cube { x, y, z };
    Instruction { direction, cube }
}

fn parse_input(input: &str) -> Vec<Instruction> {
    input.lines().map(|line| parse_instruction(line)).collect()
}

fn get_cubes(instructions: &Vec<Instruction>) -> Vec<Cube> {
    let mut cubes = vec![];
    for instr in instructions.iter() {
        cubes = subtract_from(&cubes, &instr.cube);
        match instr.direction {
            Direction::On => {
                cubes.push(instr.cube.clone());
            }
            _ => {}
        }
    }
    cubes
}

pub fn day22_part1(input: &str) -> String {
    let instructions = parse_input(input);
    let start_cube = Cube {
        x: Range { min: -50, max: 50 },
        y: Range { min: -50, max: 50 },
        z: Range { min: -50, max: 50 },
    };
    let cubes = get_cubes(&instructions);
    let valid_cubes = intersect_with(&cubes, &start_cube);
    format!("{}", total_volume(&valid_cubes))
}

pub fn day22_part2(input: &str) -> String {
    let instructions = parse_input(input);
    let cubes = get_cubes(&instructions);
    format!("{}", total_volume(&cubes))
}
