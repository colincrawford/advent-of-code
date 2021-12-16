use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashSet;

type Grid = Vec<Vec<u16>>;

#[derive(PartialEq, Eq, Hash, Clone)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    fn moves(&self, grid: &Grid, seen: &HashSet<Position>) -> Vec<Position> {
        let x = self.x as i16;
        let y = self.y as i16;
        vec![(x, y + 1), (x, y - 1), (x + 1, y), (x - 1, y)]
            .into_iter()
            .filter(|(x, y)| *x >= 0 && *y >= 0)
            .filter(|(x, y)| *y < grid.len() as i16 && *x < grid[0].len() as i16)
            .map(|(x, y)| (x as usize, y as usize))
            .map(|(x, y)| Position { x, y })
            .filter(|pos| !seen.contains(pos))
            .collect()
    }
}

#[derive(PartialEq, Eq, Hash, Clone)]
struct PositionWithCost {
    pos: Position,
    cost: u16,
}

impl Ord for PositionWithCost {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for PositionWithCost {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

const RADIX: u32 = 10;

fn parse_digit(c: char) -> u16 {
    c.to_digit(RADIX).expect("Invalid grid digit") as u16
}

fn parse_line(line: &str) -> Vec<u16> {
    line.chars().map(parse_digit).collect()
}

fn parse_input(input: &str) -> Grid {
    input.lines().map(parse_line).collect()
}

fn is_last(grid: &Grid, pos: &Position) -> bool {
    pos.x == (grid[0].len() - 1) && pos.y == (grid.len() - 1)
}

// Basic implementation of Dijkstra's algorithm treating the grid as a graph
fn shortest_path_risk(grid: &mut Grid) -> u16 {
    let mut seen: HashSet<Position> = HashSet::new();
    let mut heap = BinaryHeap::new();
    heap.push(PositionWithCost {
        cost: 0,
        pos: Position { x: 0, y: 0 },
    });

    while let Some(PositionWithCost { cost, pos }) = heap.pop() {
        if seen.contains(&pos) {
            continue;
        }

        if is_last(grid, &pos) {
            return cost;
        }

        for p in pos.moves(grid, &seen) {
            heap.push(PositionWithCost {
                cost: cost + grid[p.y][p.x],
                pos: p,
            })
        }

        seen.insert(pos);
    }
    0
}

fn add_times(n: u16, adds: u16) -> u16 {
    if n + adds > 9 {
        n + adds - 9
    } else {
        n + adds
    }
}

fn duplicate_grid_with_adds(grid: &Grid, adds: u16) -> Grid {
    grid.iter()
        .map(|row| row.iter().map(|v| add_times(*v, adds)).collect())
        .collect()
}

fn copy_grid(grid: &Grid) -> Grid {
    grid.iter()
        .map(|row| row.iter().map(|v| *v).collect())
        .collect()
}

fn merge_row_of_grids(grids: &Vec<Grid>) -> Grid {
    let mut new_grid: Grid = copy_grid(&grids[0]);
    for grid in grids.iter().skip(1) {
        for (i, row) in grid.iter().enumerate() {
            new_grid[i] = new_grid[i].iter().chain(row.iter()).map(|v| *v).collect();
        }
    }
    new_grid
}

fn merge_col_of_grids(grids: &Vec<Grid>) -> Grid {
    grids.iter().flat_map(|grid| copy_grid(grid)).collect()
}

fn build_grid_of_grids(grid: &Grid) -> Grid {
    let uncombined_grids: Vec<Vec<Grid>> = (0..5)
        .map(|row_inx| {
            (0..5)
                .map(|col_inx| duplicate_grid_with_adds(&grid, row_inx + col_inx))
                .collect()
        })
        .collect();
    let with_rows_combined: Vec<Grid> = uncombined_grids.iter().map(merge_row_of_grids).collect();
    merge_col_of_grids(&with_rows_combined)
}

pub fn day15_part1(input: &str) -> String {
    let mut grid = parse_input(input);
    let risk = shortest_path_risk(&mut grid);
    format!("{}", risk)
}

pub fn day15_part2(input: &str) -> String {
    let grid = parse_input(input);
    let mut grids = build_grid_of_grids(&grid);
    let risk = shortest_path_risk(&mut grids);
    format!("{}", risk)
}
