use std::collections::LinkedList;

#[derive(Debug)]
enum Brace {
    Paren(bool),      // ( or )
    Square(bool),     // [ or ]
    Curly(bool),      // { or }
    Comparison(bool), // < or >
}

enum LineScore {
    Invalid(u64),
    Valid(u64),
}

fn get_invalid_score(brace: &Brace) -> u64 {
    match brace {
        Brace::Paren(false) => 3,
        Brace::Square(false) => 57,
        Brace::Curly(false) => 1197,
        Brace::Comparison(false) => 25137,
        _ => panic!("Invalid brace for get_invalid_score"),
    }
}

fn parse_input(input: &str) -> Vec<Vec<Brace>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '(' => Brace::Paren(true),
                    ')' => Brace::Paren(false),
                    '{' => Brace::Curly(true),
                    '}' => Brace::Curly(false),
                    '[' => Brace::Square(true),
                    ']' => Brace::Square(false),
                    '<' => Brace::Comparison(true),
                    '>' => Brace::Comparison(false),
                    _ => panic!("Invalid input char"),
                })
                .collect()
        })
        .collect()
}

fn braces_match(b1: &Brace, b2: &Brace) -> bool {
    match (b1, b2) {
        (Brace::Paren(true), Brace::Paren(false))
        | (Brace::Curly(true), Brace::Curly(false))
        | (Brace::Square(true), Brace::Square(false))
        | (Brace::Comparison(true), Brace::Comparison(false)) => true,
        _ => false,
    }
}

fn score_line(line: &Vec<Brace>) -> LineScore {
    let mut queue = LinkedList::new();
    for brace in line {
        match brace {
            Brace::Paren(true)
            | Brace::Square(true)
            | Brace::Curly(true)
            | Brace::Comparison(true) => queue.push_back(brace),
            _ => match queue.pop_back() {
                None => {
                    return LineScore::Invalid(get_invalid_score(brace));
                }
                Some(open) => {
                    if !braces_match(open, brace) {
                        return LineScore::Invalid(get_invalid_score(brace));
                    }
                }
            },
        }
    }

    let mut score: u64 = 0;
    for brace in queue.iter().rev() {
        score = match brace {
            Brace::Paren(true) => (score * 5) + 1,
            Brace::Square(true) => (score * 5) + 2,
            Brace::Curly(true) => (score * 5) + 3,
            Brace::Comparison(true) => (score * 5) + 4,
            _ => panic!("Invalid line state"),
        };
    }

    LineScore::Valid(score)
}

fn score_lines(lines: &Vec<Vec<Brace>>) -> Vec<LineScore> {
    lines.iter().map(score_line).collect()
}

pub fn day10_part1(input: &str) -> String {
    let lines = parse_input(input);
    let score: u64 = score_lines(&lines)
        .iter()
        .map(|score| match score {
            LineScore::Valid(_) => 0,
            LineScore::Invalid(score) => *score,
        })
        .sum();
    format!("{}", score)
}

pub fn day10_part2(input: &str) -> String {
    let lines = parse_input(input);
    let mut scores: Vec<u64> = score_lines(&lines)
        .iter()
        .map(|score| match score {
            LineScore::Valid(score) => *score,
            LineScore::Invalid(_) => 0,
        })
        .filter(|score| *score > 0)
        .collect();
    scores.sort();
    // there being a middle element is an invariant of the program
    format!("{}", scores[(scores.len() - 1) / 2])
}
