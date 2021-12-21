fn to_bin(c: char) -> char {
    match c {
        '.' => '0',
        '#' => '1',
        _ => panic!("invalid char input"),
    }
}

fn parse_input(input: &str) -> (Vec<char>, Vec<Vec<char>>) {
    let mut lines = input.lines();
    let algo = lines.next().unwrap().chars().collect();
    lines.next();
    let mut img: Vec<Vec<char>> = lines
        .map(|line| {
            let mut ln: Vec<char> = line.chars().collect();
            for _ in 0..51 {
                ln.insert(0, '.');
                ln.push('.');
            }
            ln
        })
        .collect();
    for _ in 0..51 {
        img.insert(0, img[0].iter().map(|_| '.').collect());
        img.push(img[0].iter().map(|_| '.').collect());
    }
    (algo, img)
}

fn digit_at(algo: &Vec<char>, img: &Vec<Vec<char>>, x: i32, y: i32, default: char) -> char {
    let cs: Vec<char> = vec![
        (x - 1, y - 1),
        (x, y - 1),
        (x + 1, y - 1),
        (x - 1, y),
        (x, y),
        (x + 1, y),
        (x - 1, y + 1),
        (x, y + 1),
        (x + 1, y + 1),
    ]
    .into_iter()
    .map(|(xx, yy)| {
        if xx < 0 || xx >= (img[0].len() as i32) {
            return default;
        }
        if yy < 0 || yy >= (img.len() as i32) {
            return default;
        }
        img[yy as usize][xx as usize]
    })
    .collect();
    let bin_s = String::from_iter(cs.iter().map(|c| to_bin(*c)));
    let inx: u32 = isize::from_str_radix(&bin_s, 2).unwrap() as u32;
    algo[inx as usize]
}

fn transform(algo: &Vec<char>, img: &Vec<Vec<char>>, n: u32) -> Vec<Vec<char>> {
    let mut default = '.';
    if n % 2 == 0 {
        default = '#';
    }

    img
        .iter()
        .enumerate()
        .map(|(y, row)| {
            row.iter()
                .enumerate()
                .map(|(x, _)| digit_at(algo, img, x as i32, y as i32, default))
                .collect()
        })
        .collect()
}

fn count_lit_digits(img: &Vec<Vec<char>>) -> u32 {
    let mut count = 0;
    for row in img.iter() {
        for v in row.iter() {
            if *v == '#' {
                count += 1;
            }
        }
    }
    count
}

fn transform_times(algo: &Vec<char>, img: &Vec<Vec<char>>, n: u32) -> Vec<Vec<char>> {
    let mut trn_img = transform(algo, img, 1);
    for i in 0..(n - 1) {
        trn_img = transform(algo, &trn_img, i + 2);
    }
    trn_img
}

pub fn day20_part1(input: &str) -> String {
    let (algo, img) = parse_input(input);
    let img3 = transform_times(&algo, &img, 2);
    format!("{}", count_lit_digits(&img3))
}

pub fn day20_part2(input: &str) -> String {
    let (algo, img) = parse_input(input);
    let img50 = transform_times(&algo, &img, 50);
    format!("{}", count_lit_digits(&img50))
}
