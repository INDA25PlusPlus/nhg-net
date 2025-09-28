use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::thread;

// https://github.com/INDA25PlusPlus/chesstp-spec

fn handle_client(mut stream: TcpStream) -> std::io::Result<()> {
    let mut buf = [0u8; 128];
    stream.write_all(b"hey client!").unwrap(); 
    loop {
        let n = stream.read(&mut buf)?;
        if n == 0 {
            println!("Client disconnected.");
            break; 
        }
        println!("Received: {:?}", String::from_utf8_lossy(&buf[..n]));
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
