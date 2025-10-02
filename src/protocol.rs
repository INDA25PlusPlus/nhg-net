// https://github.com/INDA25PlusPlus/chesstp-spec
// https://www.chessprogramming.org/Forsyth-Edwards_Notation
pub enum ProtocolMsg {
    Move(MoveMsg),
    Quit(QuitMsg),
}
impl ProtocolMsg {
    pub fn serialize(&self) -> String {
        match self {
            ProtocolMsg::Move(m) => m.serialize(),
            ProtocolMsg::Quit(q) => q.serialize(),
        }
    }
    pub fn deserialize(raw: &str) -> Option<Self> {
        if raw.starts_with("ChessMOVE") {
            MoveMsg::deserialize(raw).map(ProtocolMsg::Move)
        } else if raw.starts_with("ChessQUIT") {
            QuitMsg::deserialize(raw).map(ProtocolMsg::Quit)
        } else {
            None
        }
    }
}

pub struct QuitMsg {
    pub reason: String, // optional message ("desync", "panic", "user quit", etc.)
}

impl QuitMsg {
    pub fn serialize(&self) -> String {
        let mut msg = format!("ChessQUIT:{}:", self.reason);
        let padding_len = 128 - msg.len();
        msg.push_str(&"0".repeat(padding_len));
        msg
    }

    pub fn deserialize(raw: &str) -> Option<Self> {
        if !raw.starts_with("ChessQUIT") {
            return None;
        }
        let parts: Vec<&str> = raw.split(':').collect();
        if parts.len() < 2 {
            return None;
        }
        Some(QuitMsg {
            reason: parts[1].to_string(),
        })
    }
}

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
        //println!("Serialized msg: {}", msg);
        let padding_len = 128 - msg.len();
        msg.push_str(&"0".repeat(padding_len));
        msg
    }
    pub fn deserialize(raw: &str) -> Option<Self> {
        //println!("Raw msg to deserialize: {}", raw);
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
