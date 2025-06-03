use crate::gui::drawable::Drawable;

#[derive(Clone)]
pub struct Field {
    pub width: usize,
    pub height: usize,
    pub try_size: usize,
    pub is_switch: bool,
    pub switch_time: usize,
    pub switch_home: Vec<(usize, usize)>,
    pub switch_away: Vec<(usize, usize)>,
    pub wind_strength: usize,
    pub wind_direction: usize, // 0-360 degrees
    pub weather: usize, // 0-100 for rain,
}

#[derive(Clone, Copy)]
pub struct Player {
    pub x: usize,
    pub y: usize,
    pub team: u8, // 1 or 2
    pub number: usize,
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

#[derive(Clone)]
pub struct GameState {
    pub field: Field,
    pub home_players: Vec<Player>,
    pub home_bench: Vec<Player>,
    pub away_players: Vec<Player>,
    pub away_bench: Vec<Player>,
    pub ball: Ball,
}

impl GameState {
    pub fn new() -> Self {
        let field = Field {
            width: 0,
            height: 0,
            try_size: 0,
            is_switch: false,
            switch_time: 0,
            switch_home: Vec::new(),
            switch_away: Vec::new(),
            wind_strength: 0,
            wind_direction: 0,
            weather: 0,
        };
        let home_players = Vec::new();
        let home_bench = Vec::new();
        let away_players = Vec::new();
        let away_bench = Vec::new();
        let ball = Ball { x: 50, y: 35 };

        Self { field, home_players, home_bench, away_players, away_bench, ball }
    }

    pub fn initialize(&mut self, field: String, home_players: Vec<String>, away_players: Vec<String>) {
        // Parse the field and players from the input

        // Initialize field
        let field_info: Vec<&str> = field.split('_').collect();
        self.field.width = field_info.get(0)
            .and_then(|s| s.split('=').nth(1))
            .and_then(|v| v.parse().ok())
            .unwrap_or(100);
        self.field.height = field_info.get(1)
            .and_then(|s| s.split('=').nth(1))
            .and_then(|v| v.parse().ok())
            .unwrap_or(70);
        self.field.try_size = field_info.get(2)
            .and_then(|s| s.split('=').nth(1))
            .and_then(|v| v.parse().ok())
            .unwrap_or(10);
        self.field.is_switch = field_info.get(3)
            .and_then(|s| s.split('=').nth(1))
            .and_then(|v| v.parse().ok())
            .unwrap_or(false);
        self.field.switch_time = field_info.get(4)
            .and_then(|s| s.split('=').nth(1))
            .and_then(|v| v.parse().ok())
            .unwrap_or(40);
        self.field.switch_home = field_info.get(5)
            .and_then(|s| s.split('=').nth(1))
            .map(|v| {
                v.split('/')
                    .filter_map(|pair| {
                        let mut coords = pair.split('-').filter_map(|x| x.parse::<usize>().ok());
                        Some((coords.next()?, coords.next()?))
                    })
                    .collect::<Vec<(usize, usize)>>()
            })
            .unwrap_or_else(Vec::new);
        self.field.switch_away = field_info.get(6)
            .and_then(|s: &&str| s.split('=').nth(1))
            .map(|v| {
                v.split('/')
                    .filter_map(|pair| {
                        let mut coords = pair.split('-').filter_map(|x| x.parse::<usize>().ok());
                        Some((coords.next()?, coords.next()?))
                    })
                    .collect::<Vec<(usize, usize)>>()
            })
            .unwrap_or_else(Vec::new);
        self.field.wind_strength = field_info.get(7)
            .and_then(|s| s.split('=').nth(1))
            .and_then(|v| v.parse().ok())
            .unwrap_or(0);
        self.field.wind_direction = field_info.get(8)
            .and_then(|s| s.split('=').nth(1))
            .and_then(|v| v.parse().ok())
            .unwrap_or(0);
        self.field.weather = field_info.get(9)
            .and_then(|s| s.split('=').nth(1))
            .and_then(|v| v.parse().ok())
            .unwrap_or(0);

        // Initialize home players
        for (i, player) in home_players.iter().enumerate() {
            let info: Vec<&str> = player.split('_').collect();
            let x: usize = self.field.width / 2 + self.field.try_size - 2;
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
            let p_tackle: usize = info.get(5)
                .and_then(|s| s.split('=').nth(1))
                .and_then(|v| v.parse().ok())
                .unwrap_or(10);
            let p_scrape: usize = info.get(6)
                .and_then(|s| s.split('=').nth(1))
                .and_then(|v| v.parse().ok())
                .unwrap_or(10);
            if i >= 15 {
                self.home_bench.push(Player { x, y, team: 1, number: (i + 1), size, strength, speed, foot, p_foot, p_tackle, p_scrape});
            } else {
                self.home_players.push(Player { x, y, team: 1, number: (i + 1), size, strength, speed, foot, p_foot, p_tackle, p_scrape});
            }
        }

        // Initialize away players
        for (i, player) in away_players.iter().enumerate() {
            let info: Vec<&str> = player.split('_').collect();
            let x: usize = (self.field.width + 2 * self.field.try_size) * 3 / 4;
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
                self.away_bench.push(Player { x, y, team: 2, number: (i + 1), size, strength, speed, foot, p_foot, p_tackle, p_scrape});
            } else {
                self.away_players.push(Player { x, y, team: 2, number: (i + 1), size, strength, speed, foot, p_foot, p_tackle, p_scrape});
            }
        }

        // Initialize ball
        self.ball.x = self.field.width / 2 + self.field.try_size - 1;
        self.ball.y = self.field.height / 2;
    }

    pub fn positions(&mut self) -> String {
        let mut result = String::new();
        result.push_str(&format!("B: {} {}\n", self.ball.x, self.ball.y));
        for player in &self.home_players {
            result.push_str(&format!("H{}: {} {}\n",
                player.number, player.x, player.y));
        }
        for player in &self.away_players {
            result.push_str(&format!("A{}: {} {}\n",
                player.number, player.x, player.y));
        }
        return result;
    }

    pub fn get_drawable(&self) -> Drawable {
        let mut drawable = Drawable::new(self.ball.x, self.ball.y);

        for player in &self.home_players {
            drawable.add_home_player(player.x, player.y, player.number);
        }
        for player in &self.away_players {
            drawable.add_away_player(player.x, player.y, player.number);
        }
        return drawable;
    }

    pub fn test(&mut self) {
        // Simple random movement placeholder (for testing)
        for player in &mut self.home_players {
            player.x = (player.x + 1) % 75;
            if player.x == 0 {
                player.x = 10
            }
        }
        for player in &mut self.away_players {
            player.x = (player.x + 1) % 75;
            if player.x == 0 {
                player.x = 10
            }
        }
        self.ball.x = (self.ball.x + 1) % 75;
        if self.ball.x == 0 {
            self.ball.x = 10
        }
    }
}
