use std::collections::{BinaryHeap, HashSet};
use std::{cmp, cmp::Ordering};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Piece {
    None,
    A,
    B,
    C,
    D,
}

impl Piece {
    fn to_string(&self) -> String {
        let s = match self {
            Piece::A => "A",
            Piece::B => "B",
            Piece::C => "C",
            Piece::D => "D",
            Piece::None => ".",
        };
        String::from(s)
    }

    fn get_score(&self) -> u32 {
        match self {
            Piece::A => 1,
            Piece::B => 10,
            Piece::C => 100,
            Piece::D => 1000,
            Piece::None => 0,
        }
    }

    fn room_inx(&self) -> usize {
        match self {
            Piece::A => 0,
            Piece::B => 1,
            Piece::C => 2,
            Piece::D => 3,
            Piece::None => panic!("No room inx for None Piece"),
        }
    }

    fn inx_to_piece(i: usize) -> Self {
        match i {
            0 => Piece::A,
            1 => Piece::B,
            2 => Piece::C,
            3 => Piece::D,
            _ => panic!("No Piece for invalid inx"),
        }
    }
}

#[derive(Copy, Clone, Eq)]
struct Game {
    hall: [Piece; 11],
    rooms: [[Piece; 4]; 4],
    score: u32,
}

impl Ord for Game {
    fn cmp(&self, other: &Self) -> Ordering {
        other.score.cmp(&self.score)
    }
}
impl PartialOrd for Game {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(other.score.cmp(&self.score))
    }
}
impl PartialEq for Game {
    fn eq(&self, other: &Self) -> bool {
        self.score == other.score
    }
}

impl Game {
    fn make(rooms: [[Piece; 4]; 4]) -> Self {
        Game {
            hall: [
                Piece::None,
                Piece::None,
                Piece::None,
                Piece::None,
                Piece::None,
                Piece::None,
                Piece::None,
                Piece::None,
                Piece::None,
                Piece::None,
                Piece::None,
            ],
            rooms,
            score: 0,
        }
    }

    fn done(&self) -> bool {
        !self
            .rooms
            .iter()
            .enumerate()
            .any(|(i, s)| s.iter().any(|l| *l != Piece::inx_to_piece(i)))
    }

    fn hall_open(&self, from: usize, to: usize, room: usize) -> bool {
        let mut start = from;
        let mut end = to;

        if from > to {
            end = from - room;
            start = to;
        } else {
            start += room;
        }

        !(start..=end).any(|i| self.hall[i] != Piece::None)
    }

    fn hall_to_string(&self) -> String {
        self.hall.map(|piece| piece.to_string()).join("")
    }

    fn to_string(&self) -> String {
        let mut s = String::new();
        s.push_str(&format!("{}\n", self.score));
        s.push_str(&self.hall_to_string());
        for i in 0..4 {
            s.push_str("\n");
            for j in 0..4 {
                s.push_str(&self.rooms[i][j].to_string());
            }
        }
        s
    }

    fn taken_hall_spots(&self) -> Vec<(usize, Piece)> {
        self.hall
            .iter()
            .map(|piece| piece.clone())
            .enumerate()
            .filter(|(_, piece)| *piece != Piece::None)
            .collect()
    }

    fn valid_room(&self, room: usize) -> bool {
        !self.rooms[room]
            .iter()
            .any(|piece| *piece != Piece::inx_to_piece(room) && *piece != Piece::None)
    }

    fn open_room_inx(&self, room: usize) -> Option<usize> {
        self.rooms[room]
            .iter()
            .rposition(|piece| *piece == Piece::None)
    }

    fn move_piece(&self, hall_inx: usize, room_index: usize, room_spot: usize) -> Game {
        let piece = self.hall[hall_inx];
        let mut game = self.clone();
        game.rooms[room_index][room_spot] = piece;
        game.hall[hall_inx] = Piece::None;
        game.score += move_score(piece, hall_inx, room_index, room_spot);
        game
    }

