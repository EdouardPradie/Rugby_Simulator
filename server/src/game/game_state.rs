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
    pub pound: f32,
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
    pub z: f32,
    pub is_carried: bool, // true if the ball is being carried by a player
}

#[derive(Clone, Copy)]
pub struct BallThrow {
    pub vx: f32,
    pub vy: f32,
    pub vz: f32,
    pub active: bool,
}

#[derive(Clone)]
pub struct State {
    pub name: String,
    pub team: char,
    pub x: f32,
    pub y: f32,
}

#[derive(Clone)]
pub struct GameState {
    pub state: State,
    pub field: Field,
    pub time: u64,
    pub home_players: Vec<Player>,
    pub home_bench: Vec<Player>,
    pub away_players: Vec<Player>,
    pub away_bench: Vec<Player>,
    pub ball: Ball,
    pub ball_throw: BallThrow,
}

const GRAVITY: f32 = 9.81;
const PASS_HEIGHT: f32 = 1.0;
const PASS_DIST: f32 = 20.0;
const DT: f32 = 0.25; // Time step in seconds
const RUNNING_SPEED_FACTOR: f32 = 1000.0 / 3600.0 * DT; // Conversion factor for speed to position change km/h -> m/(reaction time)s
const WALK_SPEED: f32 = 6.4;

impl GameState {
    pub fn new() -> Self {
        let state = State {
            name: String::from("start"),
            team: 'H',
            x: 0.0,
            y: 0.0,
        };
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
        let ball: Ball = Ball { x: 50.0, y: 35.0, z: 1.0, is_carried: false };
        let ball_throw = BallThrow { vx: 0.0, vy: 0.0, vz: 0.0, active: false };

        Self { state, field, time, home_players, home_bench, away_players, away_bench, ball, ball_throw }
    }

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
            let p_scrape: f32 = info.get(6)
                .and_then(|s| s.split('=').nth(1))
                .and_then(|v| v.parse().ok())
                .unwrap_or(10.0);
            if i >= 15 {
                self.home_bench.push(Player { x, y, number: (i + 1), ball_pos: false, size, pound, speed, foot, p_foot, p_tackle, p_scrape});
            } else {
                self.home_players.push(Player { x, y, number: (i + 1), ball_pos: false, size, pound, speed, foot, p_foot, p_tackle, p_scrape});
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
            let p_tackle: f32 = info.get(4)
                .and_then(|s| s.split('=').nth(1))
                .and_then(|v| v.parse().ok())
                .unwrap_or(10.0);
            let p_scrape: f32 = info.get(4)
                .and_then(|s| s.split('=').nth(1))
                .and_then(|v| v.parse().ok())
                .unwrap_or(10.0);
            if i >= 15 {
                self.away_bench.push(Player { x, y, number: (i + 1), ball_pos: false, size, pound, speed, foot, p_foot, p_tackle, p_scrape});
            } else {
                self.away_players.push(Player { x, y, number: (i + 1), ball_pos: false, size, pound, speed, foot, p_foot, p_tackle, p_scrape});
            }
        }

