use crate::game::game_state::GameState;
use crate::game::constants::*;
use crate::game::models::*;

impl GameState {

    //SCRUM

    pub fn check_ball_out_of_scrum(&mut self) {
        let distance = ((self.ball.x - self.state.x).powi(2) + (self.ball.y - self.state.y).powi(2)).sqrt();
        if !self.ball.is_carried && distance >= SCRUM_SIZE {
            self.state.name = "play".to_string();
            self.state.x = self.ball.x;
            self.state.y = self.ball.y;
            print!("{}|{:.2}|", self.addr, (self.time as f32)/100.0);
            print!("Ball out of scrum, resuming play\n");
        }
    }

    //RUCK
    pub fn check_tackler(&mut self) -> bool {
        for (team, player) in
        self.home_team.players.iter_mut().map(|p| ('H', p))
        .chain(self.away_team.players.iter_mut().map(|p| ('A', p))) {
            if player.is_tackler {
                let distance = ((player.x - self.state.x).powi(2) + (player.y - self.state.y).powi(2)).sqrt();
                if distance < 1.0 {
                    let team_fouled = if team == 'H' { 'A' } else { 'H' };
                    print!("{}|{:.2}|", self.addr, (self.time as f32)/100.0);
                    print!("Tackler penalty: ball for {}\n", team_fouled);
                    return true;
                } else {
                    player.is_tackler = false;
                }
            }
        }
        false
    }

    pub fn check_scrap(&mut self) {
        let mut scrap_h_pound = 0.0;
        let mut scrap_a_pound = 0.0;
        let mut contest_player: Option<&mut Player> = None;
        let mut contest_dist: f32 = f32::MAX;

        for (team, player) in
        self.home_team.players.iter_mut().map(|p| ('H', p))
        .chain(self.away_team.players.iter_mut().map(|p| ('A', p))) {
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
                    print!("{}|{:.2}|", self.addr, (self.time as f32)/100.0);
                    print!("Away player {} picked up the ball from ruck\n", player.number);
                    self.ball.x = player.x + if self.field.home_direction_try == 'N' { -0.5 } else { 0.5 };
                } else {
                    print!("{}|{:.2}|", self.addr, (self.time as f32)/100.0);
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

    pub fn check_ball_position(&mut self) {
        // Check if the ball is out of bounds on the try zone
        // Check if the ball is out of bounds on the left or right side

        // Check if the ball is carried by a player
        if self.ball.is_carried || !self.ball_throw.active {
            return;
        }

        // Check if the ball pass a penalty
        let goal_post = if self.field.home_direction_try == 'N' {
            if self.state.team == 'H' {'N'} else {'S'}
        } else {
            if self.state.team == 'H' {'S'} else {'N'}
        };

        let goal_post_half_size = 2.8;
        let goal_post_x = if goal_post == 'N' { self.field.width as f32 + self.field.try_size as f32 } else { self.field.try_size as f32 };
        let goal_post_y = self.field.height as f32 / 2.0;

        if  (goal_post == 'N' &&
        self.ball.x > goal_post_x &&
        self.ball_throw.prev_x < goal_post_x) ||
        (goal_post == 'S'  &&
        self.ball.x < goal_post_x &&
        self.ball_throw.prev_x > goal_post_x) {
            let dix = self.ball.x - self.ball_throw.prev_x;
            let diy = self.ball.y - self.ball_throw.prev_y;
            let djx = 0.0;
            let djy = goal_post_half_size * 2.0;

            let cross = dix * djy - diy * djx;
            let is_goal = if cross.abs() < 1e-10 {
                false
            } else {
                let dx_start = goal_post_x - self.ball_throw.prev_x;
                let dy_start = goal_post_y - goal_post_half_size - self.ball_throw.prev_y;

                let t = (dx_start * djy - dy_start * djx) / cross;
                let u = (dx_start * diy - dy_start * dix) / cross;

                t >= 0.0 && t <= 1.0 && u >= 0.0 && u <= 1.0
            };

            if is_goal {
                if self.state.name == "play" || self.state.name == "penalty-kick" {
                    print!("{}|{:.2}|", self.addr, (self.time as f32)/100.0);
                    if self.state.name == "play" {
                        print!("Drop scored by team {}\n", self.state.team);
                    } else {
                        print!("Penalty scored by team {}\n", self.state.team);
                    }
                    if self.state.team == 'H' {
                        self.home_team.score += 3;
                    } else {
                        self.away_team.score += 3;
                    }
                }
                if self.state.name == "transformation-kick" {
                    print!("{}|{:.2}|", self.addr, (self.time as f32)/100.0);
                    print!("Penalty scored by team {}\n", self.state.team);
                    if self.state.team == 'H' {
                        self.home_team.score += 2;
                    } else {
                        self.away_team.score += 2;
                    }
                }
                //self.setup_restart()
            }
        }
    }
}