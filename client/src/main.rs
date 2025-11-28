use std::env;
use dotenv::dotenv;
use std::io::{Write, Read};
use std::thread;
use std::time::Duration;
use std::net::TcpStream;

mod init;
use init::init_game::initialize_game;

mod ai;
use ai::take_decision::take_decision;

fn main() {
    // Load environment variables from .env file
    dotenv().ok();

    // Read IP and PORT from environment
    let ip = env::var("IP").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env::var("PORT").unwrap_or_else(|_| "7878".to_string());
    let address = format!("{}:{}", ip, port);

    // Try to connect to the server
    match TcpStream::connect(&address) {
        Ok(mut stream) => {
            println!("Successfully connected to server at {}", address);

            let input = initialize_game();

            // Send init       message to server
            if let Err(e) = stream.write_all(input.as_bytes()) {
                println!("Failed to send message: {}", e);
                return;
            }

            stream.set_nonblocking(true).expect("Failed to set non-blocking");

            let mut buffer = [0; 2048];
            // Read response from server
            loop {
                match stream.read(&mut buffer) {
                    Ok(0) => {
                        // Serveur a fermé la connexion
                        println!("Server disconnected.");
                        break;
                    }
                    Ok(n) => {
                        let msg = String::from_utf8_lossy(&buffer[..n]);

                        let decision = take_decision(msg.as_ref());
                        if let Err(e) = stream.write_all(decision.as_bytes()) {
                            println!("Failed to send decision: {}", e);
                            break;
                        }
                    }
                    Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                        // Rien à lire pour le moment
                        thread::sleep(Duration::from_millis(100));
                        continue;
                    }
                    Err(e) => {
                        println!("Error while reading: {}", e);
                        break;
                    }
                }
            }
        }
        Err(e) => {
            println!("Failed to connect to server: {}", e);
        }
    }
}
