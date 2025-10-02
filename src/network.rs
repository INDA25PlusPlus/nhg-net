use std::io::{Read, Write};
use std::net::{TcpStream, TcpListener};
use std::sync::mpsc::{Sender, Receiver};
use std::thread;
use crate::protocol::ProtocolMsg;

pub fn start_client_with_channel(
    addr: &str, 
    rx: Receiver<ProtocolMsg>, 
    tx_to_gui: Sender<ProtocolMsg>,
) -> std::io::Result<()> {
    let mut stream = TcpStream::connect(addr)?;
    let mut reader = stream.try_clone()?;

    println!("Connected to server at {}", addr);
    
    let tx_clone = tx_to_gui.clone();
    thread::spawn(move || {
        let mut buf = [0u8; 128];
        while let Ok(n) = reader.read(&mut buf) {
            if n == 0 { break; }
            let raw = String::from_utf8_lossy(&buf[..n]);
            println!("Raw msg received: {}", raw);

            if let Some(msg) = ProtocolMsg::deserialize(&raw) {
                tx_clone.send(msg).unwrap();
            }
        }

    });

    // Main send loop: receives moves from GUI and sends over TCP
    while let Ok(msg) = rx.recv() {
                let serialized = msg.serialize();
        stream.write_all(serialized.as_bytes())?;
    }
    Ok(())
}

pub fn start_server_with_channel(
    addr: &str,
    rx: Receiver<ProtocolMsg>, 
    tx_to_gui: Sender<ProtocolMsg>,
) -> std::io::Result<()> {
    let listener = TcpListener::bind(addr)?;
    let (mut stream, _) = listener.accept()?;
    let mut reader = stream.try_clone()?;

    println!("Client connected to server at {}", addr);

    let tx_clone = tx_to_gui.clone();
    thread::spawn(move || {
        let mut buf = [0u8; 128];
        while let Ok(n) = reader.read(&mut buf) {
            if n == 0 { break; }
            let raw = String::from_utf8_lossy(&buf[..n]);
            println!("Raw msg received: {}", raw);

            if let Some(msg) = ProtocolMsg::deserialize(&raw) {
                tx_clone.send(msg).unwrap(); 
            }
        }
    });
    while let Ok(msg) = rx.recv() {
                let serialized = msg.serialize();
        stream.write_all(serialized.as_bytes())?;
    }
    Ok(())
}
