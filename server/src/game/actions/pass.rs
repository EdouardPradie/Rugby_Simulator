use crate::game::game_state::GameState;
use crate::game::constants::*;
use crate::game::models::*;

impl GameState {
    pub fn pass(&mut self, team: char, number: i32, direction: f32) {
        let direction_rad = direction.to_radians();
        let high_rad = (PASS_HEIGHT as f32).to_radians();

        let players = if team == 'H' {
            &mut self.home_players
        } else {
            &mut self.away_players
        };

        if let Some(p) = players.iter_mut().find(|p| p.number == number as usize) {
            if p.ball_pos {
                p.ball_pos = false;
                self.ball.is_carried = false;

                let speed = (PASS_DIST * GRAVITY).sqrt() / 1.12; // Empirical correction

                let vx = speed * high_rad.cos() * direction_rad.cos();
                let vy = speed * high_rad.cos() * direction_rad.sin();
                let vz = speed * high_rad.sin();

                self.ball_throw = BallThrow {
                    vx,
                    vy,
                    vz,
                    active: true,
                };
            }
        }
    }
}