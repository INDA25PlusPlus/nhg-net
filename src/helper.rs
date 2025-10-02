use hermanha_chess::{Board, Color, PieceType, Position};
use crate::protocol::{MoveMsg,square_to_position, position_to_square};

pub fn print_board(board: &Board) {
    for row in (0..8).rev() {
        print!("{} ", row + 1);
        for col in 0..8 {
            let pos = Position::new(row, col);
            match board.get(pos) {
                Some(piece) => {
                    let c = match piece.piece_type {
                        PieceType::Pawn   => 'P',
                        PieceType::Knight => 'N',
                        PieceType::Bishop => 'B',
                        PieceType::Rook   => 'R',
                        PieceType::Queen  => 'Q',
                        PieceType::King   => 'K',
                    };
                    let symbol = match piece.color {
                        Color::White => c,
                        Color::Black => c.to_ascii_lowercase(),
                    };
                    print!("{} ", symbol);
                }
                None => print!(". "),
            }
        }
        println!();
    }
    println!("  A B C D E F G H");
}

pub fn apply_message_to_board(board: &mut Board, msg: &MoveMsg) -> Result<(), String> {
    let mv = &msg.move_str;
    println!("Applying move: {}", mv);
    if mv.len() != 5 { return Err("Bad move string".into()); }

    let from = square_to_position(&mv[0..2]).ok_or("Bad from-square")?;
    let to   = square_to_position(&mv[2..4]).ok_or("Bad to-square")?;
    let promo = match &mv[4..5] {
        "Q" | "q" => Some(PieceType::Queen),
        "R" | "r" => Some(PieceType::Rook),
        "B" | "b" => Some(PieceType::Bishop),
        "N" | "n" | "k" => Some(PieceType::Knight), // "k" is ambiguous, here Knight
        "0" => None,
        _ => return Err("Bad promotion char".into()),
    };

    println!("Trying move: {:?} -> {:?}", from, to);
    match board.move_piece(from, to, promo) {
        Ok(_) => {
            println!("Move applied: {:?} -> {:?}", from, to);
            //print_board(board);
            Ok(())
        }
        Err(e) => Err(format!("illegal move: {:?}", e)),
    }
}

pub fn board_move_to_message(
    from: Position,
    to: Position,
    promo: Option<PieceType>,
    board: &Board,
) -> MoveMsg {
    let mut move_str = format!(
        "{}{}",
        position_to_square(from),
        position_to_square(to)
    );
    move_str.push(match promo {
        Some(PieceType::Queen) => 'Q',
        Some(PieceType::Rook) => 'R',
        Some(PieceType::Bishop) => 'B',
        Some(PieceType::Knight) => 'N',
        None => '0',
        _ => '0',
    });

    MoveMsg {
        move_str,
        game_state: "0-0".to_string(), // change later
        fen: board_to_fen(board),      
    }
}

pub fn board_to_fen(board: &Board) -> String {
    let mut fen = String::new();
    for row in (0..8).rev() { // FEN goes 8->1
        let mut empty = 0;
        for col in 0..8 {
            match board.squares[row as usize][col as usize] {
                Some(piece) => {
                    if empty > 0 {
                        fen.push_str(&empty.to_string());
                        empty = 0;
                    }
                    let symbol = match piece.piece_type {
                        PieceType::Pawn => 'P',
                        PieceType::Knight => 'N',
                        PieceType::Bishop => 'B',
                        PieceType::Rook => 'R',
                        PieceType::Queen => 'Q',
                        PieceType::King => 'K',
                    };
                    let sym = match piece.color {
                        Color::White => symbol,
                        Color::Black => symbol.to_ascii_lowercase(),
                    };
                    fen.push(sym);
                }
                None => empty += 1,
            }
        }
        if empty > 0 {
            fen.push_str(&empty.to_string());
        }
        if row > 0 {
            fen.push('/');
        }
    }
    fen
}
