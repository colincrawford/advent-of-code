use std::collections::HashSet;

type Grid = Vec<Vec<u8>>;
type Point = (i16, i16);

fn parse_line(line: &str) -> Vec<u8> {
    const RADIX: u32 = 10;
    line.chars()
        .map(|c| c.to_digit(RADIX).expect("Invalid input char") as u8)
        .collect()
}

fn parse_input(input: &str) -> Grid {
    input.lines().map(parse_line).collect()
}

fn adjacent_points((i, j): Point, num_rows: usize, row_len: usize) -> Vec<Point> {
    let points = ((i - 1)..=(i + 1)).flat_map(|i| ((j - 1)..=(j + 1)).map(move |j| (i, j)));
    points
        .filter(|(i2, j2)| !(i == *i2 && j == *j2))
        .filter(|(i2, j2)| *i2 >= 0 && *i2 < num_rows as i16 && *j2 >= 0 && *j2 < row_len as i16)
        .collect()
}

fn flash(grid: &mut Grid, point: Point, flashed: &mut HashSet<Point>) -> u32 {
    let (i, j) = point;
    if flashed.contains(&point) || grid[i as usize][j as usize] <= 9 {
        return 0;
    }

    let adjacent = adjacent_points(point, grid.len(), grid[0].len());
    grid[i as usize][j as usize] = 0;
    flashed.insert(point);
    for (pi, pj) in adjacent.iter().filter(|p| !flashed.contains(&p)) {
        grid[*pi as usize][*pj as usize] += 1;
    }

    1 + adjacent
        .iter()
        .map(|p| flash(grid, *p, flashed))
        .sum::<u32>()
}

fn points(grid: &Grid) -> Vec<Point> {
    (0..grid.len())
        .flat_map(|i| ((0..grid[i].len()).map(move |j| (i as i16, j as i16))))
        .collect()
}

fn step(grid: &mut Grid) -> u32 {
    let mut total = 0;
    let mut flashed = HashSet::new();
    for (i, j) in points(grid) {
        grid[i as usize][j as usize] += 1;
    }
    for (i, j) in points(grid) {
        total += flash(grid, (i as i16, j as i16), &mut flashed);
    }
    total
}

fn count_flashes(grid: &mut Grid, steps: u16) -> u32 {
    (0..steps).map(|_| step(grid)).sum()
}

fn first_step_with_all_flash(grid: &mut Grid) -> u32 {
    let grid_size = grid.len() * grid[0].len();
    let mut steps = 1;
    loop {
        if step(grid) == (grid_size as u32) {
            return steps;
        }
        steps += 1;
    }
}

pub fn day11_part1(input: &str) -> String {
    let steps = 100;
    let mut grid = parse_input(input);
    format!("{}", count_flashes(&mut grid, steps))
}

pub fn day11_part2(input: &str) -> String {
    let mut grid = parse_input(input);
    format!("{}", first_step_with_all_flash(&mut grid))
}
