use std::io::{Read, Write};
use std::sync::mpsc::Sender;
use std::net::TcpStream;
use std::io::ErrorKind;

use crate::game::game_state::GameState;
use crate::network::event::ClientEvent;

/// Handles communication with a single client.
pub fn handle_client(mut stream: TcpStream, display_enable: bool, tx: Sender<ClientEvent>) {
    let mut buffer = [0; 2048];
    let mut state= 0;
    let addr = stream.peer_addr().unwrap();
    // Initialize the client environment
    let mut client = GameState::new();

    loop {
        match stream.read(&mut buffer) {
            Ok(0) => {
                println!("Client {} disconnected.", addr);
                if display_enable {
                    let _ = tx.send(ClientEvent::Disconnected(addr));
                }
                break;
            }
            Ok(n) => {
                // Check if the received data is an "init" message
                if buffer.starts_with(b"init") && state == 0 {
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

                    client.initialize(field.clone(), home_players, away_players);
                    if display_enable {
                        let _ = tx.send(ClientEvent::Initialized {
                            addr,
                            field,
                            drawable: client.get_drawable(),
                        });
                    }

                    state = 1; // Change state to indicate initialization is done
                    let mut response = String::from("start\n");
                    response.push_str(client.positions().as_str());
                    // if let Err(e) = stream.write_all(response.as_bytes()) {
                    //     println!("Failed to send player positions: {}", e);
                    //     break;
                    // }
                    continue;
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
