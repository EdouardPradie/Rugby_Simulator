use std::io::{Read, Write};
use std::net::TcpStream;
use std::io::ErrorKind;
use std::sync::mpsc::Sender;

use crate::network::event::ClientEvent;

/// Handles communication with a single client.
pub fn handle_client(mut stream: TcpStream, tx: Sender<ClientEvent>) {
    let mut buffer = [0; 2048];
    let addr = stream.peer_addr().unwrap();

    loop {
        match stream.read(&mut buffer) {
            Ok(0) => {
                println!("Client {} disconnected.", addr);
                let _ = tx.send(ClientEvent::Disconnected(addr));
                break;
            }
            Ok(n) => {
                // Check if the received data is an "init" message
                if buffer.starts_with(b"init") {
                    println!("Initialization message received from {}", addr);

                    let tmp  = buffer.split(|&byte| byte == b'\n');
                    let mut field: String = String::new();
                    let mut home_players: Vec<String> = Vec::new();
                    let mut away_players: Vec<String> = Vec::new();
                    for (index, part) in tmp.enumerate() {
                        if index == 0 {
                            // Skip the first part which is "init"
                            continue;
                        } else if index == 1 {
                            // The second part is the field
                            field = String::from_utf8_lossy(part).to_string();
                        } else if index <= 24 {
                            // The next 23 parts are home players
                            home_players.push(std::str::from_utf8(part).expect("Invalid UTF-8 sequence").to_string());
                        } else {
                            // The remaining parts are away players
                            away_players.push(std::str::from_utf8(part).expect("Invalid UTF-8 sequence").to_string());
                        }
                    }

                    println!("Field: {}", field);
                    println!("Home Players: {}", home_players.len());
                    println!("Away Players: {}", away_players.len());

                    let _ = tx.send(ClientEvent::Initialized {
                        addr,
                        field,
                        home_players,
                        away_players,
                    });
                    continue; // don't respond
                }

                // Echo back the received data
                if let Err(e) = stream.write_all(&buffer[..n]) {
                    println!("Failed to send response: {}", e);
                    break;
                }
            }
            Err(ref e) if e.kind() == ErrorKind::WouldBlock => {
                // No data to read yet, it's fine, just wait a little
                std::thread::sleep(std::time::Duration::from_millis(10));
                continue;
            }
            Err(e) => {
                println!("Failed to read from stream: {}", e);
                break;
            }
        }
    }
}
