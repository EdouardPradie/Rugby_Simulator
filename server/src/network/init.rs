use std::io::{Read, Write};
use std::sync::mpsc::Sender;
use std::net::TcpStream;
use std::io::ErrorKind;

use crate::game::game_state::GameState;
use crate::network::event::ClientEvent;

/// Handles communication with a single client.
pub fn init_client(mut stream: TcpStream, buffer: &str, tx: Sender<ClientEvent>) -> String {
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

    let mut response = String::from("start\n");
    response.push_str(client.positions().as_str());
    return response;
}