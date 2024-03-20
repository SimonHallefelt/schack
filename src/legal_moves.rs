pub fn get_all_legal_moves(board: &Vec<Vec<i8>>, board_history: Vec<Vec<Vec<i8>>>, player: i8) -> Vec<Vec<usize>> {
    let mut legal_moves = Vec::new();
    let mut pieces = Vec::new();
    for i in 0..8 {
        for j in 0..8 {
            if board[i][j] * player > 0{
                pieces.push((i, j));
            }
        }
    }

    for piece in pieces {
        let piece_legal_moves = get_legal_moves(board, &board_history, piece, player);
        for legal_move in piece_legal_moves {
            legal_moves.push(legal_move);
        }
    }

    println!("Number of legal moves: {}", legal_moves.len());
    legal_moves
}

fn get_legal_moves(board: &Vec<Vec<i8>>, board_history: &Vec<Vec<Vec<i8>>>, start: (usize, usize), player: i8) -> Vec<Vec<usize>> {
    let legal_moves = match board[start.0][start.1].abs() {
        6 => legal_king_moves(board, start, player),
        5 => legal_queen_moves(board, start, player),
        4 => legal_rook_moves(board, start, player),
        3 => legal_bishop_moves(board, start, player),
        2 => legal_knight_moves(board, start, player),
        1 => legal_pawn_moves(board, board_history, start, player),
        _ => Vec::new()
    };

    legal_moves
}

fn legal_king_moves(board: &Vec<Vec<i8>>, start: (usize, usize), player: i8) -> Vec<Vec<usize>> {
    let mut possible_moves = vec![];
    for i in start.0 as i8 - 1 .. start.0 as i8 + 2 {
        for j in start.1 as i8 - 1 .. start.1 as i8 + 2 {
            if i < 0 || i > 7 || j < 0 || j > 7 {
                continue;
            }
            if board[i as usize][j as usize] * player <= 0 {
                possible_moves.push((i as usize,j as usize));
            }
        }
    }

    let mut legal_moves = Vec::new();
    for m in possible_moves {
        let mut b = board.clone();
        b[m.0][m.1] = b[start.0][start.1];
        b[start.0][start.1] = 0;
        if !in_check(&b, player) {
            legal_moves.push(vec![start.0, start.1, m.0, m.1]);
        }
    }
    // println!("hej, king, amount of legal moves: {}", legal_moves.len());
    legal_moves
}

fn legal_queen_moves(board: &Vec<Vec<i8>>, start: (usize, usize), player: i8) -> Vec<Vec<usize>> {
    let dir = vec![(1,0),(-1,0),(0,1),(0,-1), (1,1),(-1,1),(-1,-1),(1,-1)];
    let legal_moves = possible_direction_moves(board, start, player, dir);
    // println!("hej, queen, amount of legal moves: {}", legal_moves.len());
    legal_moves
}

fn legal_rook_moves(board: &Vec<Vec<i8>>, start: (usize, usize), player: i8) -> Vec<Vec<usize>> {
    let dir = vec![(1,0),(-1,0),(0,1),(0,-1)];
    let legal_moves = possible_direction_moves(board, start, player, dir);
    // println!("hej, rook, amount of legal moves: {}", legal_moves.len());
    legal_moves
}

fn legal_bishop_moves(board: &Vec<Vec<i8>>, start: (usize, usize), player: i8) -> Vec<Vec<usize>> {
    let dir = vec![(1,1),(-1,1),(-1,-1),(1,-1)];
    let legal_moves = possible_direction_moves(board, start, player, dir);
    // println!("hej, bishop, amount of legal moves: {}", legal_moves.len());
    legal_moves
}

fn legal_knight_moves(board: &Vec<Vec<i8>>, start: (usize, usize), player: i8) -> Vec<Vec<usize>> {
    let mut possible_moves = vec![];
    let dir = vec![(2,1),(-2,1),(-2,-1),(2,-1), (1,2),(-1,2),(-1,-2),(1,-2)];
    for d in dir {
        let a = start.0 as i8 + d.0;
        let b = start.1 as i8 + d.1;
        if a < 0 || a > 7 || b < 0 || b > 7 {
            continue;
        }
        if board[a as usize][b as usize] * player > 0 {
            continue;
        }
        possible_moves.push((a as usize, b as usize));
        if board[a as usize][b as usize] * player != 0 {
            continue;
        }
    }

    let mut legal_moves = Vec::new();
    for m in possible_moves {
        let mut b = board.clone();
        b[m.0][m.1] = b[start.0][start.1];
        b[start.0][start.1] = 0;
        if !in_check(&b, player) {
            legal_moves.push(vec![start.0, start.1, m.0, m.1]);
        }
    }
    // println!("hej, knight, amount of legal moves: {}", legal_moves.len());
    legal_moves
}

