
/*
    0 - empty
    1 - pawn
    2 - knight
    3 - bishop
    4 - rook
    5 - queen
    6 - king
 */

/*
    0 - empty
    1 - white
    -1 - black
*/

/*
    0 - no result
    1 - white won
    -1 - black won
    2 - illegal move by black, white won
    -2 - illegal move by white, black won
    3 - draw
*/

use std::collections::{HashMap, HashSet};

#[derive(Clone)]
pub struct Board {
    pub board: Vec<Vec<i8>>,
    pub board_history: Vec<Vec<Vec<i8>>>,
    pub board_repeated: HashMap<Vec<Vec<i8>>, i8>,
    pub turn: i8,
    pub fifty_move_rule: i16,
    pub castle_pieces: HashSet<(usize,usize)>,
    move_history: Vec<String>,
}

impl Board {

    pub fn new_board(starting_player: i8) -> Board {
        let mut board = vec![vec![0; 8]; 8];
        board[0] = vec![4,2,3,5,6,3,2,4];
        board[1] = vec![1; 8];
        board[6] = vec![-1; 8];
        board[7] = vec![-4,-2,-3,-5,-6,-3,-2,-4];
        print_board(&board);
        let mut hm = HashMap::new();
        hm.insert(board.clone(), 1);
        Board {
            board: board,
            board_history: vec![],
            board_repeated: hm,
            turn: starting_player,
            fifty_move_rule: 0,
            castle_pieces: vec![(0,0),(0,4),(0,7), (7,0),(7,4),(7,7)].into_iter().collect::<HashSet<(usize,usize)>>(),
            move_history: vec![],
        }
    }

    pub fn update_board(&mut self, start: Vec<usize>, end: Vec<usize>, promote_to: i8) -> i8 {
        if self.board[start[0]][start[1]] == 0 {
            println!("no piece at start");
            game_ended(self, start, end, promote_to);
            return self.turn * 2 * -1;
        }
        let player = self.board[start[0]][start[1]] / self.board[start[0]][start[1]].abs();
        if !legal_move(self, &start, &end, player, true) {
            println!("illegal move, start: {:?}, end: {:?}", start, end);
            game_ended(self, start, end, promote_to);
            return player * 2 * -1
        }
        self.castle_pieces.remove(&(start[0], start[1]));
        self.castle_pieces.remove(&(end[0], end[1]));
        self.board_history.push(self.board.clone());
        self.move_history.push(format_move(self.board.clone(), start[0], start[1], end[0], end[1], promote_to));
        if self.board[end[0]][end[1]] != 0 || self.board[start[0]][start[1]].abs() == 1{
            self.fifty_move_rule = 0;
        } else {
            self.fifty_move_rule += 1;
        }
        self.board[end[0]][end[1]] = self.board[start[0]][start[1]];
        self.board[start[0]][start[1]] = 0;
        if self.board[end[0]][end[1]].abs() == 1 && (end[0] == 0 || end[0] == 7) {
            if promote_to < 2 || promote_to > 5 {
                println!("illegal promotion, promotion: {}", promote_to * player);
                game_ended(self, start, end, promote_to);
                return player * 2 * -1
            }
            self.board[end[0]][end[1]] = promote_to * player;
        }
        self.turn *= -1;
        if won(&self.board, &self.board_history, self.turn) {
            print_board(&self.board);
            return player;
        }
        if draw(&self.board, &self.board_history, &mut self.board_repeated, self.turn, self.fifty_move_rule) {
            print_board(&self.board);
            return 3;
        }
        0
    }

    pub fn get_move_history(&self) -> Vec<String> {
        self.move_history.clone()
    }

}

fn game_ended(board: &Board, start: Vec<usize>, end: Vec<usize>, promote_to: i8) {
    println!("game_ended");
    println!("move history: {:?}", board.board_history);
    println!("start: {:?}, end: {:?}, promote_to: {:?}", start, end, promote_to);
    print_board(&board.board);
}

fn legal_move(board: &mut Board, start: &Vec<usize>, end: &Vec<usize>, player: i8, check_checker: bool) -> bool {
    if board.turn != player {
        println!("wrong turn");
        return false;
    }
    if board.board[end[0]][end[1]] != 0 {
        if board.board[end[0]][end[1]] / (board.board[end[0]][end[1]]).abs() == player {
            // println!("illegal move, end position occupied by own piece");
            return false;
        }
    }
    for i in vec![start[0],start[1],end[0], end[1]] {
        if i > 7 {
            println!("illegal move, out of bounds");
            return false;
        }
    }
    // println!("hej, legal move, match {}, start: {:?}", board.board[start[0]][start[1]].abs(), start);
    let legal = match (board.board[start[0]][start[1]]).abs() {
        1 => legal_pawn_move(&mut board.board, &board.board_history, start, end, player, check_checker),
        2 => legal_knight_move(&board.board, start, end, player, check_checker),
        3 => legal_bishop_move(&board.board, start, end, player, check_checker),
        4 => legal_rook_move(&board.board, start, end, player, check_checker),
        5 => legal_queen_move(&board.board, start, end, player, check_checker),
        6 => legal_king_move(&mut board.board, start, end, player, check_checker, &board.castle_pieces),
        _ => false,
    };
    legal
}

