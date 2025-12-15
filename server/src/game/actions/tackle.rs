use crate::game::game_state::GameState;

impl GameState {
    pub fn tackle(&mut self, team: char, number: i32) {
        if !self.ball.is_carried {
            print!("No player has the ball to tackle\n");
            return;
        }
        let (players, opponents) = if team == 'H' {
            (&mut self.home_players, &mut self.away_players)
        } else {
            (&mut self.away_players, &mut self.home_players)
        };

        if let Some(p) = players.iter_mut().find(|p| p.ball_pos) {
            print!("No player in the other team have the ball because {} have the ball\n", p.number);
            return;
        }
        if let Some(p) = players.iter_mut().find(|p| p.number == number as usize) {
            if let Some(o) = opponents.iter_mut().find(|p| p.ball_pos) {
                let distance = ((p.x - o.x).powi(2) + (p.y - o.y).powi(2)).sqrt();
                if distance < 1.2 {
                    let is_successful = rand::random::<f32>() * 100.0 < p.p_tackle;
                    if is_successful {
                        p.is_tackle = true;
                        print!("Tackle successful by player {} {}\n", team, number);
                        let ruck_team = if team == 'H' { 'A' } else { 'H' };
                        let ruck_x = o.x;
                        let ruck_y = o.y;
                        o.ball_pos = false;
                        self.set_ruck(ruck_x, ruck_y, ruck_team);
                    } else {
                        print!("Tackle failed by player {} {}\n", team, number);
                    }
                } else {
                    print!("Player {} {} is too far to tackle\n", team, number);
                }
            }
        }
    }

    fn set_ruck(&mut self, x: f32, y: f32, team: char) {
        self.ball.is_carried = false;
        self.ball.x = x - 0.5;
        self.ball.y = y;
        self.ball.z = 0.0;
        self.state.name = "ruck".to_string();
        self.state.x = x;
        self.state.y = y;
        self.state.team = team;
        print!("Ruck formed at position {} {}\n", x, y);
    }
}