fn legal_pawn_moves(board: &Vec<Vec<i8>>, board_history: &Vec<Vec<Vec<i8>>>, start: (usize, usize), player: i8) -> Vec<Vec<usize>> {
    let mut legal_moves = Vec::new();
    let mut possible_moves = vec![];
    let start_row;
    let en_passant_row;
    let dir;
    if player == 1 {
        start_row = 1;
        en_passant_row = 4;
        dir = 1;
    } else {
        start_row = 6;
        en_passant_row = 3;
        dir = -1;
    }
    // move forward
    if board[(start.0 as i8 + dir) as usize][start.1] == 0 {
        possible_moves.push(((start.0 as i8 + dir) as usize, start.1));
        if start.0 == start_row && board[(start.0 as i8 + dir * 2) as usize][start.1] == 0{
            possible_moves.push(((start.0 as i8 + dir * 2) as usize, start.1));
        }
    }
    // attack side
    if start.1 < 7 && board[(start.0 as i8 + dir) as usize][start.1 + 1] * player < 0 {
        possible_moves.push(((start.0 as i8 + dir) as usize, start.1 + 1));
    }
    if start.1 > 0 && board[(start.0 as i8 + dir) as usize][start.1 - 1] * player < 0 {
        possible_moves.push(((start.0 as i8 + dir) as usize, start.1 - 1));
    }
    // En passant
    if start.0 == en_passant_row && board_history.len() > 0 {
        let mut temp = vec![];
        if start.1 < 7 {temp.push(start.1 + 1)}
        if start.1 > 0 {temp.push((start.1 as i8 - 1) as usize)}
        for i in temp {
            if board[start.0][start.1 + 1] == player * -1 && board[(start.0 as i8 + dir * 2) as usize][i] == 0 {
                let mut b = board_history.last().unwrap().clone();
                if b[start.0][start.1 + 1] == 0 && b[(start.0 as i8 + dir * 2) as usize][i] == player * -1 {
                    b = board.clone();
                    b[start.0][i] = 0;
                    b[(start.0 as i8 + dir) as usize][i] = b[start.0][start.1];
                    b[start.0][start.1] = 0;
                    if !in_check(&b, player) {
                        legal_moves.push(vec![start.0, start.1, (start.0 as i8 + dir) as usize, i]);
                    }
                }
            }
        }
    }

    for m in possible_moves {
        let mut b = board.clone();
        b[m.0][m.1] = b[start.0][start.1];
        b[start.0][start.1] = 0;
        if !in_check(&b, player) {
            legal_moves.push(vec![start.0, start.1, m.0, m.1]);
        }
    }
    // println!("hej, pawn, amount of legal moves: {}", legal_moves.len());
    legal_moves
}

fn in_check(board: &Vec<Vec<i8>>, player: i8) -> bool {
    false
}

fn possible_direction_moves(board: &Vec<Vec<i8>>, start: (usize, usize), player: i8, dir: Vec<(i8, i8)>) -> Vec<Vec<usize>> {
    let mut possible_moves = vec![];
    for d in dir {
        for i in 1..8 {
            let a = start.0 as i8 + d.0 * i;
            let b = start.1 as i8 + d.1 * i;
            if a < 0 || a > 7 || b < 0 || b > 7 {
                break;
            }
            if board[a as usize][b as usize] * player > 0 {
                break;
            }
            possible_moves.push((a as usize, b as usize));
            if board[a as usize][b as usize] * player != 0 {
                break;
            }
        }
    }

    let mut legal_moves = Vec::new();
    for m in possible_moves {
        let mut b = board.clone();
        b[m.0][m.1] = b[start.0][start.1];
        b[start.0][start.1] = 0;
        if !in_check(&b, player) {
            legal_moves.push(vec![start.0, start.1, m.0, m.1]);
        }
    }
    legal_moves
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_new_board() -> Vec<Vec<i8>> {
        vec![
            vec![4, 2, 3, 5, 6, 3, 2, 4],
            vec![1, 1, 1, 1, 1, 1, 1, 1],
            vec![0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0],
            vec![-1,-1,-1,-1,-1,-1,-1,-1],
            vec![-4,-2,-3,-5,-6,-3,-2,-4]
        ]
    }

    fn get_empty_board() -> Vec<Vec<i8>> {
        vec![
            vec![0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0]
        ]
    }

    #[test]
    fn get_all_legal_moves_1() {
        let board = get_new_board();
        let legal_moves = get_all_legal_moves(&board, vec![], 1);
        assert_eq!(legal_moves.len(), 20);
        let legal_moves = get_all_legal_moves(&board, vec![], -1);
        assert_eq!(legal_moves.len(), 20);
    }

    #[test]
    fn get_all_legal_moves_2() {
        let mut board = get_new_board();
        board[1] = vec![0, 0, 0, 0, 0, 0, 0, 0];
        board[6] = vec![0, 0, 0, 0, 0, 0, 0, 0];
        let legal_moves = get_all_legal_moves(&board, vec![], 1);
        println!("{:?}", legal_moves);
        assert_eq!(legal_moves.len(), 50);
        let legal_moves = get_all_legal_moves(&board, vec![], -1);
        assert_eq!(legal_moves.len(), 50);
    }
}