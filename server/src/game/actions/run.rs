use crate::game::game_state::GameState;
use crate::game::constants::*;

impl GameState {
    pub fn run(&mut self, team: char, number: i32, direction: f32, is_running: bool) {
        let players = if team == 'H' {
            &mut self.home_team.players
        } else {
            &mut self.away_team.players
        };

        if let Some(p) = players.iter_mut().find(|p| p.number == number as usize) {
            let speed = if is_running { p.speed } else { WALK_SPEED };
            let rad = direction.to_radians();

            let dx = rad.cos() * speed * RUNNING_SPEED_FACTOR;
            let dy = rad.sin() * speed * RUNNING_SPEED_FACTOR;

            p.x += dx;
            p.y += dy;

            if p.ball_pos {
                self.ball.x += dx;
                self.ball.y += dy;
            }
        }
    }

    pub fn run_ruck(&mut self, team: char, number: i32, direction: f32, is_running: bool) {
        let players = if team == 'H' {
            &mut self.home_team.players
        } else {
            &mut self.away_team.players
        };

        if let Some(p) = players.iter_mut().find(|p| p.number == number as usize) {
            if p.is_tackle {
                print!("Player {} {} is tackled and cannot move in ruck\n", team, number);
                return;
            }
            let speed = if is_running { p.speed } else { WALK_SPEED };
            let rad = direction.to_radians();
            p.x += rad.cos() * (speed * RUNNING_SPEED_FACTOR);
            p.y += rad.sin() * (speed * RUNNING_SPEED_FACTOR);
            if p.ball_pos {
                self.ball.x += rad.cos() * (speed * RUNNING_SPEED_FACTOR);
                self.ball.y += rad.sin() * (speed * RUNNING_SPEED_FACTOR);
            }
            let distance = ((p.x - self.state.x).powi(2) + (p.y - self.state.y).powi(2)).sqrt();
            if distance < self.state.size {
                if team == 'H' {
                    if (self.field.home_direction_try == 'S' && direction >= 135.0 && direction <= 225.0) ||
                    (self.field.home_direction_try == 'N' && (direction >= 315.0 || direction <= 45.0)) {
                        print!("Home player {} go in ruck\n", number);
                    } else {
                        print!("Penalty for {}\n", "A");
                    }
                }
                if team == 'A' {
                    if (self.field.home_direction_try == 'N' && direction >= 135.0 && direction <= 225.0) ||
                    (self.field.home_direction_try == 'S' && (direction >= 315.0 || direction <= 45.0)) {
                        print!("Away player {} go in ruck\n", number);
                    } else {
                        print!("Penalty for {}\n", "H");
                    }
                }
            }
        }
    }
}