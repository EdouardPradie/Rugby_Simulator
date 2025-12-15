use crate::game::models::*;

#[derive(Clone)]
pub struct GameState {
    pub state: State,
    pub field: Field,
    pub time: u64,
    pub home_players: Vec<Player>,
    pub home_bench: Vec<Player>,
    pub away_players: Vec<Player>,
    pub away_bench: Vec<Player>,
    pub ball: Ball,
    pub ball_throw: BallThrow,
}