fn legal_pawn_move(board: &mut Vec<Vec<i8>>, bh: &Vec<Vec<Vec<i8>>>, start: &Vec<usize>, end: &Vec<usize>, player: i8, check_checker: bool) -> bool {
    let mut test_board = board.clone();
    if start[1] != end[1] { // side move
        // println!("hej, legal move, pawn, side");
        if (start[1] as i8 - end[1] as i8).abs() != 1 {
            return false;
        }
        if start[0] as i8 + player != end[0] as i8 {
            return false;
        }
        if board[end[0]][end[1]] * player < 0 { // capture opponent 
            test_board[start[0]][start[1]] = 0;
            test_board[end[0]][end[1]] = player;
            if !check_checker {
                return true;
            } else if !player_in_check(&test_board, player) {
                return true;
            }
        } else if board[end[0]][end[1]] == 0 && (start[0] == 3 || start[0] == 4) { // En passant
            if bh.len() == 0 {
                return false;
            }
            let last = bh.last().unwrap();
            if last[start[0]][end[1]] != 0 && board[start[0]][end[1]] != player * -1 {
                return false;
            }
            if last[(start[0] as i8 + 2 * player) as usize][end[1]] != player * -1 {
                return false;
            }
            if board[(start[0] as i8 + 2 * player) as usize][end[1]] != 0 {
                return false;
            }
            test_board[start[0]][start[1]] = 0;
            test_board[start[0]][end[1]] = 0;
            test_board[end[0]][end[1]] = player;
            if !check_checker {
                return true;
            } else if !player_in_check(&test_board, player) {
                board[start[0]][end[1]] = 0;
                return true;
            }
        } else {
            return false;
        }
    } else if (start[0] as i8 - end[0] as i8).abs() <= 2 { // move forward
        // println!("hej, legal move, pawn, forward");
        if board[end[0]][end[1]] != 0 {
            return false;
        }
        if board[(start[0] as i8 + player) as usize][start[1]] != 0 {
            return false;
        }
        if (end[0] as i8 - start[0] as i8) * player <= 0 {
            return false;
        }
        if (start[0] as i8 - end[0] as i8).abs() == 2 {
            // println!("hej, legal move, pawn, forward, 2");
            if start[0] != 1 && start[0] != 6 {
                return false;
            }
        }
        test_board[end[0]][end[1]] = test_board[start[0]][start[1]];
        test_board[start[0]][start[1]] = 0;
        if !check_checker {
            return true;
        } else if !player_in_check(&test_board, player) {
            return true;
        }
    }
    // println!("hej, legal move, pawn, false");
    false
}

fn legal_knight_move(board: &Vec<Vec<i8>>, start: &Vec<usize>, end: &Vec<usize>, player: i8, check_checker: bool) -> bool {
    let mut test_board = board.clone();
    let a = (start[0] as i8 - end[0] as i8).abs();
    let b = (start[1] as i8 - end[1] as i8).abs();
    if (a == 1 || b == 1) && (a == 2 || b == 2) {
        test_board[end[0]][end[1]] = test_board[start[0]][start[1]];
        test_board[start[0]][start[1]] = 0;
        if !check_checker {
            return true;
        } else if !player_in_check(&test_board, player) {
            return true;
        }
    }
    false
}

fn legal_bishop_move(board: &Vec<Vec<i8>>, start: &Vec<usize>, end: &Vec<usize>, player: i8, check_checker: bool) -> bool {
    let mut test_board = board.clone();
    let a = (start[0] as i8 - end[0] as i8).abs();
    let b = (start[1] as i8 - end[1] as i8).abs();
    if a == b {
        let x = if start[0] < end[0] { 1 } else { -1 };
        let y = if start[1] < end[1] { 1 } else { -1 };
        let mut i = (start[0] as i8 + x) as usize;
        let mut j = (start[1] as i8 + y) as usize;
        while i != end[0] {
            if test_board[i][j] != 0 {
                return false;
            }
            i = (i as i8 + x) as usize;
            j = (j as i8 + y) as usize;
        }
        test_board[end[0]][end[1]] = test_board[start[0]][start[1]];
        test_board[start[0]][start[1]] = 0;
        if !check_checker {
            return true;
        } else if !player_in_check(&test_board, player) {
            return true;
        }
    }

    false
}

