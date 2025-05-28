#[derive(Clone, Copy)]
pub struct Player {
    pub x: usize,
    pub y: usize,
    pub team: u8, // 1 or 2
    pub size: usize,
    pub strength: usize,
    pub speed: usize,
    pub foot: usize,
    pub p_foot: usize,
    pub p_tackle: usize,
    pub p_scrape: usize,
}

#[derive(Clone, Copy)]
pub struct Ball {
    pub x: usize,
    pub y: usize,
}

pub struct GameState {
    pub home_players: Vec<Player>,
    pub home_bench: Vec<Player>,
    pub away_players: Vec<Player>,
    pub away_bench: Vec<Player>,
    pub ball: Ball,
}

impl GameState {
    pub fn new() -> Self {
        let home_players = Vec::new();
        let home_bench = Vec::new();
        let away_players = Vec::new();
        let away_bench = Vec::new();
        let ball = Ball { x: 50, y: 35 };

        Self { home_players, home_bench, away_players, away_bench, ball }
    }

    pub fn initialize(&mut self, home_players: Vec<String>, away_players: Vec<String>) {
        // Parse the field and players from the input
        // For simplicity, we assume the field is a string of underscores and newlines
        for (i, player) in home_players.iter().enumerate() {
            let info: Vec<&str> = player.split('_').collect();
            let x: usize = 30;
            let y: usize = 6 + i * 3;
            let size: usize = info.get(0)
                .and_then(|s| s.split('=').nth(1))
                .and_then(|v| v.parse().ok())
                .unwrap_or(180);
            let strength: usize = info.get(1)
                .and_then(|s| s.split('=').nth(1))
                .and_then(|v| v.parse().ok())
                .unwrap_or(100);
            let speed: usize = info.get(2)
                .and_then(|s| s.split('=').nth(1))
                .and_then(|v| v.parse().ok())
                .unwrap_or(10);
            let foot: usize = info.get(3)
                .and_then(|s| s.split('=').nth(1))
                .and_then(|v| v.parse().ok())
                .unwrap_or(10);
            let p_foot: usize = info.get(4)
                .and_then(|s| s.split('=').nth(1))
                .and_then(|v| v.parse().ok())
                .unwrap_or(10);
            let p_tackle: usize = info.get(4)
                .and_then(|s| s.split('=').nth(1))
                .and_then(|v| v.parse().ok())
                .unwrap_or(10);
            let p_scrape: usize = info.get(4)
                .and_then(|s| s.split('=').nth(1))
                .and_then(|v| v.parse().ok())
                .unwrap_or(10);
            if i >= 15 {
                self.home_bench.push(Player { x, y, team: 1, size, strength, speed, foot, p_foot, p_tackle, p_scrape});
            } else {
                self.home_players.push(Player { x, y, team: 1, size, strength, speed, foot, p_foot, p_tackle, p_scrape});
            }
        }

        for (i, player) in away_players.iter().enumerate() {
            let info: Vec<&str> = player.split('_').collect();
            let x: usize = 70;
            let y: usize = 6 + i * 3;
            let size: usize = info.get(0)
                .and_then(|s| s.split('=').nth(1))
                .and_then(|v| v.parse().ok())
                .unwrap_or(180);
            let strength: usize = info.get(1)
                .and_then(|s| s.split('=').nth(1))
                .and_then(|v| v.parse().ok())
                .unwrap_or(100);
            let speed: usize = info.get(2)
                .and_then(|s| s.split('=').nth(1))
                .and_then(|v| v.parse().ok())
                .unwrap_or(10);
            let foot: usize = info.get(3)
                .and_then(|s| s.split('=').nth(1))
                .and_then(|v| v.parse().ok())
                .unwrap_or(10);
            let p_foot: usize = info.get(4)
                .and_then(|s| s.split('=').nth(1))
                .and_then(|v| v.parse().ok())
                .unwrap_or(10);
            let p_tackle: usize = info.get(4)
                .and_then(|s| s.split('=').nth(1))
                .and_then(|v| v.parse().ok())
                .unwrap_or(10);
            let p_scrape: usize = info.get(4)
                .and_then(|s| s.split('=').nth(1))
                .and_then(|v| v.parse().ok())
                .unwrap_or(10);
            if i >= 15 {
                self.away_bench.push(Player { x, y, team: 2, size, strength, speed, foot, p_foot, p_tackle, p_scrape});
            } else {
                self.away_players.push(Player { x, y, team: 2, size, strength, speed, foot, p_foot, p_tackle, p_scrape});
            }
        }
    }

    pub fn update(&mut self) {
        // Simple random movement placeholder (for testing)
        for player in &mut self.home_players {
            player.x = (player.x + 1) % 75;
        }
        for player in &mut self.away_players {
            player.x = (player.x + 1) % 75;
        }
        self.ball.x = (self.ball.x + 1) % 75;
    }
}
