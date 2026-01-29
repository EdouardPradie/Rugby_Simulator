use crate::game::game_state::GameState;

impl GameState {

    //SCRUM

    pub fn try_catch_ball_in_scrum(&mut self, team: char, number: i32) {

        let players = if team == 'H' {
            &mut self.home_team.players
        } else {
            &mut self.away_team.players
        };

        if let Some(p) = players.iter_mut().find(|p| p.number == number as usize) {
            let distance = ((p.x - self.ball.x).powi(2) + (p.y - self.ball.y).powi(2)).sqrt();

            if distance < 1.0 {
                p.ball_pos = true;
                self.ball.is_carried = true;
                self.ball_throw.active = false;

                match team {
                    'H' => {
                        print!("Home player {} picked up the ball in scrum\n", p.number);
                        self.ball.x = p.x + if self.field.home_direction_try == 'N' { 0.5 } else { -0.5 };
                        self.state.x = self.ball.x;
                    },
                    'A' => {
                        print!("Away player {} picked up the ball in scrum\n", p.number);
                        self.ball.x = p.x + if self.field.home_direction_try == 'N' { -0.5 } else { 0.5 };
                        self.state.x = self.ball.x;
                    },
                    _ => {}
                }
                self.state.name = "play".to_string();
                self.state.team = team;
                self.ball.y = p.y;
                self.state.y = self.ball.y;
                self.ball.z = 1.0;
            }
        }
    }

    //RUCK

    pub fn try_catch_ball_in_ruck(&mut self, team: char, number: i32) -> bool {
        let is_offside = self.check_offside_ruck();
        let players = if team == 'H' {
            &mut self.home_team.players
        } else {
            &mut self.away_team.players
        };

        if let Some(p) = players.iter_mut().find(|p| p.number == number as usize) {
            let distance_ball = ((p.x - self.ball.x).powi(2) + (p.y - self.ball.y).powi(2)).sqrt();
            let distance_ruck = ((p.x - self.state.x).powi(2) + (p.y - self.state.y).powi(2)).sqrt();

            if distance_ball < 1.0 && distance_ruck >= 1.0 {
                if is_offside {
                    print!("Offside penalty for {}\n", self.state.team);
                    return true;
                }
                p.ball_pos = true;
                self.ball.is_carried = true;
                self.ball_throw.active = false;

                match team {
                    'H' => {
                        print!("Home player {} picked up the ball in scrum\n", p.number);
                        self.ball.x = p.x + if self.field.home_direction_try == 'N' { 0.5 } else { -0.5 };
                        self.state.x = self.ball.x;
                    },
                    'A' => {
                        print!("Away player {} picked up the ball in scrum\n", p.number);
                        self.ball.x = p.x + if self.field.home_direction_try == 'N' { -0.5 } else { 0.5 };
                        self.state.x = self.ball.x;
                    },
                    _ => {}
                }
                self.state.name = "play".to_string();
                self.state.team = team;
                self.ball.y = p.y;
                self.state.y = self.ball.y;
                self.ball.z = 1.0;
            }
        }
        false
    }

    fn check_offside_ruck(&self) -> bool {
        let players = if self.state.team == 'H' {
            &self.away_team.players
        } else {
            &self.home_team.players
        };
        let diff = if (self.state.team == 'H' && self.field.home_direction_try == 'N') ||
                            (self.state.team == 'A' && self.field.home_direction_try == 'S') {
            1.0
        } else {
            -1.0
        };

        for player in players.iter() {
            let distance = ((player.x - self.state.x).powi(2) + (player.y - self.state.y).powi(2)).sqrt();
            if distance >= self.state.size &&
            ((diff == 1.0 && player.x < self.state.x + diff) ||
            (diff == -1.0 && player.x > self.state.x + diff)) {
                print!("Player {} is offside\n", player.number);
                return true;
            }
        }
        return false;
    }
}