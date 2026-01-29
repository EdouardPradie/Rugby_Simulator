use crate::game::game_state::GameState;
use crate::gui::drawable::Drawable;

impl GameState {
    pub fn get_drawable(&self) -> Drawable {
        let mut drawable = Drawable::new(self.ball.x, self.ball.y);

        for player in &self.home_team.players {
            drawable.add_home_player(player.x, player.y, player.number);
        }
        for player in &self.away_team.players {
            drawable.add_away_player(player.x, player.y, player.number);
        }
        if self.state.name == "scrum" {
            drawable.set_state(self.state.x, self.state.y, self.state.name.clone(), self.state.size);
        }
        if self.state.name == "ruck" {
            drawable.set_state(self.state.x, self.state.y, self.state.name.clone(), self.state.size);
        }
        drawable.home_score = self.home_team.score as usize;
        drawable.away_score = self.away_team.score as usize;
        drawable.set_time(self.time);
        return drawable;
    }
}