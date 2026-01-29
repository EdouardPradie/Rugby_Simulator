use crate::game::models::*;

#[derive(Clone)]
pub struct GameState {
    pub state: State,
    pub field: Field,
    pub time: u64,
    pub home_team: Team,
    pub away_team: Team,
    pub ball: Ball,
    pub ball_throw: BallThrow,
}
