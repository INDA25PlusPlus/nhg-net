use std::net::{Shutdown, TcpListener, TcpStream};
use std::io::{Read, Write, self};
use std::thread;
use std::time::Duration;

use crate::protocol;
use protocol::{MoveMsg};


// https://github.com/INDA25PlusPlus/chesstp-spec
// https://eleftheriabatsou.hashnode.dev/tutorial-chat-application-client-server-in-rust

fn handle_client(mut stream: TcpStream) -> std::io::Result<()> {
    let mut s = stream.try_clone()?; // need to clone for reading and writing
    thread::spawn(move || {
        let mut buf = [0u8; 128];
        while let Ok(n) = stream.read(&mut buf) {
            if n == 0 {
                println!("Client disconnected.");
                break;
            }
            let raw = String::from_utf8_lossy(&buf[..n]);
            if let Some(msg) = MoveMsg::deserialize(&raw) {
                println!("Received move: {}, state: {}, fen: {}", msg.move_str, msg.game_state, msg.fen);
            } else {
                println!("Received invalid message: {}", raw);
            }
        }
    });

    let stdin = io::stdin();
    let mut line = String::new();
    loop {
        line.clear();
        if stdin.read_line(&mut line).is_err() {
            break;
        }
        let trimmed = line.trim();
        if line.trim().is_empty() {
            continue;
        }
        let msg = MoveMsg {
            move_str: trimmed.to_string(),
            game_state: "0-0".to_string(),
            fen: "8/8/8/8/8/8/8/8".to_string(),
        };
        let serialized = msg.serialize();
        println!("Sending: {}", serialized);
        s.write_all(serialized.as_bytes())?;
    }
    Ok(())
}

pub fn start_server(addr: &str) -> std::io::Result<()> {
    let listener = TcpListener::bind(addr)?;
    println!("Server listening on {}", addr);
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| {
                    handle_client(stream).unwrap();
                });
            }
            Err(e) => eprintln!("Connection failed: {}", e),
        }
    }
    Ok(())
}

pub fn start_client(addr: &str) -> std::io::Result<()>{
    let mut stream = TcpStream::connect(addr)?;
    let mut s = stream.try_clone()?;

    thread::spawn(move || {
        let mut buf = [0u8; 128];
        while let Ok(n) = stream.read(&mut buf) {
            if n == 0 {
                println!("Client disconnected.");
                break;
            }
            let raw = String::from_utf8_lossy(&buf[..n]);
            if let Some(msg) = MoveMsg::deserialize(&raw) {
                println!("Received move: {}, state: {}, fen: {}", msg.move_str, msg.game_state, msg.fen);
            } else {
                println!("Received invalid message: {}", raw);
            }
        }
    });

    let stdin = io::stdin();
    let mut line = String::new();
    loop {
        line.clear();
        if stdin.read_line(&mut line).is_err() {
            break;
        }
        let trimmed = line.trim();
        if line.trim().is_empty() {
            continue;
        }
        let msg = MoveMsg {
            move_str: trimmed.to_string(),
            game_state: "0-0".to_string(),
            fen: "8/8/8/8/8/8/8/8".to_string(),
        };
        let serialized = msg.serialize();
        println!("Sending: {}", serialized);
        s.write_all(serialized.as_bytes())?;
    }
    Ok(())
}