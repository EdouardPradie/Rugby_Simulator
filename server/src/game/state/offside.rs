use crate::game::game_state::GameState;
use crate::game::constants::*;

impl GameState {

    pub fn offside(&mut self, input: String) {
        self.time += 25;

        if self.state.size != SCRUM_SIZE {
            self.state.size = SCRUM_SIZE;
        }

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
                        self.run_offside(team, number, direction, true);
                    },
                    'W' => {
                        let team = player.chars().next().unwrap();
                        let number = player[1..].parse().unwrap_or(0);
                        let direction = action[1..].parse().unwrap_or(0.0);
                        // println!("Player {} {} walk {}", team, number, direction);
                        self.run_offside(team, number, direction, false);
                    },
                    'T' => {
                        let team = player.chars().next().unwrap();
                        let number = player[1..].parse().unwrap_or(0);
                        let angle = action[1..].parse().unwrap_or(0.0);
                        self.throw_from_offside(team, number, angle);
                    },
                    'J' => {
                        let team = player.chars().next().unwrap();
                        let number = player[1..].parse().unwrap_or(0);
                        self.jump_offside(team, number);
                    },
                    'L' => {
                        let team = player.chars().next().unwrap();
                        let number = player[1..].parse().unwrap_or(0);
                        self.lift_offside(team, number);
                    },
                    'S' => continue,
                    _ => {
                        print!("{}|{:.2}|{}|", self.addr, (self.time as f32)/100.0, self.state.name);
                        print!("Unknown action in offside: {}\n", action);
                    },
                }
            }
        }
    }
}