fn legal_rook_move(board: &Vec<Vec<i8>>, start: &Vec<usize>, end: &Vec<usize>, player: i8, check_checker: bool) -> bool {
    // println!("hej, legal move, rook");
    let mut test_board = board.clone();
    let mut x = 0;
    let mut y = 0;
    if start[0] == end[0] && start[1] != end[1] {
        x = if start[1] < end[1] { 1 } else { -1 }
    } else if start[1] == end[1] && start[0] != end[0] {
        y = if start[0] < end[0] { 1 } else { -1 }
    } else {
        return false;
    }
    // println!("hej, legal move, rook, after if");
    let mut i = (start[0] as i8 + y) as usize;
    let mut j = (start[1] as i8 + x) as usize;
    while i != end[0] || j != end[1] {
        if test_board[i][j] != 0 {
            return false;
        }
        i = (i as i8 + y) as usize;
        j = (j as i8 + x) as usize;
    }
    // println!("hej, legal move, rook, after while");
    test_board[end[0]][end[1]] = test_board[start[0]][start[1]];
    test_board[start[0]][start[1]] = 0;
    if !check_checker {
        return true;
    } else if !player_in_check(&test_board, player) {
        return true;
    }
    false
}

fn legal_queen_move(board: &Vec<Vec<i8>>, start: &Vec<usize>, end: &Vec<usize>, player: i8, check_checker: bool) -> bool {
    let mut test_board = board.clone();
    let a = (start[0] as i8 - end[0] as i8).abs();
    let b = (start[1] as i8 - end[1] as i8).abs();
    if a != b && (a != 0 && b != 0) {
        return false;
    }
    let mut x = 0;
    let mut y = 0;
    if start[0] == end[0] && start[1] != end[1] {
        x = if start[1] < end[1] { 1 } else { -1 }
    } else if start[1] == end[1] && start[0] != end[0] {
        y = if start[0] < end[0] { 1 } else { -1 }
    } else {
        x = if start[1] < end[1] { 1 } else { -1 };
        y = if start[0] < end[0] { 1 } else { -1 };
    }
    let mut i = (start[0] as i8 + y) as usize;
    let mut j = (start[1] as i8 + x) as usize;
    while i != end[0] || j != end[1] {
        if test_board[i][j] != 0 {
            return false;
        }
        i = (i as i8 + y) as usize;
        j = (j as i8 + x) as usize;
    }
    test_board[end[0]][end[1]] = test_board[start[0]][start[1]];
    test_board[start[0]][start[1]] = 0;
    if !check_checker {
        return true;
    } else if !player_in_check(&test_board, player) {
        return true;
    }
    false
}

fn legal_king_move(board: &mut Vec<Vec<i8>>, start: &Vec<usize>, end: &Vec<usize>, player: i8, check_checker: bool, castle_pieces: &HashSet<(usize,usize)>) -> bool {
    let mut test_board = board.clone();
    let a = (start[0] as i8 - end[0] as i8).abs();
    let b = (start[1] as i8 - end[1] as i8).abs();
    // castle
    if castle_pieces.contains(&(start[0],start[1])) && b == 2 && a == 0 {
        // println!("hej, legal move, king, castle");
        if player_in_check(&test_board, player) {
            return false;
        }
        let s;
        let e;
        let rms;
        let rme;
        if start[1] < end[1] && castle_pieces.contains(&(start[0],7)) {
            s = start[1]+1;
            e = 7;
            rms = (start[0],7);
            rme = (start[0],start[1]+1);
        } else if start[1] > end[1] && castle_pieces.contains(&(start[0],0)) {
            s = 1;
            e = start[1];
            rms = (start[0],0);
            rme = (start[0],start[1]-1);
        } else {
            return false;
        }
        for i in s..e {
            if board[start[0]][i] != 0 {
                return false;
            }
        }
        test_board[end[0]][end[1]] = test_board[start[0]][start[1]];
        test_board[start[0]][start[1]] = 0;
        test_board[rme.0][rme.1] = test_board[rms.0][rms.1];
        test_board[rms.0][rms.1] = 0;
        if !check_checker {
            return true;
        } else if !player_in_check(&test_board, player) {
            test_board[rme.0][rme.1] = test_board[end[0]][end[1]];
            test_board[end[0]][end[1]] = 0;
            if !player_in_check(&test_board, player) { // can not be in check for middle square
                board[rme.0][rme.1] = board[rms.0][rms.1];
                board[rms.0][rms.1] = 0;
                return true;
            }
        }
    }
    // normal move
    if a <= 1 && b <= 1 && a + b > 0 { 
        test_board[end[0]][end[1]] = test_board[start[0]][start[1]];
        test_board[start[0]][start[1]] = 0;
        if !check_checker {
            return true;
        } else if !player_in_check(&test_board, player) {
            return true;
        }
    }
    false
}

