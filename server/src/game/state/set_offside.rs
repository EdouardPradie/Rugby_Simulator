use crate::game::game_state::GameState;

impl GameState {

    pub fn set_offside(&mut self, input: String) {

        let info: Vec<&str> = input.split('\n').collect();
        let action: char = info[1].chars().nth(0).unwrap_or('\0');
        match action {
            'O' => {
                let action: Vec<&str> = info[1][1..].split('/').collect();
                let number: usize = action[1].parse().unwrap_or(0);

                self.setup_offside(number, action[2].to_string().split('-').collect(), action[3].to_string().split('-').collect());
            },
            _ => {
                print!("{}|{:.2}|{}|", self.addr, (self.time as f32)/100.0, self.state.name);
                print!("Unknown action in transformation: {}\n", action);
            }
        }
    }
}