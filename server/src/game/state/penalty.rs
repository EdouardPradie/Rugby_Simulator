use crate::game::game_state::GameState;

impl GameState {

    pub fn penalty(&mut self, input: String) {
        self.time += 25;

        let actions: Vec<&str> = input.split('\n').collect();
        for action in actions {
            if let Some((player, action)) = action.split_once(':') {
                let tmp = action.chars().nth(0).unwrap_or('\0');
                match tmp {
                    'K' => {
                    },
                    'M' => {
                    },
                    'T' => {
                    },
                    'H' => {
                    },
                    _ => {
                        print!("Unknown action in penalty: {}\n", action);
                    },
                }
            }
        }

        self.check_tackler();
        self.check_scrap();
    }
}