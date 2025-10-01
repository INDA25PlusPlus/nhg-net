// https://github.com/INDA25PlusPlus/chesstp-spec
// https://www.chessprogramming.org/Forsyth-Edwards_Notation

use hermanha_chess::pieces::PieceType;
use hermanha_chess::board::{Position, Board};
use crate::helper::{board_to_fen};

pub struct MoveMsg {
    pub move_str: String,   // "E2E40"
    pub game_state: String, // "0-0"
    pub fen: String,        // "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR"
}

/*
"ChessMOVE:E2E40:0-0:rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR:00000000000000000000000000000000000000000000000000000000000000"
*/
impl MoveMsg {
    pub fn serialize(&self) -> String {
        let mut msg = format!(
            "ChessMOVE:{}:{}:{}:",
            self.move_str, self.game_state, self.fen
        );
        println!("Serialized msg: {}", msg);
        let padding_len = 128 - msg.len();
        msg.push_str(&"0".repeat(padding_len));
        msg
    }
    pub fn deserialize(raw: &str) -> Option<Self> {
        println!("Raw msg to deserialize: {}", raw);
        let parts: Vec<&str> = raw.split(':').collect();
        if parts.len() < 4 || !raw.starts_with("ChessMOVE") {
            return None;
        }
        Some(MoveMsg {
            move_str: parts[1].to_string(),
            game_state: parts[2].to_string(),
            fen: parts[3].to_string(),
        })
    }
}

/// Convert a square string like "E2" to a Position
pub fn square_to_position(sq: &str) -> Option<hermanha_chess::Position> {
    if sq.len() != 2 { return None; }
    let mut chars = sq.chars();
    let col = chars.next()? as u8;
    let row = chars.next()?.to_digit(10)? as i8 - 1;
    let col_idx = (col as i8) - ('A' as i8);
    Some(hermanha_chess::Position::new(row, col_idx))
}
/// Convert a Position to a square string like "E2"
pub fn position_to_square(pos: hermanha_chess::Position) -> String {
    let file = (b'A' + pos.col as u8) as char;
    let rank = (pos.row + 1).to_string();
    format!("{}{}", file, rank)
}

pub fn board_move_from_string(input: &str, board: &Board) -> Option<MoveMsg> {
    // Input expected like "E2E4" or "E7E8Q"
    if input.len() < 4 { return None; }

    let from_sq = &input[0..2];
    let to_sq = &input[2..4];
    let promo = if input.len() == 5 {
        match &input[4..5] {
            "Q" => Some(PieceType::Queen),
            "R" => Some(PieceType::Rook),
            "B" => Some(PieceType::Bishop),
            "N" => Some(PieceType::Knight),
            _ => None,
        }
    } else { None };

    let from = square_to_position(from_sq)?;
    let to = square_to_position(to_sq)?;

    let fen = board_to_fen(board);

    Some(MoveMsg {
        move_str: input.to_string(),
        game_state: "0-0".to_string(),
        fen,
    })
}
