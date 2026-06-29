use crate::game::game_state::GameState;

impl GameState {

    pub fn transformation(&mut self, input: String) {

        let info: Vec<&str> = input.split('\n').collect();
        let action: char = info[1].chars().nth(0).unwrap_or('\0');
        match action {
            'K' => {
                let action: Vec<&str> = info[1][1..].split('/').collect();
                let number: usize = action[1].parse().unwrap_or(0);
                let distance: f32 = action[2].parse().unwrap_or(0.0);
                let direction: f32 = action[3].parse().unwrap_or(0.0);
                let high: f32 = action[4].parse().unwrap_or(0.0);
                self.setup_transformation(number, distance, direction, high);
            },
            _ => {
                print!("{}|T{:.2}|", self.addr, (self.time as f32)/100.0);
                print!("Unknown action in transformation: {}\n", action);
            }
        }
    }
}