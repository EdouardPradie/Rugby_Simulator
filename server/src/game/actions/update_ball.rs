use crate::game::game_state::GameState;
use crate::game::constants::*;

impl GameState {

    // PLAY
    pub fn update_ball_velocity(&mut self) {
        if self.ball_throw.active {
            self.ball.x += self.ball_throw.vx * DT;
            self.ball.y += self.ball_throw.vy * DT;
            self.ball.z += self.ball_throw.vz * DT;
            self.ball_throw.vz -= GRAVITY * DT; // gravity effect

            self.state.x = self.ball.x;
            self.state.y = self.ball.y;

            if self.ball.z <= 0.0 {
                self.ball.z = 0.0;
                self.ball_throw.active = false;
            }
        }
    }

    pub fn update_ball_carrie(&mut self) {
        if !self.ball.is_carried && self.ball.z <= 3.5 {
            for (team, player) in
            self.home_players.iter_mut().map(|p| ('H', p))
            .chain(self.away_players.iter_mut().map(|p| ('A', p))) {
                let distance = ((player.x - self.ball.x).powi(2) + (player.y - self.ball.y).powi(2)).sqrt();
                if distance < 1.0 && self.ball.z <= player.size + 50.0 { // 50 cm player arm
                    let is_successful = rand::random::<f32>() * 100.0 > (self.field.weather / 2) as f32;
                    if !is_successful {
                        print!("Player {} failed to pick up the ball due to weather\n", player.number);
                        continue;
                    }
                    player.ball_pos = true;
                    self.ball.is_carried = true;
                    self.ball_throw.active = false;

                    match team {
                        'H' => {
                            print!("Home player {} picked up the ball\n", player.number);
                            self.ball.x = player.x + if self.field.home_direction_try == 'N' { 0.5 } else { -0.5 };
                            self.state.x = self.ball.x;
                        },
                        'A' => {
                            print!("Away player {} picked up the ball\n", player.number);
                            self.ball.x = player.x + if self.field.home_direction_try == 'N' { -0.5 } else { 0.5 };
                            self.state.x = self.ball.x;
                        },
                        _ => {}
                    }
                    self.state.team = team;
                    self.ball.y = player.y;
                    self.state.y = self.ball.y;
                    self.ball.z = 1.0;
                    break;
                }
            }
        }
    }

    // SCRUM
    pub fn update_ball_position_scrum(&mut self, scrum_h_pound: f32, scrum_a_pound: f32) {
        let scrum_in = if self.state.team == 'H' { scrum_h_pound } else { scrum_a_pound };
        let scrum_front = if self.state.team == 'H' { scrum_a_pound } else { scrum_h_pound };
        let mut direction = if self.state.team == 'H' {
            if self.field.home_direction_try == 'N' { -0.5 } else { 0.5 }
        } else {
            if self.field.home_direction_try == 'N' { 0.5 } else { -0.5 }
        };
        if scrum_front > scrum_in {
            let diff_ratio = (scrum_front - scrum_in) / (scrum_front + scrum_in) * 20.0;
            let reverse_probability = diff_ratio.clamp(0.0, 1.0);
            if rand::random::<f32>() < reverse_probability {
                direction = -direction;
                print!("Scrum contest\n");
            }
        }
        self.ball.x += direction;
    }
}