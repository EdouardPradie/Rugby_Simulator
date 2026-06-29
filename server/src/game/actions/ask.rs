use crate::game::game_state::GameState;

impl GameState {
    pub fn ask_transformation(&mut self, team: char) {
        self.state.name = "set-transformation".to_string();
        self.state.x = self.ball.x;
        self.state.y = self.ball.y;
        self.state.team = team;
    }

    pub fn ask_line_out(&mut self, team: char) {
        self.state.name = "set-line_out".to_string();
        if self.ball.y < 1.0 {
            self.state.y = 0.5;
        } else if self.ball.y > self.field.height as f32 + 1.0 {
            self.state.y = self.field.height as f32 + 1.5;
        } else {
            print!("{}|{:.2}|{}|", self.addr, (self.time as f32)/100.0, self.state.name);
            print!("An error happen when the ball is out {} {}\n", self.ball.x, self.ball.y);
            return;
        }

        if self.ball.is_carried && !self.ball_throw.active {
            self.state.x = self.ball.x;
        } else {
            let dx = self.ball_throw.prev_x - self.ball.x;
            let dy = self.ball_throw.prev_y - self.ball.y;

            if dx.abs() > f32::EPSILON {
                let slope = dy / dx;
                self.state.x = self.ball.x + (self.state.y - self.ball.y) / slope;
            } else {
                self.state.x = self.ball.x; // Vertical line case
            }

            if self.state.x > self.field.try_size as f32 + 1.0 &&
            self.state.x < self.field.try_size as f32 + 6.0 {
                self.state.x = self.field.try_size as f32 + 6.0;
            }
            if self.state.x > self.field.width as f32 +
            self.field.try_size as f32 - 4.0 &&
            self.state.x < self.field.width as f32 +
            self.field.try_size as f32 + 1.0 {
                self.state.x = self.field.width as f32 + self.field.try_size as f32 - 4.0;
            }
        }
        self.state.team = team;
    }
}