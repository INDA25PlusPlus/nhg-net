use std::io::{self, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;

use crate::protocol::{MoveMsg, board_move_from_string};
use crate::helper::{apply_message_to_board, board_move_to_message};
use hermanha_chess::{Board, Position, PieceType};

// https://github.com/INDA25PlusPlus/chesstp-spec
// https://eleftheriabatsou.hashnode.dev/tutorial-chat-application-client-server-in-rust

pub fn start_server(addr: &str) -> std::io::Result<()> {
    let listener = TcpListener::bind(addr)?;
    println!("Server listening on {}", addr);

    let board = Arc::new(Mutex::new(Board::start_pos()));

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let board_clone = Arc::clone(&board);
                thread::spawn(move || {
                    if let Err(e) = handle_client(stream, board_clone) {
                        eprintln!("Client handler error: {}", e);
                    }
                });
            }
            Err(e) => eprintln!("Connection failed: {}", e),
        }
    }
    Ok(())
}
fn spawn_receiver(mut reader: TcpStream, board: Arc<Mutex<Board>>, peer: &'static str) {
    thread::spawn(move || {
        let mut buf = [0u8; 128];
        while let Ok(n) = reader.read(&mut buf) {
            if n == 0 {
                println!("{} disconnected.", peer);
                break;
            }

            let raw = String::from_utf8_lossy(&buf[..n]);
            if let Some(msg) = MoveMsg::deserialize(&raw) {
                let mut board = board.lock().unwrap();
                if let Err(e) = apply_message_to_board(&mut board, &msg) {
                    eprintln!("Invalid move from {}: {}", peer, e);
                } else {
                    println!("Applied move from {}: {}", peer, msg.move_str);
                }
            } else {
                println!("Received invalid message from {}: {}", peer, raw);
            }
        }
    });
}
fn stdin_sender(mut writer: TcpStream, board: Arc<Mutex<Board>>) -> io::Result<()> {
    let stdin = io::stdin();
    let mut line = String::new();
    loop {
        line.clear();
        if stdin.read_line(&mut line).is_err() {
            break;
        }
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }

        let mut board = board.lock().unwrap();

        if let Some(msg) = board_move_from_string(trimmed, &board) {
            if let Err(e) = apply_message_to_board(&mut board, &msg) {
                eprintln!("Invalid local move: {}", e);
                continue; 
            }
            let serialized = msg.serialize();
            println!("Sending: {}", serialized);
            writer.write_all(serialized.as_bytes())?;
        } else {
            eprintln!("Invalid move string: {}", trimmed);
        }
    }
    Ok(())
}

fn handle_client(stream: TcpStream, board: Arc<Mutex<Board>>) -> io::Result<()> {
    let reader = stream.try_clone()?;
    let writer = stream;
    spawn_receiver(reader, Arc::clone(&board), "client");
    stdin_sender(writer, board)
}

pub fn start_client(addr: &str) -> io::Result<()> {
    let stream = TcpStream::connect(addr)?;
    println!("Connected to server {}", addr);
    let reader = stream.try_clone()?;
    let writer = stream;
    let board = Arc::new(Mutex::new(Board::start_pos()));
    spawn_receiver(reader, Arc::clone(&board), "server");
    stdin_sender(writer, board)
}
