type Grid = Vec<Vec<char>>;

fn parse_input(input: &str) -> Grid {
    input.lines().map(|line| {
        line.chars().collect()
    }).collect()
}

fn next_val_left(grid: &Grid, row: usize, inx: usize) -> char {
    let last_row_inx = grid[row].len() - 1;
    let val = grid[row][inx];
    // if I am empty and the one to my left is ">", I am ">"
    if val == '.' {
        if inx == 0 && (grid[row][last_row_inx] == '>') {
            '>'
        } else if 0 < inx && grid[row][inx - 1] == '>' {
            '>'
        } else {
            '.'
        }
    } else if val == 'v' {
        // 'v' cannot move during this phase
        'v'
    } else {
        // we are a '>', we will move if the spot to our right is open
        if inx == last_row_inx && grid[row][0] == '.' {
            '.'
        } else if inx < last_row_inx && grid[row][inx + 1] == '.' {
            '.'
        } else {
            '>'
        }
    }
}

fn next_val_down(grid: &Grid, row: usize, inx: usize) -> char {
    let last_row_inx = grid.len() - 1;
    let val = grid[row][inx];
    // if I am empty and the one above me is "v", I am "v"
    if val == '.' {
        if row == 0 && (grid[last_row_inx][inx] == 'v') {
            'v'
        } else if row > 0 && grid[row - 1][inx] == 'v' {
            'v'
        } else {
            '.'
        }
    } else if val == '>' {
        // '>' cannot move during this phase
        '>'
    } else {
        // we are a 'v', we will move if the spot below us is open
        if row == last_row_inx && grid[0][inx] == '.' {
            '.'
        } else if row < last_row_inx &&  grid[row + 1][inx] == '.' {
            '.'
        } else {
            'v'
        }
    }
}

fn move_left(grid: &Grid) -> (Grid, u32) {
    let mut changes = 0;
    let new_grid = grid.iter().enumerate().map(|(row_inx, row)| {
        row.iter().enumerate().map(|(inx, val)| {
            let new_val = next_val_left(&grid, row_inx, inx);
            if *val != new_val {
                changes += 1;
            }
            new_val
        }).collect()
    }).collect();
    (new_grid, changes)
}

fn move_down(grid: &Grid) -> (Grid, u32) {
    let mut changes = 0;
    let new_grid = grid.iter().enumerate().map(|(row_inx, row)| {
        row.iter().enumerate().map(|(inx, val)| {
            let new_val = next_val_down(&grid, row_inx, inx);
            if *val != new_val {
                changes += 1;
            }
            new_val
        }).collect()
    }).collect();
    (new_grid, changes)
}

pub fn day25_part1(input: &str) -> String {
    let mut grid = parse_input(input);
    let mut step = 0;
    loop {
        step += 1;
        let (new_grid_l, changes_l) = move_left(&grid);
        let (new_grid_r, changes_r) = move_down(&new_grid_l);
        if (changes_l + changes_r) == 0 {
            break;
        }
        grid = new_grid_r;
    };
    format!("{}", step)
}

pub fn day25_part2(_input: &str) -> String {
    format!("(having all 50 stars is the solution)")
}
