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
    pub x: f32,
    pub y: f32,
    pub number: usize,
    pub ball_pos: bool, // true if player has the ball
    pub size: f32,
    pub strength: f32,
    pub speed: f32,
    pub foot: f32,
    pub p_foot: f32,
    pub p_tackle: f32,
    pub p_scrape: f32,
}

#[derive(Clone, Copy)]
pub struct Ball {
    pub x: f32,
    pub y: f32,
    pub is_carried: bool, // true if the ball is being carried by a player
}

#[derive(Clone)]
pub struct GameState {
    pub field: Field,
    pub time: u64,
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
        let time = 0;
        let home_players = Vec::new();
        let home_bench = Vec::new();
        let away_players = Vec::new();
        let away_bench = Vec::new();
        let ball = Ball { x: 50.0, y: 35.0, is_carried: false };

        Self { field, time, home_players, home_bench, away_players, away_bench, ball }
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
            let x: f32 = (self.field.width / 2 + self.field.try_size - 2) as f32;
            let y: f32 = (6 + i * 3) as f32;
            let size: f32 = info.get(0)
                .and_then(|s| s.split('=').nth(1))
                .and_then(|v| v.parse().ok())
                .unwrap_or(180.0);
            let strength: f32 = info.get(1)
                .and_then(|s| s.split('=').nth(1))
                .and_then(|v| v.parse().ok())
                .unwrap_or(100.0);
            let speed: f32 = info.get(2)
                .and_then(|s| s.split('=').nth(1))
                .and_then(|v| v.parse().ok())
                .unwrap_or(10.0);
            let foot: f32 = info.get(3)
                .and_then(|s| s.split('=').nth(1))
                .and_then(|v| v.parse().ok())
                .unwrap_or(10.0);
            let p_foot: f32 = info.get(4)
                .and_then(|s| s.split('=').nth(1))
                .and_then(|v| v.parse().ok())
                .unwrap_or(10.0);
            let p_tackle: f32 = info.get(5)
                .and_then(|s| s.split('=').nth(1))
                .and_then(|v| v.parse().ok())
                .unwrap_or(10.0);
            let p_scrape: f32 = info.get(6)
                .and_then(|s| s.split('=').nth(1))
                .and_then(|v| v.parse().ok())
                .unwrap_or(10.0);
            if i >= 15 {
                self.home_bench.push(Player { x, y, number: (i + 1), ball_pos: false, size, strength, speed, foot, p_foot, p_tackle, p_scrape});
            } else {
                self.home_players.push(Player { x, y, number: (i + 1), ball_pos: false, size, strength, speed, foot, p_foot, p_tackle, p_scrape});
                if i == 9 {
                    // Set the half opener player as the one with the ball
                    self.home_players[i].ball_pos = true;
                    self.ball.is_carried = true;
                }
            }
        }

        // Initialize away players
        for (i, player) in away_players.iter().enumerate() {
            let info: Vec<&str> = player.split('_').collect();
            let x: f32 = ((self.field.width + 2 * self.field.try_size) * 3 / 4) as f32;
            let y: f32 = (6 + i * 3) as f32;
            let size: f32 = info.get(0)
                .and_then(|s| s.split('=').nth(1))
                .and_then(|v| v.parse().ok())
                .unwrap_or(180.0);
            let strength: f32 = info.get(1)
                .and_then(|s| s.split('=').nth(1))
                .and_then(|v| v.parse().ok())
                .unwrap_or(100.0);
            let speed: f32 = info.get(2)
                .and_then(|s| s.split('=').nth(1))
                .and_then(|v| v.parse().ok())
                .unwrap_or(10.0);
            let foot: f32 = info.get(3)
                .and_then(|s| s.split('=').nth(1))
                .and_then(|v| v.parse().ok())
                .unwrap_or(10.0);
            let p_foot: f32 = info.get(4)
                .and_then(|s| s.split('=').nth(1))
                .and_then(|v| v.parse().ok())
                .unwrap_or(10.0);
            let p_tackle: f32 = info.get(4)
                .and_then(|s| s.split('=').nth(1))
                .and_then(|v| v.parse().ok())
                .unwrap_or(10.0);
            let p_scrape: f32 = info.get(4)
                .and_then(|s| s.split('=').nth(1))
                .and_then(|v| v.parse().ok())
                .unwrap_or(10.0);
            if i >= 15 {
                self.away_bench.push(Player { x, y, number: (i + 1), ball_pos: false, size, strength, speed, foot, p_foot, p_tackle, p_scrape});
            } else {
                self.away_players.push(Player { x, y, number: (i + 1), ball_pos: false, size, strength, speed, foot, p_foot, p_tackle, p_scrape});
            }
        }

        // Initialize ball
        self.ball.x = self.home_players[9].x + 1.0;
        self.ball.y = self.home_players[9].y;
    }

    pub fn play(&mut self, input: String) {
        self.time += 25;

        let actions: Vec<&str> = input.split('\n').collect();
        for action in actions {
            if let Some((player, action)) = action.split_once(':') {
                if action.starts_with("R") {
                    let team = player.chars().next().unwrap();
                    let number = player[1..].parse().unwrap_or(0);
                    let direction = &action[1..];
                    // println!("Player {}{} runs {}", team, number, direction);
                    self.run(team, number, direction);
                }
            }
        }
    }

    pub fn positions(&self) -> String {
        let mut result = String::new();
        if !self.ball.is_carried {
            result.push_str(&format!("B: {} {}\n", self.ball.x, self.ball.y));
        }
        for player in self.home_players.iter() {
            result.push_str(&format!("H{}: {} {}", player.number, player.x, player.y));
            if player.ball_pos {
                result.push_str(&format!("/B: {} {}\n", self.ball.x, self.ball.y));
            } else {
                result.push('\n');
            }
        }
        for player in self.away_players.iter() {
            result.push_str(&format!("A{}: {} {}", player.number, player.x, player.y));
            if player.ball_pos {
                result.push_str(&format!("/B: {} {}\n", self.ball.x, self.ball.y));
            } else {
                result.push('\n');
            }
        }
        result
    }

    pub fn get_drawable(&self) -> Drawable {
        let mut drawable = Drawable::new(self.ball.x, self.ball.y);

        for player in &self.home_players {
            drawable.add_home_player(player.x, player.y, player.number);
        }
        for player in &self.away_players {
            drawable.add_away_player(player.x, player.y, player.number);
        }
        drawable.set_time(self.time);
        return drawable;
    }

    fn run(&mut self, team: char, number: i32, direction: &str) {
        if team == 'H' {
            if let Some(p) = self.home_players.iter_mut().find(|p| p.number == number as usize) {
                match direction {
                    "N" => {
                        if p.x < (self.field.width + (self.field.try_size * 2)) as f32 {
                            p.x += p.speed * 0.069444;
                            if p.ball_pos {
                                self.ball.x += p.speed * 0.069444;
                            }
                        }
                    }
                    "S" => {
                        if p.x > 0.0 {
                            p.x -= p.speed * 0.069444;
                            if p.ball_pos {
                                self.ball.x -= p.speed * 0.069444;
                            }
                        }
                    }
                    "E" => {
                        if p.y < self.field.height  as f32 {
                            p.y += p.speed * 0.069444;
                            if p.ball_pos {
                                self.ball.y += p.speed * 0.069444;
                            }
                        }
                    }
                    "W" => {
                        if p.y > 0.0 {
                            p.y -= p.speed * 0.069444;
                            if p.ball_pos {
                                self.ball.y -= p.speed * 0.069444;
                            }
                        }
                    }
                    "A" => {
                        // Diagonal NW
                        if p.x < (self.field.width + (self.field.try_size * 2)) as f32 {
                            p.x += p.speed * 0.069444;
                            if p.ball_pos {
                                self.ball.x += p.speed * 0.069444;
                            }
                        }
                        if p.y > 0.0 {
                            p.y -= p.speed * 0.069444;
                            if p.ball_pos {
                                self.ball.y -= p.speed * 0.069444;
                            }
                        }
                    }
                    "B" => {
                        // Diagonal NE
                        if p.x < (self.field.width + (self.field.try_size * 2)) as f32 {
                            p.x += p.speed * 0.069444;
                            if p.ball_pos {
                                self.ball.x += p.speed * 0.069444;
                            }
                        }
                        if p.y < (self.field.height) as f32 {
                            p.y += p.speed * 0.069444;
                            if p.ball_pos {
                                self.ball.y += p.speed * 0.069444;
                            }
                        }
                    }
                    "C" => {
                        // Diagonal SW
                        if p.x > 0.0 {
                            p.x -= p.speed * 0.069444;
                            if p.ball_pos {
                                self.ball.x -= p.speed * 0.069444;
                            }
                        }
                        if p.y > 0.0 {
                            p.y -= p.speed * 0.069444;
                            if p.ball_pos {
                                self.ball.y -= p.speed * 0.069444;
                            }
                        }
                    }
                    "D" => {
                        // Diagonal SE
                        if p.x > 0.0 {
                            p.x -= p.speed * 0.069444;
                            if p.ball_pos {
                                self.ball.x -= p.speed * 0.069444;
                            }
                        }
                        if p.y < (self.field.height) as f32 {
                            p.y += p.speed * 0.069444;
                            if p.ball_pos {
                                self.ball.y += p.speed * 0.069444;
                            }
                        }
                    }
                    _ => {}
                }
            }
        } else if team == 'A' {
            if let Some(p) = self.away_players.iter_mut().find(|p| p.number == number as usize) {
                match direction {
                    "N" => {
                        if p.x < (self.field.width + (self.field.try_size * 2)) as f32 {
                            p.x += p.speed * 0.069444;
                            if p.ball_pos {
                                self.ball.x += p.speed * 0.069444;
                            }
                        }
                    }
                    "S" => {
                        if p.x > 0.0 {
                            p.x -= p.speed * 0.069444;
                            if p.ball_pos {
                                self.ball.x -= p.speed * 0.069444;
                            }
                        }
                    }
                    "E" => {
                        if p.y < (self.field.height) as f32 {
                            p.y += p.speed * 0.069444;
                            if p.ball_pos {
                                self.ball.y += p.speed * 0.069444;
                            }
                        }
                    }
                    "W" => {
                        if p.y > 0.0 {
                            p.y -= p.speed * 0.069444;
                            if p.ball_pos {
                                self.ball.y -= p.speed * 0.069444;
                            }
                        }
                    }
                    "A" => {
                        // Diagonal NW
                        if p.x < (self.field.width + (self.field.try_size * 2)) as f32 {
                            p.x += p.speed * 0.069444;
                            if p.ball_pos {
                                self.ball.x += p.speed * 0.069444;
                            }
                        }
                        if p.y > 0.0 {
                            p.y -= p.speed * 0.069444;
                            if p.ball_pos {
                                self.ball.y -= p.speed * 0.069444;
                            }
                        }
                    }
                    "B" => {
                        // Diagonal NE
                        if p.x < (self.field.width + (self.field.try_size * 2)) as f32 {
                            p.x += p.speed * 0.069444;
                            if p.ball_pos {
                                self.ball.x += p.speed * 0.069444;
                            }
                        }
                        if p.y < self.field.height as f32 {
                            p.y += p.speed * 0.069444;
                            if p.ball_pos {
                                self.ball.y += p.speed * 0.069444;
                            }
                        }
                    }
                    "C" => {
                        // Diagonal SW
                        if p.x > 0.0 {
                            p.x -= p.speed * 0.069444;
                            if p.ball_pos {
                                self.ball.x -= p.speed * 0.069444;
                            }
                        }
                        if p.y > 0.0 {
                            p.y -= p.speed * 0.069444;
                            if p.ball_pos {
                                self.ball.y -= p.speed * 0.069444;
                            }
                        }
                    }
                    "D" => {
                        // Diagonal SE
                        if p.x > 0.0 {
                            p.x -= p.speed * 0.069444;
                            if p.ball_pos {
                                self.ball.x -= p.speed * 0.069444;
                            }
                        }
                        if p.y < self.field.height as f32 {
                            p.y += p.speed * 0.069444;
                            if p.ball_pos {
                                self.ball.y += p.speed * 0.069444;
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
    }
}
