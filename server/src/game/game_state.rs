#[derive(Clone, Copy)]
pub struct Player {
    pub x: usize,
    pub y: usize,
    pub team: u8, // 1 or 2
}

#[derive(Clone, Copy)]
pub struct Ball {
    pub x: usize,
    pub y: usize,
}

pub struct GameState {
    pub players: Vec<Player>,
    pub ball: Ball,
}

impl GameState {
    pub fn new() -> Self {
        let mut players = Vec::new();
        // Initialize 15 players for each team
        for i in 0..15 {
            players.push(Player { x: 5, y: i * 3 + 5, team: 1 });
            players.push(Player { x: 70, y: i * 3 + 5, team: 2 });
        }
        let ball = Ball { x: 37, y: 30 };

        Self { players, ball }
    }

    pub fn update(&mut self) {
        // Simple random movement placeholder (for testing)
        for player in &mut self.players {
            player.x = (player.x + 1) % 75;
        }
        self.ball.x = (self.ball.x + 1) % 75;
    }
}