fn player_in_check(board: &Vec<Vec<i8>>, player: i8) -> bool {
    let mut king = vec![];
    for i in 0..8 {
        for j in 0..8 {
            if board[i][j] == 6 * player {
                king = vec![i, j];
            }
        }
    }

    if king.len() == 0 {
        println!("hej, player in check, no king");
        return false;
    }

    // println!();
    // print_board(board);

    for i in 0..8 {
        for j in 0..8 {
            if board[i][j] * player * -1 > 0 {
                let start = vec![i, j];
                let end = vec![king[0], king[1]];
                let mut b = Board { 
                    board: board.clone(), 
                    board_history: vec![], 
                    board_repeated: HashMap::new(),
                    turn: player*-1, 
                    fifty_move_rule: 0,
                    castle_pieces: vec![(0,0),(0,4),(0,7), (7,0),(7,4),(7,7)].into_iter().collect::<HashSet<(usize,usize)>>(),
                    move_history: vec![]};
                if legal_move(&mut b, &start, &end, player * -1, false) {
                    // println!("hej, player in check, true, player: {}, start: {:?}, end: {:?}", player, start, end);
                    return true;
                }
            }
        }
    }

    false
}

fn won(board: &Vec<Vec<i8>>, board_history: &Vec<Vec<Vec<i8>>>, player: i8) -> bool {
    if !player_in_check(board, player) {
        // println!("hej, won, player not in check");
        return false;
    }
    let mut players_piece_positions = vec![];
    for i in 0..8 {
        for j in 0..8 {
            if board[i][j] * player > 0 {
                players_piece_positions.push(vec![i, j]);
            }
        }
    }

    let b = Board { 
        board: board.clone(), 
        board_history: board_history.clone(), 
        board_repeated: HashMap::new(),
        turn: player, 
        fifty_move_rule: 0,
        castle_pieces: vec![(0,0),(0,4),(0,7), (7,0),(7,4),(7,7)].into_iter().collect::<HashSet<(usize,usize)>>(),
        move_history: vec![]};
    // print_board(&b.board);
    // println!("hej, won, search for a legal opponent move");
    for start in players_piece_positions {
        // println!("hej, won, start: {:?}", start);
        for i in 0..8 {
            for j in 0..8 {
                let end = vec![i, j];
                let mut bb = b.clone();
                let s = start.clone();
                // println!("hej, won, s: {:?}, end: {:?}", s, end);
                if legal_move(&mut bb, &s, &end, player, true) {
                    // println!("hej, won, no legal move, false");
                    return false;
                }
            }
        }
    }

    true
}

fn draw(board: &Vec<Vec<i8>>, board_history: &Vec<Vec<Vec<i8>>>, board_repeated: &mut HashMap<Vec<Vec<i8>>, i8>, player: i8, fifty_move_rule: i16) -> bool {
    if fifty_move_rule >= 50*2 {
        println!("hej, draw, 50 moves since last capture");
        return true;
    }
    if board_repeated.contains_key(board) {
        board_repeated.insert(board.clone(), board_repeated.get(board).unwrap() + 1);
        if *board_repeated.get(board).unwrap() == 3 {
            println!("hej, draw, same board 3 times");
            return true;
        }
    } else {
        board_repeated.insert(board.clone(), 1);
    }
    if player_in_check(board, player) {
        // println!("hej, draw, player is in check");
        return false;
    }
    let no_opponent_moves = no_opponent_moves(board, board_history, player);
    let only_kings = only_kings(board);
    no_opponent_moves || only_kings
}

fn no_opponent_moves(board: &Vec<Vec<i8>>, board_history: &Vec<Vec<Vec<i8>>>, player: i8) -> bool {
    let mut players_piace_positions = vec![];
    for i in 0..8 {
        for j in 0..8 {
            if board[i][j] * player > 0 {
                players_piace_positions.push(vec![i, j]);
            }
        }
    }

    let b = Board { 
        board: board.clone(), 
        board_history: board_history.clone(),
        board_repeated: HashMap::new(), 
        turn: player, 
        fifty_move_rule: 0,
        castle_pieces: vec![(0,0),(0,4),(0,7), (7,0),(7,4),(7,7)].into_iter().collect::<HashSet<(usize,usize)>>(),
        move_history: vec![]};
    // print_board(&b.board);
    for start in players_piace_positions {
        for i in 0..8 {
            for j in 0..8 {
                let end = vec![i, j];
                let mut bb = b.clone();
                let s = start.clone();
                if legal_move(&mut bb, &s, &end, player, true) {
                    return false;
                }
            }
        }
    }

    true
}

