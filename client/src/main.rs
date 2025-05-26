use std::env;
use std::net::TcpStream;
use std::io::{self, Write, Read};
use dotenv::dotenv;

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

            let mut input = String::new();
            println!("Enter a message to send:");
            io::stdin().read_line(&mut input).unwrap();

            // Send message to server
            if let Err(e) = stream.write_all(input.as_bytes()) {
                println!("Failed to send message: {}", e);
                return;
            }

            let mut buffer = [0; 512];
            // Read response from server
            match stream.read(&mut buffer) {
                Ok(bytes_read) => {
                    println!("Server response: {}", String::from_utf8_lossy(&buffer[..bytes_read]));
                }
                Err(e) => {
                    println!("Failed to read from server: {}", e);
                }
            }
        }
        Err(e) => {
            println!("Failed to connect to server: {}", e);
        }
    }
}
