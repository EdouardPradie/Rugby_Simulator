use crate::game::game_state::GameState;

impl GameState {
    pub fn positions(&self) -> String {
        let mut result = String::new();
        if !self.ball.is_carried {
            result.push_str(&format!("B: {} {}\n", self.ball.x, self.ball.y));
        }
        for player in self.home_team.players.iter() {
            result.push_str(&format!("H{}: {} {}", player.number, player.x, player.y));
            if player.ball_pos {
                result.push_str(&format!("/B: {} {}\n", self.ball.x, self.ball.y));
            } else {
                result.push('\n');
            }
        }
        for player in self.away_team.players.iter() {
            result.push_str(&format!("A{}: {} {}", player.number, player.x, player.y));
            if player.ball_pos {
                result.push_str(&format!("/B: {} {}\n", self.ball.x, self.ball.y));
            } else {
                result.push('\n');
            }
        }
        result
    }
}