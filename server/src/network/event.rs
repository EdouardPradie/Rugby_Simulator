use std::net::SocketAddr;
use crate::gui::drawable::Drawable;

pub enum ClientEvent {
    Disconnected(SocketAddr),
    Initialized {
        addr: SocketAddr,
        field: String,
        drawable: Drawable,
    },
    DisplayUpdate {
        addr: SocketAddr,
        drawable: Drawable,
    },
}