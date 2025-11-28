use std::env;
use dotenv::dotenv;
use std::collections::HashMap;
use std::thread;
use std::net::{SocketAddr, TcpListener};
use std::sync::mpsc::{channel, Sender, Receiver};
use std::time::Duration;

mod gui;
use gui::display::Display;
use gui::drawable::Drawable;

mod game;

mod network;
use network::handler::handle_client;
use network::event::ClientEvent;

fn main() {
    // Load environment variables from .env file
    dotenv().ok();

    // Read IP and PORT from environment
    let ip = env::var("IP").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env::var("PORT").unwrap_or_else(|_| "7878".to_string());
    let address = format!("{}:{}", ip, port);

    // Communication with the handler
    let (tx, rx): (Sender<ClientEvent>, Receiver<ClientEvent>) = channel();

    // Initialize display availability
    let display_enable: bool = env::var("DISPLAY").unwrap_or("false".to_string()).to_lowercase() == "true";
    let run_time: bool = env::var("RUN_TIME").unwrap_or("false".to_string()).to_lowercase() == "true";
    let pixel_per_cell: usize = env::var("PIXEL_SIZE").unwrap_or("0".to_string()).parse().unwrap();
    let mut displays: HashMap<SocketAddr, Display> = HashMap::new();
    let mut drawables: HashMap<SocketAddr, Drawable> = HashMap::new();

    // Start TCP listener
    let listener = TcpListener::bind(&address).expect("Failed to bind server address");
    listener.set_nonblocking(true).expect("Cannot set non-blocking");

    if display_enable {
        println!("\nGraphics mode enabled.");
    } else {
        println!("\nGraphics mode disabled.");
    }

    println!("Server listening on {}\n", address);

    // Accept incoming connections
    loop {
        match listener.accept() {
            Ok((stream, _addr)) => {
                println!("New client connected {}.", stream.peer_addr().unwrap());
                // Initialize a new client state
                let client_id = stream.peer_addr().unwrap();

                if display_enable {
                    let field_width: usize = env::var("FIELD_MAX_WIDTH").unwrap_or("100".to_string()).parse().unwrap();
                    let field_height: usize = env::var("FIELD_MAX_HEIGHT").unwrap_or("70".to_string()).parse().unwrap();
                    let try_size: usize = env::var("TRY_MIN_SIZE").unwrap_or("10".to_string()).parse().unwrap();
                    let display = Display::new(field_width * pixel_per_cell, field_height * pixel_per_cell, try_size * pixel_per_cell);

                    displays.insert(client_id, display);
                }
                let tx_clone = tx.clone();
                let display_enable_clone = display_enable.clone();
                let run_time_clone = run_time.clone();
                thread::spawn(move || {
                    handle_client(stream, display_enable_clone, run_time_clone, tx_clone);
                });
            }
            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                // No new connection, it's fine, just continue
            }
            Err(e) => {
                println!("Connection failed: {}", e);
            }
        }
        // Catch clients event (from handle_client via tx)
        while let Ok(event) = rx.try_recv() {
            match event {
                ClientEvent::Disconnected(addr) => {
                    if let Some(mut display) = displays.remove(&addr) {
                        display.close();
                        println!("Cleaned up {} window.", addr);
                    }
                }
                ClientEvent::Initialized { addr, field, drawable} => {
                    if display_enable {
                        if let Some(display) = displays.get_mut(&addr) {
                            display.initialize(field, pixel_per_cell);
                            drawables.insert(addr, drawable);
                        }
                    }
                }
                ClientEvent::DisplayUpdate { addr, drawable } => {
                    if display_enable {
                        drawables.insert(addr, drawable);
                    }
                }
            }
        }
        // Game logic
        for (client_id, drawable) in &mut drawables {
            // state.test();
            if display_enable && displays.iter().any(|(id, _)| *id == *client_id) {
                if let Some(display) = displays.iter_mut().find(|(id, _)| **id == *client_id) {
                    if display.1.is_open() {
                        display.1.render(drawable, pixel_per_cell);
                        thread::sleep(Duration::from_millis(1000 / 30)); // 30 FPS
                    }
                }
            }
        }
    }
}
