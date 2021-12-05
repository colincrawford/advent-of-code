use std::collections::HashMap;

#[derive(Debug, Hash, Eq)]
struct Point {
    x: u16,
    y: u16,
}

impl Point {
    fn comes_before(&self, other: &Point) -> bool {
        (self.x <= other.x && self.y <= other.y) || self.x < other.x && self.y != other.y
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Point) -> bool {
        self.x == other.x && self.y == other.y
    }
}

#[derive(Debug)]
struct Line {
    start: Point,
    end: Point,
}

impl Line {
    fn is_vertical(&self) -> bool {
        self.start.x == self.end.x
    }

    fn is_horizontal(&self) -> bool {
        self.start.y == self.end.y
    }

    fn points(&self) -> Vec<Point> {
        if self.is_vertical() {
            (self.start.y..=self.end.y)
                .map(|y| Point { x: self.start.x, y })
                .collect()
        } else if self.is_horizontal() {
            (self.start.x..=self.end.x)
                .map(|x| Point { x, y: self.start.y })
                .collect()
        } else {
            (self.start.x..=self.end.x)
                .map(|x| {
                    let y = if self.start.y > self.end.y {
                        self.start.y - (x - self.start.x)
                    } else {
                        self.start.y + (x - self.start.x)
                    };
                    Point { x, y }
                })
                .collect()
        }
    }
}

fn parse_point(point_str: &str) -> Point {
    let mut parts = point_str.split(',');
    let x_part = parts.next().expect("No x part in input point string");
    let y_part = parts.next().expect("No y part in input point string");
    Point {
        x: x_part.parse().expect("Invalid x for input Point"),
        y: y_part.parse().expect("Invalid y for input Point"),
    }
}

fn line_from_points(first: Point, second: Point) -> Line {
    // Put the points in a canonical order
    if first.comes_before(&second) {
        Line {
            start: first,
            end: second,
        }
    } else {
        Line {
            start: second,
            end: first,
        }
    }
}

fn parse_input(input: &str) -> Vec<Line> {
    input
        .lines()
        .map(|line| {
            let mut points = line.split(" -> ");
            let first = parse_point(points.next().expect("No starting point in input line"));
            let second = parse_point(points.next().expect("No ending point in input line"));
            line_from_points(first, second)
        })
        .collect()
}

fn get_num_points_with_at_least_two_lines(lines: &Vec<Line>) -> u32 {
    let mut seen: HashMap<Point, u32> = HashMap::new();
    for point in lines.iter().flat_map(|line| line.points()) {
        let count = *seen.get(&point).unwrap_or(&0);
        seen.insert(point, count + 1);
    }
    seen.values().filter(|n| **n > 1).count() as u32
}

pub fn day5_part1(input: &str) -> String {
    let lines = parse_input(input)
        .into_iter()
        .filter(|line| line.is_vertical() || line.is_horizontal())
        .collect();
    format!("{}", get_num_points_with_at_least_two_lines(&lines))
}

pub fn day5_part2(input: &str) -> String {
    let lines = parse_input(input);
    format!("{}", get_num_points_with_at_least_two_lines(&lines))
}
