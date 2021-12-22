use std::collections::HashMap;

struct NormalDi {
    value: u16,
    total_rolls: u32,
}

#[derive(Clone, Eq, Hash)]
struct Player {
    score: u32,
    position: u32,
}

impl PartialEq for Player {
    fn eq(&self, other: &Self) -> bool {
        self.score == other.score && self.position == other.position
    }
}

struct NormalGame {
    is_player1_turn: bool,
    di: NormalDi,
    player1: Player,
    player2: Player,
}

impl NormalDi {
    fn roll(&self) -> NormalDi {
        let value = if self.value == 100 { 1 } else { self.value + 1 };
        let total_rolls = self.total_rolls + 1;
        NormalDi { value, total_rolls }
    }
}

impl NormalGame {
    fn loser(&self) -> Option<Player> {
        let winning_score = 1000;
        if self.player1.score >= winning_score {
            return Some(self.player2.clone());
        }
        if self.player2.score >= winning_score {
            return Some(self.player1.clone());
        }
        None
    }

    fn turn(&self) -> NormalGame {
        let roll1 = self.di.roll();
        let roll2 = roll1.roll();
        let di = roll2.roll();
        let player_move = roll1.value + roll2.value + di.value;

        if self.is_player1_turn {
            NormalGame {
                is_player1_turn: false,
                di,
                player1: self.player1.move_spaces(player_move),
                player2: self.player2.clone(),
            }
        } else {
            NormalGame {
                is_player1_turn: true,
                di,
                player1: self.player1.clone(),
                player2: self.player2.move_spaces(player_move),
            }
        }
    }
}

impl Player {
    fn move_spaces(&self, n: u16) -> Player {
        let mut position = (self.position + (n as u32)) % 10;
        if position == 0 {
            position = 10;
        }
        let score = self.score + position;
        Player { position, score }
    }
}

fn parse_input(input: &str) -> (u16, u16) {
    let mut starts = input.lines().map(|line| {
        let mut pieces = line.split(": ");
        pieces.next();
        pieces.next().unwrap().parse().unwrap()
    });
    (starts.next().unwrap(), starts.next().unwrap())
}

fn play_game(start1: u16, start2: u16) -> (u32, u32) {
    let mut game = NormalGame {
        is_player1_turn: true,
        di: NormalDi {
            total_rolls: 0,
            value: 0,
        },
        player1: Player {
            position: start1 as u32,
            score: 0,
        },
        player2: Player {
            position: start2 as u32,
            score: 0,
        },
    };
    loop {
        game = game.turn();
        match game.loser() {
            None => {}
            Some(player) => return (game.di.total_rolls, player.score),
        }
    }
}

fn new_rolls((r1, r2, r3): (u8, u8, u8)) -> Vec<(u8, u8, u8)> {
    if r3 == 0 && r2 > 0 {
        return vec![(r1, r2, 1), (r1, r2, 2), (r1, r2, 3)];
    }
    if r2 == 0 {
        return vec![(r1, 1, 0), (r1, 2, 0), (r1, 3, 0)];
    }
    return vec![(1, 0, 0), (2, 0, 0), (3, 0, 0)];
}

fn play_rolls(
    player1: &Player,
    player2: &Player,
    rolls: Vec<(u8, u8, u8)>,
    is_player1_turn: bool,
    cache: &mut HashMap<(Player, Player, (u8, u8, u8), bool), (u128, u128)>,
) -> (u128, u128) {
    let (p11, p21) = play_quantum(player1, player2, rolls[0], is_player1_turn, cache);
    let (p12, p22) = play_quantum(player1, player2, rolls[1], is_player1_turn, cache);
    let (p13, p23) = play_quantum(player1, player2, rolls[2], is_player1_turn, cache);
    (p11 + p12 + p13, p21 + p22 + p23)
}

fn play_quantum(
    player1: &Player,
    player2: &Player,
    rolls: (u8, u8, u8),
    is_player1_turn: bool,
    cache: &mut HashMap<(Player, Player, (u8, u8, u8), bool), (u128, u128)>,
) -> (u128, u128) {
    let cache_key = (player1.clone(), player2.clone(), rolls, is_player1_turn);
    if cache.contains_key(&cache_key) {
        let (p1, p2) = cache.get(&cache_key).unwrap();
        return (*p1, *p2);
    }

    let (r1, r2, r3) = rolls;
    let answer = if r1 > 0 && r2 > 0 && r3 > 0 {
        let to_move = (r1 + r2 + r3) as u16;
        let mut new_player1 = player1.clone();
        let mut new_player2 = player2.clone();
        if is_player1_turn {
            new_player1 = player1.move_spaces(to_move);
        } else {
            new_player2 = player2.move_spaces(to_move);
        }

        if new_player1.score >= 21 {
            (1, 0)
        } else if new_player2.score >= 21 {
            (0, 1)
        } else {
            let rolls = vec![(1, 0, 0), (2, 0, 0), (3, 0, 0)];
            play_rolls(&new_player1, &new_player2, rolls, !is_player1_turn, cache)
        }
    } else {
        play_rolls(player1, player2, new_rolls(rolls), !is_player1_turn, cache)
    };

    cache.insert(cache_key, answer);
    answer
}

pub fn day21_part1(input: &str) -> String {
    let (start1, start2) = parse_input(input);
    let (dice_rolls, losing_score) = play_game(start1, start2);
    format!("{}", dice_rolls * losing_score)
}

pub fn day21_part2(input: &str) -> String {
    let (start1, start2) = parse_input(input);
    let mut cache: HashMap<(Player, Player, (u8, u8, u8), bool), (u128, u128)> = HashMap::new();
    let player1 = Player {
        position: start1 as u32,
        score: 0,
    };
    let player2 = Player {
        position: start2 as u32,
        score: 0,
    };
    let rolls = vec![(1, 0, 0), (2, 0, 0), (3, 0, 0)];
    let (player1_wins, player2_wins) = play_rolls(&player1, &player2, rolls, true, &mut cache);
    let max = if player1_wins > player2_wins {
        player1_wins
    } else {
        player2_wins
    };
    format!("{}", max)
}
