use crate::game::game_state::GameState;

impl GameState {

    pub fn ruck(&mut self, input: String) {
        self.time += 25;
        let mut is_offside = false;

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
                        is_offside = self.try_catch_ball_in_ruck(team, number);
                    },
                    'S' => continue,
                    _ => {
                        print!("Unknown action in ruck: {}\n", action);
                    },
                }
            }
        }

        if is_offside {
            self.setup_penalty(self.state.team, self.state.x, self.state.y);
        }

        if self.check_tackler() {
            self.setup_penalty(if self.state.team == 'H' { 'A' } else { 'H' } , self.state.x, self.state.y);
        }
        self.check_scrap();
    }
}