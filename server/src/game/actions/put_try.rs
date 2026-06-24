use crate::game::game_state::GameState;

impl GameState {
    pub fn put_try(&mut self, team: char, number: i32) {
        if !self.ball.is_carried {
            print!("{}|{:.2}|{}|", self.addr, (self.time as f32)/100.0, self.state.name);
            print!("No player has the ball to put a try\n");
            return;
        }

        if self.ball.y <= 1.0 && self.ball.y >= self.field.height as f32 + 1.0 {
            print!("{}|{:.2}|{}|", self.addr, (self.time as f32)/100.0, self.state.name);
            print!("Wait to check ball out off bounds {}\n", self.ball.y);
            return;
        }

        let try_team = if team == 'H' { &mut self.home_team } else { &mut self.away_team };
        let players = &mut try_team.players;
        if let Some(p) = players.iter_mut().find(|p| p.number == number as usize) {
            if p.ball_pos {

                if (team == 'H' && self.field.home_direction_try == 'N') ||
                (team == 'A' && self.field.home_direction_try == 'S') {
                    if self.ball.x > self.field.width as f32 + self.field.try_size as f32 + 1.0 &&
                    self.ball.x < self.field.width as f32 + (self.field.try_size as f32 * 2.0) + 1.0 {
                        try_team.try_scored += 1;
                        try_team.score += 5;
                        print!("{}|{:.2}|{}|", self.addr, (self.time as f32)/100.0, self.state.name);
                        print!("Try scored by player {} {}\n", team, number);
                        self.ask_transformation(team)
                    } else {
                        print!("{}|{:.2}|{}|", self.addr, (self.time as f32)/100.0, self.state.name);
                        print!("Player {} {} is too far from the goal line to put a try\n", team, number);
                    };
                } else {
                    if self.ball.x > 1.0 &&
                    self.ball.x < self.field.try_size as f32 + 1.0 {
                        try_team.try_scored += 1;
                        try_team.score += 5;
                        print!("{}|{:.2}|{}|", self.addr, (self.time as f32)/100.0, self.state.name);
                        print!("Try scored by player {} {}\n", team, number);
                        self.ask_transformation(team)
                    } else {
                        print!("{}|{:.2}|{}|", self.addr, (self.time as f32)/100.0, self.state.name);
                        print!("Player {} {} is too far from the goal line to put a try\n", team, number);
                    };
                };
            } else {
                print!("{}|{:.2}|{}|", self.addr, (self.time as f32)/100.0, self.state.name);
                print!("Player {} {} does not have the ball to put a try\n", team, number);
            }
        } else {
            print!("{}|{:.2}|{}|", self.addr, (self.time as f32)/100.0, self.state.name);
            print!("No player in the other team have this number {}\n", number);
        }
        return;
    }
}