fn only_kings(board: &Vec<Vec<i8>>) -> bool {
    let mut count = 0;
    for i in 0..8 {
        for j in 0..8 {
            if board[i][j] != 0 {
                count += 1;
            }
        }
    }
    count == 2
}

fn print_board(board: &Vec<Vec<i8>>) {
    for r in board.iter().rev() {
        for i in r {
            if *i < 0 {
                print!("{} ", i);
            } else {
                print!(" {} ", i);
            }
        }
        println!();
    }
}



fn format_move(board: Vec<Vec<i8>>, start_row: usize, start_col: usize, end_row: usize, end_col: usize, promote_to: i8) -> String {
    let mut s = String::new();
    let piece = board[start_row][start_col];
    let piece_name = match piece.abs() {
        1 => "P",
        2 => "N",
        3 => "B",
        4 => "R",
        5 => "Q",
        6 => "K",
        _ => "",
    };

    let start = format!("{}{}-", (start_col + 97) as u8 as char, start_row);
    let end = format!("{}{}", (end_col + 97) as u8 as char, end_row);

    if piece.abs() == 1 && (end_row == 0 || end_row == 7){
        let promote = match promote_to.abs() {
            2 => "N",
            3 => "B",
            4 => "R",
            5 => "Q",
            _ => "",
        };
        s.push_str(&format!("{}{}{}/{}", piece_name, start, end, promote));
    } else if piece.abs() == 6 && (start_col as i8 - end_col as i8).abs() == 2 {
        if start_col < end_col {
            s.push_str("O-O");
        } else {
            s.push_str("O-O-O");
        }
    } else {
        s.push_str(&format!("{}{}{}", piece_name, start, end));
    }

    s
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_board() {
        let board = Board::new_board(1).board;
        assert_eq!(board[0], vec![4,2,3,5,6,3,2,4]);
        assert_eq!(board[1], vec![1; 8]);
        assert_eq!(board[2], vec![0; 8]);
        assert_eq!(board[3], vec![0; 8]);
        assert_eq!(board[4], vec![0; 8]);
        assert_eq!(board[5], vec![0; 8]);
        assert_eq!(board[6], vec![-1; 8]);
        assert_eq!(board[7], vec![-4,-2,-3,-5,-6,-3,-2,-4]);
    }

    #[test]
    fn test_update_board() {
        let mut board = Board::new_board(1);
        let start = vec![1, 0];
        let end = vec![2, 0];
        assert_eq!(board.update_board(start, end, 0), 0);
        assert_eq!(board.board[1], vec![0,1,1,1,1,1,1,1]);
        assert_eq!(board.board[2], vec![1,0,0,0,0,0,0,0]);
    }

    #[test]
    fn wrong_turn() {
        let mut board = Board::new_board(1);
        let start = vec![6, 0];
        let end = vec![5, 0];
        assert_eq!(board.update_board(start, end, 0), 2);
    }

    #[test]
    fn legal_pawn_move_1() {
        let mut board = Board::new_board(1);
        let start = vec![1, 0];
        let end = vec![3, 0];
        assert_eq!(board.update_board(start, end, 0), 0);
        assert_eq!(board.board[1], vec![0,1,1,1,1,1,1,1]);
        assert_eq!(board.board[2], vec![0,0,0,0,0,0,0,0]);
        assert_eq!(board.board[3], vec![1,0,0,0,0,0,0,0]);
    }

    #[test]
    fn legal_pawn_move_2() {
        let mut board = Board::new_board(1);
        assert_eq!(board.update_board(vec![1, 0], vec![2, 0], 0), 0);
        assert_eq!(board.update_board(vec![6, 0], vec![4, 0], 0), 0);
        assert_eq!(board.update_board(vec![1, 1], vec![2, 1], 0), 0);
        assert_eq!(board.update_board(vec![6, 1], vec![5, 1], 0), 0);
        assert_eq!(board.update_board(vec![2, 1], vec![4, 1], 0), -2);
    }

    #[test]
    fn legal_pawn_move_3() {
        let mut board = Board::new_board(1);
        assert_eq!(board.update_board(vec![1, 0], vec![3, 0], 0), 0);
        assert_eq!(board.update_board(vec![6, 0], vec![4, 0], 0), 0);
        assert_eq!(board.update_board(vec![1, 1], vec![3, 1], 0), 0);
        assert_eq!(board.update_board(vec![6, 2], vec![4, 2], 0), 0);
        assert_eq!(board.update_board(vec![3, 1], vec![4, 0], 0), 0);
        assert_eq!(board.update_board(vec![6, 1], vec![4, 1], 0), 0);
        assert_eq!(board.update_board(vec![4, 0], vec![5, 1], 0), 0);
        assert_eq!(board.board[0], vec![ 4, 2, 3, 5, 6, 3, 2, 4]);
        assert_eq!(board.board[1], vec![ 0, 0, 1, 1, 1, 1, 1, 1]);
        assert_eq!(board.board[2], vec![ 0, 0, 0, 0, 0, 0, 0, 0]);
        assert_eq!(board.board[3], vec![ 1, 0, 0, 0, 0, 0, 0, 0]);
        assert_eq!(board.board[4], vec![ 0, 0,-1, 0, 0, 0, 0, 0]);
        assert_eq!(board.board[5], vec![ 0, 1, 0, 0, 0, 0, 0, 0]);
        assert_eq!(board.board[6], vec![ 0, 0, 0,-1,-1,-1,-1,-1]);
        assert_eq!(board.board[7], vec![-4,-2,-3,-5,-6,-3,-2,-4]);
    }

    #[test]
    fn legal_pawn_move_4() {
        let mut board = Board::new_board(1);
        board.board[0] = vec![0,0,0,0,0, 0,0,0];
        board.board[1] = vec![0,0,0,6,1, 0,0,0];
        board.board[2] = vec![0,0,0,0,0, 0,0,0];
        board.board[3] = vec![0,0,0,0,0,-3,0,0];
        assert_eq!(board.update_board(vec![1, 4], vec![2, 4], 0), 0);
    }

    #[test]
    fn legal_knight_move_1() {
        let mut board = Board::new_board(1);
        assert_eq!(board.update_board(vec![0, 1], vec![2, 0], 0), 0);
        assert_eq!(board.board[0], vec![4,0,3,5,6,3,2,4]);
        assert_eq!(board.board[1], vec![1; 8]);
        assert_eq!(board.board[2], vec![2,0,0,0,0,0,0,0]);
    }

    #[test]
    fn legal_knight_move_2() {
        let mut board = Board::new_board(1);
        assert_eq!(board.update_board(vec![0, 1], vec![2, 0], 0), 0);
        assert_eq!(board.update_board(vec![7, 1], vec![5, 2], 0), 0);
        assert_eq!(board.update_board(vec![2, 0], vec![3, 2], 0), 0);
        assert_eq!(board.update_board(vec![5, 2], vec![3, 1], 0), 0);
        assert_eq!(board.update_board(vec![3, 2], vec![1, 3], 0), -2);
    }

    #[test]
    fn legal_bishop_move_1() {
        let mut board = Board::new_board(1);
        assert_eq!(board.update_board(vec![1, 3], vec![2, 3], 0), 0);
        assert_eq!(board.update_board(vec![6, 3], vec![5, 3], 0), 0);
        assert_eq!(board.update_board(vec![0, 2], vec![4, 6], 0), 0);
        assert_eq!(board.board[0], vec![4,2,0,5,6,3,2,4]);
        assert_eq!(board.board[1], vec![1,1,1,0,1,1,1,1]);
        assert_eq!(board.board[2], vec![0,0,0,1,0,0,0,0]);
        assert_eq!(board.board[3], vec![0,0,0,0,0,0,0,0]);
        assert_eq!(board.board[4], vec![0,0,0,0,0,0,3,0]);
    }

    #[test]
    fn legal_bishop_move_2() {
        let mut board = Board::new_board(1);
        assert_eq!(board.update_board(vec![0, 2], vec![4, 6], 0), -2);
    }

    #[test]
    fn legal_bishop_move_3() {
        let mut board = Board::new_board(1);
        assert_eq!(board.update_board(vec![1, 4], vec![2, 4], 0), 0);
        assert_eq!(board.update_board(vec![6, 3], vec![5, 3], 0), 0);
        assert_eq!(board.update_board(vec![0, 5], vec![4, 1], 0), 0);
        assert_eq!(board.board[0], vec![4,2,3,5,6,0,2,4]);
        assert_eq!(board.board[1], vec![1,1,1,1,0,1,1,1]);
        assert_eq!(board.board[2], vec![0,0,0,0,1,0,0,0]);
        assert_eq!(board.board[3], vec![0,0,0,0,0,0,0,0]);
        assert_eq!(board.board[4], vec![0,3,0,0,0,0,0,0]);
    }

    #[test]
    fn legal_rook_move_1() {
        let mut board = Board::new_board(1);
        assert_eq!(board.update_board(vec![1, 0], vec![3, 0], 0), 0);
        assert_eq!(board.update_board(vec![6, 0], vec![5, 0], 0), 0);
        print_board(&board.board);
        assert_eq!(board.update_board(vec![0, 0], vec![2, 0], 0), 0);
        print_board(&board.board);
        assert_eq!(board.board[0], vec![0,2,3,5,6,3,2,4]);
        assert_eq!(board.board[1], vec![0,1,1,1,1,1,1,1]);
        assert_eq!(board.board[2], vec![4,0,0,0,0,0,0,0]);
        assert_eq!(board.board[3], vec![1,0,0,0,0,0,0,0]);
    }

    #[test]
    fn legal_rook_move_2() {
        let mut board = Board::new_board(-1);
        board.board[7] = vec![-4, 0, 0, 0, 0,-6, 0, 0];
        board.board[6] = vec![ 0, 0, 0, 0, 0, 0, 0, 0];
        board.board[5] = vec![-3, 0, 0,-1, 0, 0,-1, 0];
        board.board[4] = vec![-1, 0,-4, 0, 0, 0, 0,-1];
        board.board[3] = vec![ 1, 0, 4, 0, 0, 0, 0, 0];
        board.board[2] = vec![ 0, 1, 0, 0, 0, 0, 0, 0];
        board.board[1] = vec![ 0, 0, 0, 0,-4, 0, 0, 0];
        board.board[0] = vec![ 0, 0, 0, 0, 0, 0, 0, 6];
        print_board(&board.board);
        assert_eq!(board.update_board(vec![4, 2], vec![4, 5], 0), 0);
    }

    #[test]
    fn legal_queen_move_1() {
        let mut board = Board::new_board(1);
        assert_eq!(board.update_board(vec![1, 4], vec![2, 4], 0), 0);
        assert_eq!(board.update_board(vec![6, 3], vec![5, 3], 0), 0);
        assert_eq!(board.update_board(vec![0, 3], vec![3, 6], 0), 0);
        assert_eq!(board.update_board(vec![6, 4], vec![5, 4], 0), 0);
        assert_eq!(board.update_board(vec![3, 6], vec![3, 2], 0), 0);
        assert_eq!(board.board[0], vec![4,2,3,0,6,3,2,4]);
        assert_eq!(board.board[1], vec![1,1,1,1,0,1,1,1]);
        assert_eq!(board.board[2], vec![0,0,0,0,1,0,0,0]);
        assert_eq!(board.board[3], vec![0,0,5,0,0,0,0,0]);
    }

    #[test]
    fn legal_king_move_1() {
        let mut board = Board::new_board(1);
        assert_eq!(board.update_board(vec![1, 4], vec![2, 4], 0), 0);
        assert_eq!(board.update_board(vec![6, 4], vec![5, 4], 0), 0);
        assert_eq!(board.update_board(vec![0, 4], vec![1, 4], 0), 0);
        assert_eq!(board.board[0], vec![4,2,3,5,0,3,2,4]);
        assert_eq!(board.board[1], vec![1,1,1,1,6,1,1,1]);
        assert_eq!(board.board[2], vec![0,0,0,0,1,0,0,0]);
    }

    #[test]
    fn legal_king_move_2() {
        let mut board = Board::new_board(1);
        board.board[0] = vec![4,0,0,0,6,3,2,4];
        assert_eq!(board.board[1], vec![1,1,1,1,1,1,1,1]);
        assert_eq!(board.update_board(vec![0, 4], vec![0, 2], 0), 0);
        assert_eq!(board.board[0], vec![0,0,6,4,0,3,2,4]);
    }

    #[test]
    fn legal_king_move_3() {
        let mut board = Board::new_board(1);
        board.board[0] = vec![4,0,0,0,6,3,2,4];
        assert_eq!(board.board[1], vec![1,1,1,1,1,1,1,1]);
        assert_eq!(board.update_board(vec![0, 0], vec![0, 1], 0), 0);
        assert_eq!(board.update_board(vec![6, 4], vec![5, 4], 0), 0);
        assert_eq!(board.update_board(vec![0, 1], vec![0, 0], 0), 0);
        assert_eq!(board.update_board(vec![5, 4], vec![4, 4], 0), 0);
        assert_eq!(board.update_board(vec![0, 4], vec![0, 2], 0), -2);
    }

    #[test]
    fn legal_castle_move_1() {
        let mut board = Board::new_board(1);
        board.board[0] = vec![4,0,0,0,6,3,2,4];
        board.board[7] = vec![0,0,0,0,-6,0,0,-4];
        assert_eq!(board.update_board(vec![0, 4], vec![0, 2], 0), 0);
        assert_eq!(board.update_board(vec![7, 4], vec![7, 6], 0), 0);
    }

    #[test]
    fn legal_castle_move_2() {
        let mut board = Board::new_board(1);
        board.board[0] = vec![4,0,0,0,6,3,2,4];
        board.board[1] = vec![0,0,0,0,0,0,0,0];
        board.board[6] = vec![0,0,0,0,0,0,0,0];
        board.board[7] = vec![0,0,0,-5,-6,0,0,-4];
        assert_eq!(board.update_board(vec![0, 4], vec![0, 2], 0), -2);
    }

    #[test]
    fn player_in_check_1() {
        let board = Board::new_board(1);
        assert_eq!(player_in_check(&board.board, 1), false);
    }

    #[test]
    fn player_in_check_2() {
        let mut board = Board::new_board(1);
        board.board[0] = vec![0,6,-5,0,0,0,0,0];
        board.board[1] = vec![0,0, 0,0,0,0,0,0];
        print_board(&board.board);
        assert_eq!(player_in_check(&board.board, 1), true);
    }

    #[test]
    fn player_in_check_3() {
        let mut board = Board::new_board(1);
        board.board[2] = vec![0,0,0,0,0,-2,0,0];
        print_board(&board.board);
        assert_eq!(player_in_check(&board.board, 1), true);
    }
    
    #[test]
    fn player_in_check_4() {
        let mut board = Board::new_board(-1);
        board.board[7] = vec![-4,-2,0,0,0,-3,0,0]; // -4 -2  0  0  0 -3  0  0
        board.board[6] = vec![-1,0,0,-3,-6,-1,-1,0]; // -1  0  0 -3 -6 -1 -1  0
        board.board[5] = vec![0,-1,-1,0,-1,0,0,-4]; //  0 -1 -1  0 -1  0  0 -4
        board.board[4] = vec![1,0,0,-1,0,0,3,-1]; //  1  0  0 -1  0  0  3 -1
        board.board[3] = vec![5,1,1,0,0,0,1,1]; //  5  1  1  0  0  0  1  1
        board.board[2] = vec![0,0,0,6,0,0,0,0]; //  0  0  0  6  0  0  0  0
        board.board[1] = vec![4,0,2,0,1,1,2,4]; //  4  0  2  0  1  1  2  4
        board.board[0] = vec![0,0,0,0,0,3,0,0]; //  0  0  0  0  0  3  0  0
        println!();
        print_board(&board.board);
        assert_eq!(player_in_check(&board.board, 1), false);
        assert_eq!(player_in_check(&board.board, -1), true);
        assert_eq!(board.update_board(vec![6, 5], vec![5, 5], 0), 0);
        assert_eq!(player_in_check(&board.board, -1), false);
    }

    #[test]
    fn won_1() {
        let board = Board::new_board(1);
        assert_eq!(won(&board.board, &board.board_history, 1), false);
    }

    #[test]
    fn won_2() {
        let mut board = Board::new_board(-1);
        board.board[0] = vec![0,6,-5,0,0,0,0,0];
        board.board[1] = vec![0,0,0,0,0,0,0,-4];
        board.board[2] = vec![-3,0,0,0,0,0,0,0];
        assert_eq!(won(&board.board, &board.board_history, 1), true);
    }

    #[test]
    fn won_3() {
        let mut board = Board::new_board(1);
        board.board[2] = vec![0,0,-2,0,0,0,0,0];
        print_board(&board.board);
        assert_eq!(won(&board.board, &board.board_history, 1), false);
    }

    #[test]
    fn won_4() {
        let mut board = Board::new_board(-1);
        board.board[0] = vec![ 0,6,-5, 0, 0,3,0,0];
        board.board[1] = vec![ 0,0, 0, 0,-4,0,0,0];
        board.board[2] = vec![-3,0, 0,-6, 0,0,0,0];
        board.board[3] = vec![ 0,0, 0, 0, 0,0,0,0];
        board.board[4] = vec![ 0,0, 0, 0, 0,0,0,0];
        board.board[5] = vec![ 0,0, 0, 0, 0,0,0,0];
        board.board[6] = vec![ 0,0, 0, 0, 0,0,0,0];
        board.board[7] = vec![ 0,0, 0, 0, 0,0,0,0];
        assert_eq!(won(&board.board, &board.board_history, 1), true);
    }

    #[test]
    fn draw_1() {
        let board = Board::new_board(1);
        assert_eq!(draw(&board.board, &board.board_history, &mut HashMap::new(), 1, 0), false);
    }

    #[test]
    fn draw_2() {
        let mut board = Board::new_board(-1);
        board.board[0] = vec![6,0,0,0,0,0,0,0];
        board.board[1] = vec![0,0,-5,0,0,0,0,0];
        assert_eq!(draw(&board.board, &board.board_history, &mut HashMap::new(), 1, 0), true);
    }

}