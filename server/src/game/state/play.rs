use crate::game::game_state::GameState;

impl GameState {

    pub fn play(&mut self, input: String) {
        let mut update = false;
        self.time += 25;
        if self.state.name == "start" {
            self.state.name = "play".to_string();
        }

        let actions: Vec<&str> = input.split('\n').collect();
        for action in actions {
            if let Some((player, action)) = action.split_once(':') {
                if update {
                    return;
                }
                let tmp = action.chars().nth(0).unwrap_or('\0');
                match tmp {
                    'R' => {
                        let team = player.chars().next().unwrap();
                        let number = player[1..].parse().unwrap_or(0);
                        let direction = action[1..].parse().unwrap_or(0.0);
                        // println!("Player {} {} runs {}", team, number, direction);
                        self.run(team, number, direction, true);
                    },
                    'W' => {
                        let team = player.chars().next().unwrap();
                        let number = player[1..].parse().unwrap_or(0);
                        let direction = action[1..].parse().unwrap_or(0.0);
                        // println!("Player {} {} walk {}", team, number, direction);
                        self.run(team, number, direction, false);
                    },
                    'T' => {
                        let team = player.chars().next().unwrap();
                        let number = player[1..].parse().unwrap_or(0);
                        self.tackle(team, number);
                    },
                    'K' => {
                        let team = player.chars().next().unwrap();
                        let number = player[1..].parse().unwrap_or(0);
                        let tmp: Vec<&str> = action[1..].split('/').collect();
                        let direction = tmp[0].parse().unwrap_or(0.0);
                        let high = tmp[1].parse().unwrap_or(0.0);
                        println!("Player {} {} Kick in way {} at {}", team, number, direction, high);
                        self.kick(team, number, direction, high);
                    },
                    'P' => {
                        let team = player.chars().next().unwrap();
                        let number = player[1..].parse().unwrap_or(0);
                        let tmp: Vec<&str> = action[1..].split('/').collect();
                        let direction = tmp[0].parse().unwrap_or(0.0);
                        println!("Player {} {} Pass in way {}", team, number, direction);
                        update = self.pass(team, number, direction);
                    },
                    'S' => continue,
                    _ => {
                        print!("Unknown action: {}\n", action);
                    },
                }
            }
        }

        self.update_ball_velocity();
        self.update_ball_carrie();
        self.check_ball_position();
    }
}