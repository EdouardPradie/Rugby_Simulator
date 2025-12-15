use crate::game::game_state::GameState;
use crate::game::constants::*;
use crate::game::models::*;

impl GameState {
    pub fn kick(&mut self, team: char, number: i32, direction: f32, high: f32) {
        let tmp_high = high.clamp(0.0, 90.0);
        let mut dir = direction.to_radians();
        let mut elev = tmp_high.to_radians();

        let players = if team == 'H' {
            &mut self.home_players
        } else {
            &mut self.away_players
        };

        if let Some(p) = players.iter_mut().find(|p| p.number == number as usize) {
            if p.ball_pos {
                p.ball_pos = false;
                self.ball.is_carried = false;
                self.ball.z = 0.0;

                let mut speed = (p.foot * GRAVITY).sqrt() / 1.12; // Empirical correction
                let is_successful = rand::random::<f32>() * 100.0 < p.p_foot;

                if !is_successful {
                    print!("Failed kick\n");
                    let horizontal_dev_deg = (rand::random::<f32>() * 20.0) - 10.0; // -10° to +10°
                    dir += horizontal_dev_deg.to_radians();

                    let vertical_dev_deg = (rand::random::<f32>() * 10.0) - 5.0; // -5° to +5°
                    elev += vertical_dev_deg.to_radians();

                    let loss = 0.80 + rand::random::<f32>() * 0.2; // between 80% and 100%
                    speed *= loss;
                } else {
                    print!("Success kick\n");
                }

                let vx = speed * elev.cos() * dir.cos();
                let vy = speed * elev.cos() * dir.sin();
                let vz = speed * elev.sin();

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