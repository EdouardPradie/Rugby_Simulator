use std::net::SocketAddr;

pub enum ClientEvent {
    Disconnected(SocketAddr),
    Initialized {
        addr: SocketAddr,
        field: String,
        home_players: Vec<String>,
        away_players: Vec<String>,
    },
}