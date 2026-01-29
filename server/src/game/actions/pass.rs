use crate::game::game_state::GameState;
use crate::game::constants::*;
use crate::game::models::*;

impl GameState {
    pub fn pass(&mut self, team: char, number: i32, direction: f32) -> bool {
        let direction_rad = direction.to_radians();
        let high_rad = (PASS_HEIGHT as f32).to_radians();
        let mut pass_invalid = false;
        let mut px = 0.0;
        let mut py = 0.0;

        let players = if team == 'H' {
            &mut self.home_team.players
        } else {
            &mut self.away_team.players
        };

        if let Some(p) = players.iter_mut().find(|p| p.number == number as usize) {
            if p.ball_pos {
                px = p.x;
                py = p.y;
                let invalid_direction = (((team == 'H' &&  self.field.home_direction_try == 'N') ||
                (team == 'A' &&  self.field.home_direction_try == 'S')) &&
                (direction > 270.0 || direction < 90.0)) ||
                (((team == 'H' &&  self.field.home_direction_try == 'S') ||
                (team == 'A' &&  self.field.home_direction_try == 'N')) &&
                direction > 90.0 && direction < 270.0);

                if invalid_direction {
                    pass_invalid = true;
                } else {
                    p.ball_pos = false;
                    self.ball.is_carried = false;

                    let speed = (PASS_DIST * GRAVITY).sqrt() / 1.12; // Empirical correction

                    self.ball_throw = BallThrow {
                        vx: speed * high_rad.cos() * direction_rad.cos(),
                        vy: speed * high_rad.cos() * direction_rad.sin(),
                        vz: speed * high_rad.sin(),
                        active: true,
                    };
                }
            }
        }

        if pass_invalid {
            print!("Invalid pass direction for player {} {}\n", team, number);
            self.setup_scrum(if team == 'H' {'A'} else {'H'} , px, py);
            return true;
        }
        false
    }
}