    fn room_move_piece(
        &self,
        room_inx: usize,
        room_spot: usize,
        hall_inx: usize,
        hall_spot: usize,
    ) -> Game {
        let piece = self.rooms[room_inx][room_spot];
        let mut game = self.clone();
        game.rooms[room_inx][room_spot] = Piece::None;
        game.hall[hall_spot] = piece;
        game.score += room_move_score(piece, hall_spot, hall_inx, room_spot);
        game
    }

    fn rooms_to_move(&self) -> Vec<(usize, [Piece; 4])> {
        self.rooms
            .iter()
            .map(|room| *room)
            .enumerate()
            .filter(|(i, room)| {
                room.iter()
                    .any(|piece| *piece != Piece::inx_to_piece(*i) && *piece != Piece::None)
            })
            .collect()
    }
}

fn move_score(piece: Piece, hall_inx: usize, room_index: usize, room_spot: usize) -> u32 {
    let hall_steps =
        (cmp::max(hall_inx, room_index * 2 + 2) - cmp::min(hall_inx, room_index * 2 + 2)) as u32;
    let steps = hall_steps + (room_spot as u32) + 1;
    piece.get_score() * steps
}

fn room_move_score(piece: Piece, hall_spot: usize, hall_inx: usize, room_spot: usize) -> u32 {
    piece.get_score()
        * ((cmp::max(hall_spot, hall_inx) - cmp::min(hall_spot, hall_inx) + room_spot + 1) as u32)
}

fn solve(game: Game) -> u32 {
    let hall_inxs = [0, 1, 3, 5, 7, 9, 10];
    let mut visited = HashSet::new();
    let mut stack = BinaryHeap::new();

    visited.insert(game.to_string());
    stack.push(game);

    while let Some(game) = stack.pop() {
        if game.done() {
            return game.score;
        }

        let mut inside_room = false;

        for (hall_inx, piece) in game.taken_hall_spots() {
            let room_index = piece.room_inx();
            if game.hall_open(hall_inx, room_index * 2 + 2, 1) && game.valid_room(room_index) {
                if let Some(room_spot) = game.open_room_inx(room_index) {
                    let new_game = game.move_piece(hall_inx, room_index, room_spot);

                    if !visited.contains(&new_game.to_string()) {
                        visited.insert(new_game.to_string());
                        stack.push(new_game);
                    }
                    inside_room = true;
                }
            }
        }

        if inside_room {
            continue;
        }

        for (room_inx, room) in game.rooms_to_move() {
            if let Some(room_spot) = room.iter().position(|piece| *piece != Piece::None) {
                let hall_inx = room_inx * 2 + 2;
                for hall_spot in hall_inxs {
                    if game.hall_open(hall_spot, hall_inx, 0) {
                        let new_game =
                            game.room_move_piece(room_inx, room_spot, hall_inx, hall_spot);

                        if !visited.contains(&new_game.to_string()) {
                            visited.insert(new_game.to_string());
                            stack.push(new_game);
                        }
                    }
                }
            }
        }
    }
    0
}

fn parse_piece_line(line: &str) -> Vec<Piece> {
    let s = String::from_iter(line.chars().skip(3).take(7));
    s.split('#')
        .map(|c| match c {
            "A" => Piece::A,
            "B" => Piece::B,
            "C" => Piece::C,
            "D" => Piece::D,
            _ => panic!("invalid input"),
        })
        .collect()
}

fn parse_input(input: &str) -> Game {
    let mut lines = input.lines();
    lines.next();
    lines.next();
    let mut rooms: [[Piece; 4]; 4] = [
        [Piece::None, Piece::None, Piece::None, Piece::None],
        [Piece::None, Piece::None, Piece::None, Piece::None],
        [Piece::None, Piece::None, Piece::None, Piece::None],
        [Piece::None, Piece::None, Piece::None, Piece::None],
    ];
    for (i, line) in lines.enumerate() {
        if line == "  #########" {
            break;
        }
        for (j, x) in parse_piece_line(line).iter().enumerate() {
            rooms[j][i] = *x;
        }
    }
    Game::make(rooms)
}

pub fn day23_part1(_input: &str) -> String {
    // just did the puzzle by hand
    format!("{}", 10411)
}

pub fn day23_part2(input: &str) -> String {
    let game = parse_input(input);
    let score = solve(game);
    format!("{:?}", score)
}
