mod random;
mod human;
mod bot;
mod bit_bot;
mod bit_bot_AI;
use crate::legal_moves::get_all_legal_moves;
use crate::board::Board;

pub struct Player {
    player: i8,
    player_type: u8,
    clicks: Vec<Vec<usize>>,
    promote_to: usize,
    total_time: u128,
    slowest_move: u128,
}

impl Player {
    pub fn new(player: i8, player_type: u8) -> Player {
        Player {
            player: player,
            player_type: player_type,
            clicks: Vec::new(),
            promote_to: 5, // should be '0' as default
            total_time: 0,
            slowest_move: 0,
        }
    }

    pub fn get_player_type(&self) -> u8 {
        self.player_type
    }

    pub fn get_promote_to(&self) -> usize {
        self.promote_to
    }

    pub fn set_promote_to(&mut self, promote_to: usize) {
        self.promote_to = promote_to;
    }

    pub fn get_clicks(&self) -> &Vec<Vec<usize>> {
        &self.clicks
    }

    pub fn get_total_time(&self) -> u128 {
        self.total_time
    }

    pub fn get_slowest_move(&self) -> u128 {
        self.slowest_move
    }

    pub fn add_time(&mut self, time: u128) {
        self.total_time += time;
        if time > self.slowest_move {
            self.slowest_move = time;
        }
    }

    pub fn clicked(&mut self, click: Vec<usize>) {
        if self.clicks.len() == 2 {
            self.clicks[0] = self.clicks.pop().unwrap();
        }
        self.clicks.push(click);
    }

    pub fn clear_clicks(&mut self) {
        self.clicks = Vec::new();
    }
}

pub fn run(player: i8, player_type: u8, board: &Board, movee: (Vec<usize>, usize)) -> Vec<usize> {
    match player_type {
        4 => bit_bot_AI::run(&board.board, &board.board_history, player, &board.castle_pieces),
        3 => bit_bot::run(&board.board, &board.board_history, player, &board.castle_pieces),
        2 => bot::run(&board.board, &board.board_history, player, &board.castle_pieces),
        1 => human::run(get_all_legal_moves(&board.board, &board.board_history, player, &board.castle_pieces), movee),
        _ => random::run(get_all_legal_moves(&board.board, &board.board_history, player, &board.castle_pieces)),
    }
}