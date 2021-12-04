use std::collections::HashSet;

type BoardNumber = u8;

struct BingoBoard {
    has_numbers: HashSet<BoardNumber>,
    rows: Vec<HashSet<BoardNumber>>,
    columns: Vec<HashSet<BoardNumber>>,
}

// determines whether a row or column has been fully called
fn sequence_is_called(seq: &HashSet<BoardNumber>, called_numbers: &HashSet<BoardNumber>) -> bool {
    seq.difference(called_numbers).count() == 0
}

impl BingoBoard {
    fn is_winner(&self, called_numbers: &HashSet<BoardNumber>) -> bool {
        self.rows
            .iter()
            .chain(self.columns.iter())
            .any(|seq| sequence_is_called(seq, called_numbers))
    }

    fn get_unused_numbers(&self, called_numbers: &HashSet<BoardNumber>) -> HashSet<BoardNumber> {
        self.has_numbers
            .difference(called_numbers)
            .map(|val| *val)
            .collect()
    }
}

fn parse_input(input: &str) -> (Vec<BingoBoard>, Vec<u8>) {
    let mut input_lines = input.lines();
    let moves = input_lines
        .next()
        .expect("No input line found")
        .split(',')
        .map(|num| num.parse().expect("Invalid Bingo number"))
        .collect();
    let mut boards = vec![];
    let mut row = 0;
    for line in input_lines {
        if line.is_empty() {
            boards.push(BingoBoard {
                rows: (0..5).map(|_| HashSet::new()).collect(),
                columns: (0..5).map(|_| HashSet::new()).collect(),
                has_numbers: HashSet::new(),
            });
            row = 0;
            continue;
        }
        for (i, c) in line.split_whitespace().enumerate() {
            let num = c.parse().expect("Invalid Bingo number");
            let board = boards.last_mut().expect("No Boards");
            board.rows[row].insert(num);
            board.columns[i].insert(num);
            board.has_numbers.insert(num);
        }
        row += 1
    }
    (boards, moves)
}

fn play_game<'a>(
    boards: &'a Vec<BingoBoard>,
    numbers: &Vec<BoardNumber>,
) -> (&'a BingoBoard, HashSet<BoardNumber>, BoardNumber) {
    let mut called_numbers = HashSet::new();
    for num in numbers {
        called_numbers.insert(*num);
        for board in boards {
            if board.is_winner(&called_numbers) {
                return (board, called_numbers, *num);
            }
        }
    }
    panic!("No winning boards!")
}

fn last_winning_board<'a>(
    boards: &'a Vec<BingoBoard>,
    numbers: &Vec<BoardNumber>,
) -> (&'a BingoBoard, HashSet<BoardNumber>, BoardNumber) {
    let mut called_numbers = HashSet::new();
    let mut non_winning_boards: Vec<&BingoBoard> = boards.iter().collect();
    let mut last_winning_board = non_winning_boards[0];
    for num in numbers {
        called_numbers.insert(*num);
        non_winning_boards = non_winning_boards
            .into_iter()
            .filter(|board| !board.is_winner(&called_numbers))
            .collect();
        if non_winning_boards.len() == 1 {
            last_winning_board = non_winning_boards[0]
        }
        if non_winning_boards.len() == 0 {
            return (last_winning_board, called_numbers, *num);
        }
    }
    panic!("Last winning board not found!")
}

fn get_board_score(
    winning_board: &BingoBoard,
    called_numbers: &HashSet<BoardNumber>,
    last_number_called: BoardNumber,
) -> u32 {
    let unused: u32 = winning_board
        .get_unused_numbers(&called_numbers)
        .into_iter()
        .map(|n| n as u32)
        .sum();
    unused * last_number_called as u32
}

pub fn day4_part1(input: &str) -> String {
    let (boards, moves) = parse_input(input);
    let (winning_board, called_numbers, last_number_called) = play_game(&boards, &moves);
    let winning_score = get_board_score(&winning_board, &called_numbers, last_number_called);
    format!("{}", winning_score)
}

pub fn day4_part2(input: &str) -> String {
    let (boards, moves) = parse_input(input);
    let (winning_board, called_numbers, last_number_called) = last_winning_board(&boards, &moves);
    let last_winning_score = get_board_score(&winning_board, &called_numbers, last_number_called);
    format!("{}", last_winning_score)
}
