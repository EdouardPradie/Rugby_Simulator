use crate::game::models::*;

#[derive(Clone)]
pub struct Team {
    pub players: Vec<Player>,
    pub bench: Vec<Player>,
    pub score: u32,
    pub try_scored: u32,
    pub transformation: u32,
    pub penalty: u32,
}