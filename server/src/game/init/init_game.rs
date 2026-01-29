use crate::game::game_state::GameState;
use crate::game::models::*;

impl GameState {
    pub fn new() -> Self {
        let state = State {
            name: String::from("start"),
            team: 'H',
            size: 0.0,
            x: 0.0,
            y: 0.0,
        };
        let field = Field {
            width: 0,
            height: 0,
            try_size: 0,
            home_direction_try: 'N',
            is_switch: false,
            switch_time: 0,
            switch_home: Vec::new(),
            switch_away: Vec::new(),
            wind_strength: 0,
            wind_direction: 0,
            weather: 0,
        };
        let time = 0;
        let home_team = Team {
            players: Vec::new(),
            bench: Vec::new(),
            score: 0,
            try_scored: 0,
            transformation: 0,
            penalty: 0,
        };
        let away_team = Team {
            players: Vec::new(),
            bench: Vec::new(),
            score: 0,
            try_scored: 0,
            transformation: 0,
            penalty: 0,
        };
        let ball: Ball = Ball { x: 50.0, y: 35.0, z: 1.0, is_carried: false };
        let ball_throw = BallThrow { vx: 0.0, vy: 0.0, vz: 0.0, active: false };

        Self { state, field, time, home_team, away_team, ball, ball_throw }
    }

    //INIT

