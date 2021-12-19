#[derive(Debug)]
enum SfNumber {
    Regular(u8),
    Pair(Box<SfNumber>, Box<SfNumber>),
}

impl Clone for SfNumber {
    fn clone(&self) -> SfNumber {
        match self {
            SfNumber::Regular(n) => SfNumber::Regular(*n),
            SfNumber::Pair(left, right) => SfNumber::Pair(left.clone(), right.clone()),
        }
    }
}

impl SfNumber {
    fn magnitude(&self) -> u32 {
        match self {
            SfNumber::Regular(n) => *n as u32,
            SfNumber::Pair(left, right) => (3 * left.magnitude()) + (2 * right.magnitude()),
        }
    }

    fn add_right(&self, add_n: u8) -> SfNumber {
        match self {
            SfNumber::Regular(n) => SfNumber::Regular(n + add_n),
            SfNumber::Pair(left, right) => {
                let new_right = Box::new(right.add_right(add_n));
                SfNumber::Pair(left.clone(), new_right)
            }
        }
    }

    fn add_left(&self, add_n: u8) -> SfNumber {
        match self {
            SfNumber::Regular(n) => SfNumber::Regular(n + add_n),
            SfNumber::Pair(left, right) => {
                let new_left = Box::new(left.add_left(add_n));
                SfNumber::Pair(new_left, right.clone())
            }
        }
    }

    fn split(&self) -> (SfNumber, bool) {
        match self {
            SfNumber::Regular(n) => {
                if *n > 9 {
                    let mut half_rounded_down = *n / 2;
                    let mut half_rounded_up = half_rounded_down;
                    if *n % 2 != 0 {
                        half_rounded_down = (*n - 1) / 2;
                        half_rounded_up = ((*n - 1) / 2) + 1;
                    }
                    let num = SfNumber::Pair(
                        Box::new(SfNumber::Regular(half_rounded_down)),
                        Box::new(SfNumber::Regular(half_rounded_up)),
                    );
                    (num, true)
                } else {
                    (SfNumber::Regular(*n), false)
                }
            }
            SfNumber::Pair(left, right) => match left.split() {
                (new_left, true) => (SfNumber::Pair(Box::new(new_left), right.clone()), true),
                (new_left, false) => match right.split() {
                    (new_right, true) => (
                        SfNumber::Pair(Box::new(new_left), Box::new(new_right)),
                        true,
                    ),
                    (new_right, false) => (
                        SfNumber::Pair(Box::new(new_left), Box::new(new_right)),
                        false,
                    ),
                },
            },
        }
    }

    fn reduce(&self, depth: usize) -> (SfNumber, (Option<u8>, Option<u8>), bool) {
        match self {
            SfNumber::Regular(n) => (SfNumber::Regular(*n), (None, None), false),
            SfNumber::Pair(left, right) => {
                if depth == 5 {
                    match (*left.clone(), *right.clone()) {
                        (SfNumber::Regular(l), SfNumber::Regular(r)) => {
                            (SfNumber::Regular(0), (Some(l), Some(r)), true)
                        }
                        _ => panic!("Bad explosion {:?}", self),
                    }
                } else {
                    match left.reduce(depth + 1) {
                        (nl, (None, None), false) => match right.reduce(depth + 1) {
                            (nr, (None, None), exploded) => (
                                SfNumber::Pair(Box::new(nl), Box::new(nr)),
                                (None, None),
                                exploded,
                            ),
                            (nr, (Some(ln), Some(rn)), exploded) => {
                                let new_left = Box::new(nl.add_right(ln));
                                let new_right = Box::new(nr);
                                (
                                    SfNumber::Pair(new_left, new_right),
                                    (None, Some(rn)),
                                    exploded,
                                )
                            }
                            (nr, (None, Some(rn)), exploded) => {
                                let new_left = Box::new(nl);
                                let new_right = Box::new(nr);
                                (
                                    SfNumber::Pair(new_left, new_right),
                                    (None, Some(rn)),
                                    exploded,
                                )
                            }
                            (nr, (Some(ln), None), exploded) => {
                                let new_left = Box::new(nl.add_right(ln));
                                let new_right = Box::new(nr);
                                (SfNumber::Pair(new_left, new_right), (None, None), exploded)
                            }
                        },
                        (nl, (None, None), true) => (
                            SfNumber::Pair(Box::new(nl), right.clone()),
                            (None, None),
                            true,
                        ),
                        (nl, (Some(ln), Some(rn)), exploded) => {
                            let new_left = Box::new(nl);
                            let new_right = Box::new(right.add_left(rn));
                            (
                                SfNumber::Pair(new_left, new_right),
                                (Some(ln), None),
                                exploded,
                            )
                        }
                        (nl, (Some(ln), None), exploded) => {
                            let new_left = Box::new(nl);
                            (
                                SfNumber::Pair(new_left, right.clone()),
                                (Some(ln), None),
                                exploded,
                            )
                        }
                        (nl, (None, Some(rn)), exploded) => {
                            let new_left = Box::new(nl);
                            let new_right = Box::new(right.add_left(rn));
                            (SfNumber::Pair(new_left, new_right), (None, None), exploded)
                        }
                    }
                }
            }
        }
    }
}

fn parse_num(chars: &Vec<char>, start: usize) -> (SfNumber, usize) {
    if chars[start].is_digit(10) {
        return (
            SfNumber::Regular(chars[start].to_digit(10).unwrap() as u8),
            start,
        );
    }
    let (left, end_left) = parse_num(chars, start + 1);
    let (right, end_right) = parse_num(chars, end_left + 2);
    let num = SfNumber::Pair(Box::new(left), Box::new(right));
    (num, end_right + 1)
}

fn fully_reduce(num: SfNumber) -> SfNumber {
    let mut n = num;
    loop {
        let (new_n, _, exploded) = n.reduce(1);
        n = new_n;
        if exploded {
            continue;
        }
        let (split_n, split) = n.split();
        n = split_n;
        if split {
            continue;
        }
        break;
    }
    n
}

fn parse_line(line: &str) -> SfNumber {
    let chars = line.chars().collect();
    let (num, _) = parse_num(&chars, 0);
    num
}

fn parse_input(input: &str) -> Vec<SfNumber> {
    input.lines().map(parse_line).collect()
}

pub fn day18_part1(input: &str) -> String {
    let mut nums = parse_input(input).into_iter();
    let mut sum: SfNumber = nums.next().unwrap();
    for num in nums {
        sum = SfNumber::Pair(Box::new(sum), Box::new(num));
        sum = fully_reduce(sum);
    }
    format!("{:?}", sum.magnitude())
}

pub fn day18_part2(input: &str) -> String {
    let nums = parse_input(input);
    let mut max = 0;
    for (i, num) in nums.iter().enumerate() {
        for num2 in nums.iter().skip(i + 1) {
            let sum = SfNumber::Pair(Box::new(num.clone()), Box::new(num2.clone()));
            let sum2 = SfNumber::Pair(Box::new(num2.clone()), Box::new(num.clone()));
            let reduced = fully_reduce(sum);
            let reduced2 = fully_reduce(sum2);
            let magnitude = reduced.magnitude();
            let magnitude2 = reduced2.magnitude();
            if magnitude > max {
                max = magnitude;
            }
            if magnitude2 > max {
                max = magnitude2;
            }
        }
    }
    format!("{:?}", max)
}
