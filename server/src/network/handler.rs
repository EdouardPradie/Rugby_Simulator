use std::io::{Read, Write};
use std::net::TcpStream;
use std::net::SocketAddr;
use std::io::ErrorKind;
use std::sync::mpsc::Sender;

/// Handles communication with a single client.
pub fn handle_client(mut stream: TcpStream, tx: Sender<SocketAddr>) {
    let mut buffer = [0; 512];
    let addr = stream.peer_addr().unwrap();

    loop {
        match stream.read(&mut buffer) {
            Ok(0) => {
                println!("Client {} disconnected.", addr);
                let _ = tx.send(addr);
                break;
            }
            Ok(n) => {
                println!("Received: {}", String::from_utf8_lossy(&buffer[..n]));
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
