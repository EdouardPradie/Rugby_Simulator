use crate::game::game_state::GameState;

impl GameState {

    pub fn penalty(&mut self, input: String) {

        let info: Vec<&str> = input.split('\n').collect();
        let action: char = info[1].chars().nth(0).unwrap_or('\0');
        match action {
            'P' => {
                let action: Vec<&str> = info[1][1..].split('/').collect();
                let number: usize = action[1].parse().unwrap_or(0);
                let direction: f32 = action[2].parse().unwrap_or(0.0);
                let high: f32 = action[3].parse().unwrap_or(0.0);
                self.setup_penalty_kick(number, direction, high);
            },
            'K' => {
                let action: Vec<&str> = info[1][1..].split('/').collect();
                let number: usize = action[1].parse().unwrap_or(0);
                let direction: f32 = action[2].parse().unwrap_or(0.0);
                let high: f32 = action[3].parse().unwrap_or(0.0);
                self.setup_free_kick(number, direction, high);
            },
            'S' => {
                self.setup_scrum(self.state.team, self.state.x, self.state.y);
            },
            _ => {
                print!("Unknown action in penalty: {}\n", action);
            }
        }
    }
}