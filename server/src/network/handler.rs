use std::io::{Read, Write};
use std::sync::mpsc::Sender;
use std::net::{SocketAddr, TcpStream};
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
                    let mut response = format!("{} {}\n", client.state.name, client.state.team);
                    response.push_str(&format!("time:{}\n", &client.time.to_string()));
                    response.push_str(client.positions().as_str());

                    if let Err(e) = stream.write_all(response.as_bytes()) {
                        println!("Failed to send player positions: {}", e);
                        break;
                    }
                    buffer.fill(0);
                    continue;
                }

                // ACTION HANDLING

                let handlers: &[(&[u8], fn(&mut GameState, String))] = &[
                    (b"play", GameState::play),
                    (b"free-kick", GameState::play),
                    (b"penalty-kick", GameState::play),
                    (b"scrum", GameState::scrum),
                    (b"ruck", GameState::ruck),
                    (b"set-penalty", GameState::penalty),
                ];

                if status == 1 {
                    for (cmd, handler) in handlers {
                        if buffer.starts_with(cmd) {
                            if handle_action(
                                &buffer, n, addr, &mut client, &mut stream, &tx,
                                display_enable, run_time,
                                |c, i| handler(c, i),
                            ) {
                                break;
                            }
                            buffer.fill(0);
                            continue;
                        }
                    }
                }

                // Handle unrecognized input
                if buffer[..n].iter().any(|&byte| byte != 0) {
                    println!("Unrecognized input from {}: {}", addr, String::from_utf8_lossy(&buffer[..n]).to_string());
                    buffer.fill(0);
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

fn handle_action<F>(
    buffer: &[u8],
    n: usize,
    addr: SocketAddr,
    client: &mut GameState,
    stream: &mut TcpStream,
    tx: &Sender<ClientEvent>,
    display_enable: bool,
    run_time: bool,
    action: F,
) -> bool
where
    F: FnOnce(&mut GameState, String),
{
    let input = String::from_utf8_lossy(&buffer[..n]).to_string();

    action(client, input);

    let mut response = format!("{} {}\n", client.state.name, client.state.team);
    response.push_str(&format!("time:{}\n", client.time));
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

    stream.write_all(response.as_bytes()).is_err()
}