    pub fn initialize(&mut self, field: String, home_players: Vec<String>, away_players: Vec<String>, state: String) {
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
        self.field.home_direction_try = field_info.get(3)
            .and_then(|s| s.split('=').nth(1))
            .and_then(|v| v.chars().next())
            .unwrap_or('N');
        self.field.is_switch = field_info.get(4)
        .and_then(|s| s.split('=').nth(1))
        .and_then(|v| v.parse().ok())
        .unwrap_or(false);
        self.field.switch_time = field_info.get(5)
            .and_then(|s| s.split('=').nth(1))
            .and_then(|v| v.parse().ok())
            .unwrap_or(40);
        self.field.switch_home = field_info.get(6)
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
        self.field.switch_away = field_info.get(7)
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
        self.field.wind_strength = field_info.get(8)
            .and_then(|s| s.split('=').nth(1))
            .and_then(|v| v.parse().ok())
            .unwrap_or(0);
        self.field.wind_direction = field_info.get(9)
            .and_then(|s| s.split('=').nth(1))
            .and_then(|v| v.parse().ok())
            .unwrap_or(0);
        self.field.weather = field_info.get(10)
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
            let pound: f32 = info.get(1)
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
            if i >= 15 {
                self.home_team.bench.push(Player { x, y, number: (i + 1), ball_pos: false, size, pound, speed, foot, p_foot, p_tackle, is_tackle: false, is_tackler: false });
            } else {
                self.home_team.players.push(Player { x, y, number: (i + 1), ball_pos: false, size, pound, speed, foot, p_foot, p_tackle, is_tackle:false, is_tackler: false });
                if i == 9 {
                    // Set the half opener player as the one with the ball
                    self.home_team.players[i].ball_pos = true;
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
            let pound: f32 = info.get(1)
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
            if i >= 15 {
                self.home_team.bench.push(Player { x, y, number: (i + 1), ball_pos: false, size, pound, speed, foot, p_foot, p_tackle, is_tackle:false, is_tackler: false });
            } else {
                self.away_team.players.push(Player { x, y, number: (i + 1), ball_pos: false, size, pound, speed, foot, p_foot, p_tackle, is_tackle:false, is_tackler: false });
            }
        }

        // Initialize ball
        self.ball.x = self.home_team.players[9].x + 1.0;
        self.ball.y = self.home_team.players[9].y;

        //Initialize state
        self.init_state(state);
    }

    fn init_state(&mut self, state: String) {
        let state_info: Vec<&str> = state.split('_').collect();

        for (i, info) in state_info.iter().enumerate() {
            if i == 0 {
                let part: Vec<&str> = info.split(' ').collect();
                self.state.name = part.get(0)
                    .and_then(|s| Some(s.to_string()))
                    .unwrap_or("start".to_string());
                self.state.team = part.get(1)
                    .and_then(|s| s.chars().next())
                    .unwrap_or('H');
                self.state.x = part.get(2)
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(0.0);
                self.state.y = part.get(3)
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(0.0);
                self.state.size = part.get(4)
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(0.0);

                if self.state.name == "ruck" {
                    let tackle = part.get(5)
                    .and_then(|s| Some(s.to_string()))
                    .unwrap_or("".to_string());
                    let team_tackle = tackle.chars().next().unwrap();
                    let number_tackle = tackle[1..].parse().unwrap_or(0);
                    if team_tackle == 'H' {
                        if let Some(p) = self.home_team.players.iter_mut().find(|p| p.number == number_tackle as usize) {
                            p.is_tackle = true;
                        }
                    } else if team_tackle == 'A' {
                        if let Some(p) = self.away_team.players.iter_mut().find(|p| p.number == number_tackle as usize) {
                            p.is_tackle = true;
                        }
                    }

                    let tackler = part.get(6)
                    .and_then(|s| Some(s.to_string()))
                    .unwrap_or("".to_string());
                    let team_tackler = tackler.chars().next().unwrap();
                    let number_tackler = tackler[1..].parse().unwrap_or(0);
                    if team_tackler == 'H' {
                        if let Some(p) = self.home_team.players.iter_mut().find(|p| p.number == number_tackler as usize) {
                            p.is_tackler = true;
                        }
                    } else if team_tackler == 'A' {
                        if let Some(p) = self.away_team.players.iter_mut().find(|p| p.number == number_tackler as usize) {
                            p.is_tackler = true;
                        }
                    }
                }

            } else if info.starts_with("B ") {
                let part: Vec<&str> = info.split(' ').collect();
                self.ball.x = part.get(1)
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(self.ball.x);
                self.ball.y = part.get(2)
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(self.ball.y);
                self.ball.is_carried = false;
                self.ball.z = 0.0;
            } else {
                let i: String;
                let ball: String;
                if info.contains("/B") {
                    let tmp: Vec<&str>  = info.split("/B").collect();
                    i = tmp.get(0)
                    .and_then(|s| Some(s.to_string()))
                    .unwrap_or("H0 0 0".to_string());
                    ball = tmp.get(1)
                    .and_then(|s| Some(s.to_string()))
                    .unwrap_or("0 0".to_string());
                    let tmp_ball: Vec<&str> = ball.trim().split(' ').collect();
                    self.ball.x = tmp_ball.get(0)
                        .and_then(|s| s.trim().parse().ok())
                        .unwrap_or(self.ball.x);
                    self.ball.y = tmp_ball.get(1)
                    .and_then(|s| s.trim().parse().ok())
                    .unwrap_or(self.ball.x);
                    self.ball.is_carried = true;
                    self.ball.z = 1.0;
                } else  {
                    i = info.to_string();
                    ball = "".to_string();
                }
                let part: Vec<&str> = i.split(' ').collect();
                let player: String = part.get(0)
                .and_then(|s| Some(s.to_string()))
                .unwrap_or("H0".to_string());
                let x: f32 = part.get(1)
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(0.0);
                let y: f32 = part.get(2)
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(0.0);
                let team: char = player.chars().next().unwrap();
                let number: i32 = player[1..].parse().unwrap_or(0);
                if team == 'H' {
                    if let Some(p) = self.home_team.players.iter_mut().find(|p| p.number == number as usize) {
                        p.x = x;
                        p.y = y;
                        if ball != "" {
                            p.ball_pos = true;
                        }
                    }
                } else if team == 'A' {
                    if let Some(p) = self.away_team.players.iter_mut().find(|p| p.number == number as usize) {
                        p.x = x;
                        p.y = y;
                        if ball != "" {
                            p.ball_pos = true;
                        }
                    }
                } else {
                    print!("Unknown team in state initialization: {}\n", team);
                }
            }
        }

        print!("Initialized game state: {}\n", self.state.name);
        print!("Ball position: {} {} {}\n", self.ball.x, self.ball.y, self.ball.z);
    }
}