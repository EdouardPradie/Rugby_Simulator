use crate::game::game_state::GameState;
use crate::game::models::*;

impl GameState {

    pub fn ruck(&mut self, input: String) {
        self.time += 25;

        let actions: Vec<&str> = input.split('\n').collect();
        for action in actions {
            if let Some((player, action)) = action.split_once(':') {
                let tmp = action.chars().nth(0).unwrap_or('\0');
                match tmp {
                    'R' => {
                        let team = player.chars().next().unwrap();
                        let number = player[1..].parse().unwrap_or(0);
                        let direction = action[1..].parse().unwrap_or(0.0);
                        // println!("Player {} {} runs {}", team, number, direction);
                        self.run_ruck(team, number, direction, true);
                    },
                    'W' => {
                        let team = player.chars().next().unwrap();
                        let number = player[1..].parse().unwrap_or(0);
                        let direction = action[1..].parse().unwrap_or(0.0);
                        // println!("Player {} {} walk {}", team, number, direction);
                        self.run_ruck(team, number, direction, false);
                    },
                    'T' => {
                        let team = player.chars().next().unwrap();
                        let number = player[1..].parse().unwrap_or(0);
                        self.try_catch_ball_in_ruck(team, number);
                    },
                    'S' => continue,
                    _ => {
                        print!("Unknown action in ruck: {}\n", action);
                    },
                }
            }
        }

        self.check_tackler();
        self.check_scrap();
    }

    fn check_tackler(&mut self) {
        for (team, player) in
        self.home_players.iter_mut().map(|p| ('H', p))
        .chain(self.away_players.iter_mut().map(|p| ('A', p))) {
            if player.is_tackler {
                let distance = ((player.x - self.state.x).powi(2) + (player.y - self.state.y).powi(2)).sqrt();
                if distance < 1.0 {
                    print!("Tackler penalty: ball for {}\n", if team == 'H' { 'A' } else { 'H' });
                } else {
                    player.is_tackler = false;
                }
            }
        }
    }

    fn check_scrap(&mut self) {
        let mut scrap_h_pound = 0.0;
        let mut scrap_a_pound = 0.0;
        let mut contest_player: Option<&mut Player> = None;
        let mut contest_dist: f32 = f32::MAX;

        for (team, player) in
        self.home_players.iter_mut().map(|p| ('H', p))
        .chain(self.away_players.iter_mut().map(|p| ('A', p))) {
            let distance = ((player.x - self.state.x).powi(2) + (player.y - self.state.y).powi(2)).sqrt();
            if distance < self.state.size && !player.is_tackle {
                if team == 'H' {
                    scrap_h_pound += player.pound;
                    if self.state.team == 'A' && distance < contest_dist {
                        contest_dist = distance;
                        contest_player = Some(player);
                    }
                } else {
                    scrap_a_pound += player.pound;
                    if self.state.team == 'H' && distance < contest_dist {
                        contest_dist = distance;
                        contest_player = Some(player);
                    }
                }
            }
        }

        let scrap_in = if self.state.team == 'H' { scrap_h_pound } else { scrap_a_pound };
        let scrap_front = if self.state.team == 'H' { scrap_a_pound } else { scrap_h_pound };

        if scrap_front > scrap_in {
            if let Some(player) = contest_player {
                //update ball
                player.ball_pos = true;
                self.ball.is_carried = true;
                self.ball.z = 1.0;
                if self.state.team == 'H' {
                    print!("Away player {} picked up the ball from ruck\n", player.number);
                    self.ball.x = player.x + if self.field.home_direction_try == 'N' { -0.5 } else { 0.5 };
                } else {
                    print!("Home player {} picked up the ball from ruck\n", player.number);
                    self.ball.x = player.x + if self.field.home_direction_try == 'N' { 0.5 } else { -0.5 };
                }
                self.ball.y = player.y;

                //update state
                self.state.name = "play".to_string();
                self.state.team = if self.state.team == 'H' { 'A' } else { 'H' };
                self.state.x = self.ball.x;
                self.state.y = self.ball.y;
            }
        }
    }
}