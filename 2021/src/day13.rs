use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Point {
    x: u16,
    y: u16,
}

type Grid = Vec<Point>;

#[derive(Debug)]
enum Fold {
    X(u16),
    Y(u16),
}

fn parse_num(s: Option<&str>) -> u16 {
    s.expect("No number").parse().expect("Invalid number")
}

fn parse_input(input: &str) -> (Grid, Vec<Fold>) {
    let points = input
        .lines()
        .take_while(|l| !l.is_empty())
        .map(|line| {
            let mut coord_pieces = line.split(',');
            let x = parse_num(coord_pieces.next());
            let y = parse_num(coord_pieces.next());
            Point { x, y }
        })
        .collect();
    let folds = input
        .lines()
        .skip_while(|l| !l.is_empty())
        .skip(1)
        .map(|f| {
            let mut pieces = f.split(' ');
            pieces.next();
            pieces.next();
            let mut fold_at = pieces.next().expect("No fold at location").split('=');
            let fold_direction = fold_at.next().expect("No fold at direction");
            let fold_amount = parse_num(fold_at.next());
            match fold_direction {
                "x" => Fold::X(fold_amount),
                "y" => Fold::Y(fold_amount),
                _ => panic!("Invalid fold direction {}", fold_direction),
            }
        })
        .collect();
    (points, folds)
}

fn fold_grid(points: &Vec<Point>, folds: &Vec<Fold>) -> Vec<Point> {
    let mut ps: HashSet<Point> = points.iter().map(|p| p.clone()).collect();
    for fold in folds {
        let mut new_ps = HashSet::new();
        match fold {
            Fold::X(amt) => {
                for p in ps.iter() {
                    if p.x > *amt {
                        let x = (*amt * 2) - p.x;
                        new_ps.insert(Point { x, y: p.y });
                    } else if p.x < *amt {
                        new_ps.insert(p.clone());
                    }
                }
            }
            Fold::Y(amt) => {
                for p in ps.iter() {
                    if p.y > *amt {
                        let y = (*amt * 2) - p.y;
                        new_ps.insert(Point { y, x: p.x });
                    } else if p.y < *amt {
                        new_ps.insert(p.clone());
                    }
                }
            }
        }
        ps = new_ps;
    }
    ps.into_iter().collect()
}

fn plot(points: &Vec<Point>) -> String {
    let mut grid: Vec<Vec<bool>> = vec![];
    let mut max_x = 0;
    let mut max_y = 0;
    for point in points.iter() {
        if point.x > max_x {
            max_x = point.x;
        }
        if point.y > max_y {
            max_y = point.y;
        }
    }
    for _ in 0..=max_y {
        grid.push((0..=max_x).map(|_| false).collect());
    }
    for p in points.iter() {
        grid[p.y as usize][p.x as usize] = true;
    }
    grid.iter()
        .map(|row| {
            row.iter()
                .map(|v| String::from(if *v { "x" } else { "." }))
                .collect::<Vec<String>>()
                .join("")
        })
        .collect::<Vec<String>>()
        .join("\n")
}

pub fn day13_part1(input: &str) -> String {
    let (points, folds) = parse_input(input);
    let first_fold = folds.into_iter().take(1).collect();
    format!("{}", fold_grid(&points, &first_fold).len())
}

pub fn day13_part2(input: &str) -> String {
    let (points, folds) = parse_input(input);
    let grid = fold_grid(&points, &folds);
    format!("\n{}", plot(&grid))
}
