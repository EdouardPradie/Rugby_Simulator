use crate::game::game_state::GameState;
use crate::game::constants::*;

impl GameState {

    pub fn scrum(&mut self, input: String) {
        self.time += 25;
        let mut scrum_h_pound = 0.0;
        let mut scrum_a_pound = 0.0;

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
                        let players = if team == 'H' {
                            &mut self.home_players
                        } else {
                            &mut self.away_players
                        };
                        let player_index = players.iter()
                        .position(|p| p.number == number as usize)
                        .unwrap_or(15);
                        if player_index >= 7 && player_index < 15 {
                            self.run(team, number, direction, true);
                        } else  {
                            print!("Player {} {} cannot run during scrum\n", team, number);
                        }
                    },
                    'W' => {
                        let team = player.chars().next().unwrap();
                        let number = player[1..].parse().unwrap_or(0);
                        let direction = action[1..].parse().unwrap_or(0.0);
                        // println!("Player {} {} walk {}", team, number, direction);
                        let players = if team == 'H' {
                            &mut self.home_players
                        } else {
                            &mut self.away_players
                        };
                        let player_index = players.iter()
                        .position(|p| p.number == number as usize)
                        .unwrap_or(15);
                        if player_index >= 7 && player_index < 15 {
                            self.run(team, number, direction, false);
                        } else  {
                            print!("Player {} {} cannot walk during scrum\n", team, number);
                        }
                    },
                    'T' => {
                        let team = player.chars().next().unwrap();
                        let number = player[1..].parse().unwrap_or(0);
                        let players = if team == 'H' {
                            &mut self.home_players
                        } else {
                            &mut self.away_players
                        };
                        let player_index = players.iter()
                        .position(|p| p.number == number as usize)
                        .unwrap_or(15);
                        if player_index >= 9 && player_index < 15 {
                            print!("Player {} {} is not in scrum\n", team, number);
                        } else if player_index <= 6 {
                            print!("Player {} {} add to push in scrum\n", team, number);
                        } else if player_index == 7 || player_index == 8 {
                            self.try_catch_ball_in_scrum(team, number);
                        } else {
                            continue;
                        }
                    },
                    'S' => {
                        let team = player.chars().next().unwrap();
                        let number = player[1..].parse().unwrap_or(0);
                        // println!("Player {} {} stop or scrum", team, number);
                        let players = if team == 'H' {
                            &mut self.home_players
                        } else {
                            &mut self.away_players
                        };
                        let player_index = players.iter()
                        .position(|p| p.number == number as usize)
                        .unwrap_or(15);
                        if player_index <= 7 {
                            if team == 'H' {
                                scrum_h_pound += players[player_index].pound;
                            } else {
                                scrum_a_pound += players[player_index].pound;
                            }
                        } else  {
                            continue;
                        }
                    },
                    _ => {
                        print!("Unknown action in scrum: {}\n", action);
                    },
                }
            }
        }
        self.update_ball_position_scrum(scrum_h_pound, scrum_a_pound);
        self.check_ball_out_of_scrum();
    }
}