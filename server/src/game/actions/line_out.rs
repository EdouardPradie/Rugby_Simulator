use crate::game::game_state::GameState;

impl GameState {
    pub fn throw_from_line_out(&mut self, team: char, number: i32, angle: f32) {
        println!("{} {} {}", team, number, angle);
    }

    pub fn jump_line_out(&mut self, team: char, number: i32) {
        println!("{} {}", team, number);
    }

    pub fn lift_line_out(&mut self, team: char, number: i32) {
        println!("{} {}", team, number);
    }
}