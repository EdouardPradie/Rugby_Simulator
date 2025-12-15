use std::io::{Read, Write};
use std::sync::mpsc::Sender;
use std::net::TcpStream;
use std::io::ErrorKind;

use crate::game::game_state::GameState;
use crate::network::event::ClientEvent;

/// Handles communication with a single client.
pub fn handle_client(mut stream: TcpStream, display_enable: bool, run_time: bool, tx: Sender<ClientEvent>) {
    let mut buffer = [0; 2500];
    let mut status= 0;
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

                // INIT
                if buffer.starts_with(b"init") && status == 0 {
                    println!("Initialization message received from {}", addr);

                    let tmp  = buffer.split(|&byte| byte == b'\n');
                    let mut field: String = String::new();
                    let mut state: String = String::new();
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
                        } else if index > 24 && index <= 47 {
                            // The remaining parts are away players
                            away_players.push(std::str::from_utf8(part).expect("Invalid UTF-8 sequence").to_string());
                        } else if index == 48 {
                            // The remaining parts are away players
                            let cleaned = part.split(|&b| b == 0).next().unwrap();
                            state = String::from_utf8_lossy(cleaned).to_string();
                        } else {
                            print!("Extra data received during initialization from {}: {:?}\n", addr, std::str::from_utf8(part).expect("Invalid UTF-8 sequence").to_string());
                        }
                    }

                    client.initialize(field.clone(), home_players, away_players, state);
                    if display_enable {
                        std::thread::sleep(std::time::Duration::from_millis(240));
                        let _ = tx.send(ClientEvent::Initialized {
                            addr,
                            field,
                            drawable: client.get_drawable(),
                        });
                    }

                    status = 1; // Change state to indicate initialization is done
                    let mut response = format!("{}\n", client.state.name);
                    response.push_str(&format!("time:{}\n", &client.time.to_string()));
                    response.push_str(client.positions().as_str());

                    if let Err(e) = stream.write_all(response.as_bytes()) {
                        println!("Failed to send player positions: {}", e);
                        break;
                    }
                    buffer.fill(0);
                    continue;
                }

                // PLAY

                if buffer.starts_with(b"play") && status == 1 {
                    let input: String = String::from_utf8_lossy(&buffer[..n]).to_string();
                    client.play(input);
                    let mut response = format!("{}\n", client.state.name);
                    response.push_str(&format!("time:{}\n", &client.time.to_string()));
                    response.push_str(client.positions().as_str());

                    if display_enable {
                        if run_time {
                            std::thread::sleep(std::time::Duration::from_millis(200));
                        }
                        let _ = tx.send(ClientEvent::DisplayUpdate {
                            addr,
                            drawable: client.get_drawable(),
                        });
                    }

                    if let Err(e) = stream.write_all(response.as_bytes()) {
                        println!("Failed to send update response: {}", e);
                        break;
                    }
                    buffer.fill(0);
                    continue;
                }

                // SCRUM

                if buffer.starts_with(b"scrum") && status == 1 {
                    let input: String = String::from_utf8_lossy(&buffer[..n]).to_string();
                    client.scrum(input);
                    let mut response = format!("{}\n", client.state.name);
                    response.push_str(&format!("time:{}\n", &client.time.to_string()));
                    response.push_str(client.positions().as_str());

                    if display_enable {
                        if run_time {
                            std::thread::sleep(std::time::Duration::from_millis(200));
                        }
                        let _ = tx.send(ClientEvent::DisplayUpdate {
                            addr,
                            drawable: client.get_drawable(),
                        });
                    }

                    if let Err(e) = stream.write_all(response.as_bytes()) {
                        println!("Failed to send update response: {}", e);
                        break;
                    }
                    buffer.fill(0);
                    continue;
                }

                // RUCK

                if buffer.starts_with(b"ruck") && status == 1 {
                    let input: String = String::from_utf8_lossy(&buffer[..n]).to_string();
                    client.ruck(input);
                    let mut response = format!("{}\n", client.state.name);
                    response.push_str(&format!("time:{}\n", &client.time.to_string()));
                    response.push_str(client.positions().as_str());

                    if display_enable {
                        if run_time {
                            std::thread::sleep(std::time::Duration::from_millis(200));
                        }
                        let _ = tx.send(ClientEvent::DisplayUpdate {
                            addr,
                            drawable: client.get_drawable(),
                        });
                    }

                    if let Err(e) = stream.write_all(response.as_bytes()) {
                        println!("Failed to send update response: {}", e);
                        break;
                    }
                    buffer.fill(0);
                    continue;
                }

                // Handle unrecognized input
                println!("Unrecognized input from {}: {:?}", addr, String::from_utf8_lossy(&buffer[..n]).to_string());
                buffer.fill(0); // Clear the buffer to avoid processing leftover data

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
