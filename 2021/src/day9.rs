use std::collections::{HashMap, HashSet};

type Grid = Vec<Vec<u8>>;

struct LowPoint {
    score: u16,
}

fn parse_input(input: &str) -> Grid {
    const RADIX: u32 = 10;
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|height| height.to_digit(RADIX).expect("Invalid input point") as u8)
                .collect()
        })
        .collect()
}

fn find_low_points(grid: &Grid) -> Vec<LowPoint> {
    grid.iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.iter()
                .enumerate()
                .filter(move |(j, &value)| {
                    let lower_left = *j == (0 as usize) || row[j - 1] > value;
                    let lower_right = *j == (row.len() - 1) || row[j + 1] > value;
                    let lower_top = i == (0 as usize) || grid[i - 1][*j] > value;
                    let lower_bottom = i == (grid.len() - 1) || grid[i + 1][*j] > value;
                    lower_left && lower_right && lower_top && lower_bottom
                })
                .map(|(_, value)| LowPoint {
                    score: (value + 1) as u16,
                })
        })
        .collect()
}

fn find_top_3_valley_sizes_score(grid: &Grid) -> u32 {
    let mut top_size = 0;
    let mut snd_size = 0;
    let mut trd_size = 0;

    let mut cache = HashMap::new();
    // make sure we don't loop while traversing the grid
    let mut visited = HashSet::new();
    for (i, row) in grid.iter().enumerate() {
        for (j, _) in row.iter().enumerate() {
            let size = find_valley_size_for((i, j), grid, &mut cache, &mut visited);

            if size > top_size {
                trd_size = snd_size;
                snd_size = top_size;
                top_size = size;
            } else if size > snd_size {
                trd_size = snd_size;
                snd_size = size;
            } else if size > trd_size {
                trd_size = size;
            }
        }
    }

    top_size * snd_size * trd_size
}

fn find_valley_size_for(
    point: (usize, usize),
    grid: &Grid,
    cache: &mut HashMap<(usize, usize), u32>,
    visited: &mut HashSet<(usize, usize)>,
) -> u32 {
    if visited.contains(&point) {
        return 0;
    }

    visited.insert(point);
    if !cache.contains_key(&point) {
        let (row, col) = point;
        let val = grid[row][col];
        if val == 9 {
            cache.insert(point, 0);
        } else {
            let mut right = 0;
            if col != (grid[row].len() - 1) {
                right = find_valley_size_for((row, col + 1), grid, cache, visited);
            };
            let mut bottom = 0;
            if row != (grid.len() - 1) {
                bottom = find_valley_size_for((row + 1, col), grid, cache, visited);
            };
            let mut left = 0;
            if col != 0 {
                left = find_valley_size_for((row, col - 1), grid, cache, visited);
            };
            let mut top = 0;
            if row != 0 {
                top = find_valley_size_for((row - 1, col), grid, cache, visited);
            };
            cache.insert(point, 1 + top + right + bottom + left);
        }
    }

    *cache.get(&point).expect("Lookup failed for point")
}

pub fn day9_part1(input: &str) -> String {
    let grid = parse_input(input);
    let low_point_scores_sum: u16 = find_low_points(&grid)
        .iter()
        .map(|low_point| low_point.score)
        .sum();
    format!("{}", low_point_scores_sum)
}

pub fn day9_part2(input: &str) -> String {
    let grid = parse_input(input);
    let score = find_top_3_valley_sizes_score(&grid);
    format!("{:?}", score)
}
