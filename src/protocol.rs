// https://github.com/INDA25PlusPlus/chesstp-spec
// https://www.chessprogramming.org/Forsyth-Edwards_Notation

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
