use crate::game::game_state::GameState;

impl GameState {
    pub fn throw_from_offside(&mut self, team: char, number: i32, angle: f32) {
        println!("{} {} {}", team, number, angle);
    }

    pub fn jump_offside(&mut self, team: char, number: i32) {
        println!("{} {}", team, number);
    }

    pub fn lift_offside(&mut self, team: char, number: i32) {
        println!("{} {}", team, number);
    }
}