        // Initialize ball
        self.ball.x = self.home_players[9].x + 1.0;
        self.ball.y = self.home_players[9].y;

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
                    if let Some(p) = self.home_players.iter_mut().find(|p| p.number == number as usize) {
                        p.x = x;
                        p.y = y;
                        if ball != "" {
                            p.ball_pos = true;
                        }
                    }
                } else if team == 'A' {
                    if let Some(p) = self.away_players.iter_mut().find(|p| p.number == number as usize) {
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
        if self.state.name == "scrum" {
            drawable.set_state(self.state.x, self.state.y, self.state.name.clone(), 4.2);
        }
        drawable.set_time(self.time);
        return drawable;
    }

    pub fn play(&mut self, input: String) {
        self.time += 25;
        if self.state.name == "start" {
            self.state.name = "play".to_string();
        }

        // Add here the logic to process the input actions
        let actions: Vec<&str> = input.split('\n').collect();
        for action in actions {
            if let Some((player, action)) = action.split_once(':') {
                let tmp = action.chars().nth(0).unwrap_or('\0');
                match tmp {
                    'R' => {
                        let team = player.chars().next().unwrap();
                        let number = player[1..].parse().unwrap_or(0);
                        let direction = action[1..].parse().unwrap_or(0.0);
                        // println!("Player {} {} runs {}", team, number, direction);
                        self.run(team, number, direction, true);
                    },
                    'W' => {
                        let team = player.chars().next().unwrap();
                        let number = player[1..].parse().unwrap_or(0);
                        let direction = action[1..].parse().unwrap_or(0.0);
                        // println!("Player {} {} walk {}", team, number, direction);
                        self.run(team, number, direction, false);
                    },
                    'K' => {
                        let team = player.chars().next().unwrap();
                        let number = player[1..].parse().unwrap_or(0);
                        let tmp: Vec<&str> = action[1..].split('/').collect();
                        let direction = tmp[0].parse().unwrap_or(0.0);
                        let high = tmp[1].parse().unwrap_or(0.0);
                        println!("Player {} {} Kick in way {} at {}", team, number, direction, high);
                        self.kick(team, number, direction, high);
                    },
                    'P' => {
                        let team = player.chars().next().unwrap();
                        let number = player[1..].parse().unwrap_or(0);
                        let tmp: Vec<&str> = action[1..].split('/').collect();
                        let direction = tmp[0].parse().unwrap_or(0.0);
                        println!("Player {} {} Pass in way {}", team, number, direction);
                        self.pass(team, number, direction);
                    },
                    'S' => continue,
                    _ => {
                        print!("Unknown action: {}\n", action);
                    },
                }
            }
        }

        // Update ball position if it's being thrown
        self.update_ball_velocity();
        self.update_ball_carrie();

        // Apply rules of rugby (incomming)

    }

    fn update_ball_velocity(&mut self) {
        if self.ball_throw.active {
            self.ball.x += self.ball_throw.vx * DT;
            self.ball.y += self.ball_throw.vy * DT;
            self.ball.z += self.ball_throw.vz * DT;
            self.ball_throw.vz -= GRAVITY * DT; // gravity effect

            self.state.x = self.ball.x;
            self.state.y = self.ball.y;

            if self.ball.z <= 0.0 {
                self.ball.z = 0.0;
                self.ball_throw.active = false;
            }
        }
    }

    fn update_ball_carrie(&mut self) {
        // Check if any player can pick up the ball (under 4 meters height)
        if !self.ball.is_carried && self.ball.z <= 4.0 {
            for (team, player) in
            self.home_players.iter_mut().map(|p| ('H', p))
            .chain(self.away_players.iter_mut().map(|p| ('A', p))) {
                let distance = ((player.x - self.ball.x).powi(2) + (player.y - self.ball.y).powi(2)).sqrt();
                if distance < 1.0 && self.ball.z <= player.size + 50.0 { // 50 cm player arm
                    let is_successful = rand::random::<f32>() * 100.0 > self.field.weather as f32;
                    if !is_successful {
                        print!("Player {} failed to pick up the ball due to weather\n", player.number);
                        continue;
                    }
                    player.ball_pos = true;
                    self.ball.is_carried = true;
                    self.ball_throw.active = false;
                    self.state.x = self.ball.x;

                    match team {
                        'H' => {
                            print!("Home player {} picked up the ball\n", player.number);
                            self.ball.x = player.x + 1.0;
                            self.state.x = self.ball.x;
                        },
                        'A' => {
                            print!("Away player {} picked up the ball\n", player.number);
                            self.ball.x = player.x - 1.0;
                            self.state.x = self.ball.x;
                        },
                        _ => {}
                    }
                    self.state.team = team;
                    self.ball.y = player.y;
                    self.state.y = self.ball.y;
                    self.ball.z = 1.0;
                    break;
                }
            }
        }
    }

    fn run(&mut self, team: char, number: i32, direction: f32, is_running: bool) {
        let players = if team == 'H' {
            &mut self.home_players
        } else {
            &mut self.away_players
        };

        if let Some(p) = players.iter_mut().find(|p| p.number == number as usize) {
            let speed = if is_running { p.speed } else { WALK_SPEED };
            let rad = direction.to_radians();
            p.x += rad.cos() * (speed * RUNNING_SPEED_FACTOR);
            p.y += rad.sin() * (speed * RUNNING_SPEED_FACTOR);
            if p.ball_pos {
                self.ball.x += rad.cos() * (speed * RUNNING_SPEED_FACTOR);
                self.ball.y += rad.sin() * (speed * RUNNING_SPEED_FACTOR);
            }
        }
    }

    fn kick(&mut self, team: char, number: i32, direction: f32, high: f32) {
        let tmp_high = high.clamp(0.0, 90.0);
        let mut dir = direction.to_radians();
        let mut elev = tmp_high.to_radians();

        let players = if team == 'H' {
            &mut self.home_players
        } else {
            &mut self.away_players
        };

        if let Some(p) = players.iter_mut().find(|p| p.number == number as usize) {
            if p.ball_pos {
                p.ball_pos = false;
                self.ball.is_carried = false;
                self.ball.z = 0.0;

                let mut speed = (p.foot * GRAVITY).sqrt() / 1.12; // Empirical correction
                let is_successful = rand::random::<f32>() * 100.0 < p.p_foot;

                if !is_successful {
                    print!("Failed kick\n");
                    let horizontal_dev_deg = (rand::random::<f32>() * 20.0) - 10.0; // -10° to +10°
                    dir += horizontal_dev_deg.to_radians();

                    let vertical_dev_deg = (rand::random::<f32>() * 10.0) - 5.0; // -5° to +5°
                    elev += vertical_dev_deg.to_radians();

                    let loss = 0.80 + rand::random::<f32>() * 0.2; // between 80% and 100%
                    speed *= loss;
                } else {
                    print!("Success kick\n");
                }

                let vx = speed * elev.cos() * dir.cos();
                let vy = speed * elev.cos() * dir.sin();
                let vz = speed * elev.sin();

                self.ball_throw = BallThrow {
                    vx,
                    vy,
                    vz,
                    active: true,
                };
            }
        }
    }

    fn pass(&mut self, team: char, number: i32, direction: f32) {
        let direction_rad = direction.to_radians();
        let high_rad = (PASS_HEIGHT as f32).to_radians();

        let players = if team == 'H' {
            &mut self.home_players
        } else {
            &mut self.away_players
        };

        if let Some(p) = players.iter_mut().find(|p| p.number == number as usize) {
            if p.ball_pos {
                p.ball_pos = false;
                self.ball.is_carried = false;

                let speed = (PASS_DIST * GRAVITY).sqrt() / 1.12; // Empirical correction

                let vx = speed * high_rad.cos() * direction_rad.cos();
                let vy = speed * high_rad.cos() * direction_rad.sin();
                let vz = speed * high_rad.sin();

                self.ball_throw = BallThrow {
                    vx,
                    vy,
                    vz,
                    active: true,
                };
            }
        }
    }
}
