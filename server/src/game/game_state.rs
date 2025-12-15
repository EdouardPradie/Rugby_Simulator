use crate::gui::drawable::Drawable;

#[derive(Clone)]
pub struct Field {
    pub width: usize,
    pub height: usize,
    pub try_size: usize,
    pub home_direction_try: char,
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
    pub is_tackle: bool,
    pub is_tackler: bool,
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
    pub size: f32,
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
const SCRUM_SIZE: f32 = 4.2;
const DT: f32 = 0.25; // Time step in seconds
const RUNNING_SPEED_FACTOR: f32 = 1000.0 / 3600.0 * DT; // Conversion factor for speed to position change km/h -> m/(reaction time)s
const WALK_SPEED: f32 = 6.4;

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
        let home_players = Vec::new();
        let home_bench = Vec::new();
        let away_players = Vec::new();
        let away_bench = Vec::new();
        let ball: Ball = Ball { x: 50.0, y: 35.0, z: 1.0, is_carried: false };
        let ball_throw = BallThrow { vx: 0.0, vy: 0.0, vz: 0.0, active: false };

        Self { state, field, time, home_players, home_bench, away_players, away_bench, ball, ball_throw }
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
                self.home_bench.push(Player { x, y, number: (i + 1), ball_pos: false, size, pound, speed, foot, p_foot, p_tackle, is_tackle: false, is_tackler: false });
            } else {
                self.home_players.push(Player { x, y, number: (i + 1), ball_pos: false, size, pound, speed, foot, p_foot, p_tackle, is_tackle:false, is_tackler: false });
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
            let p_tackle: f32 = info.get(5)
                .and_then(|s| s.split('=').nth(1))
                .and_then(|v| v.parse().ok())
                .unwrap_or(10.0);
            if i >= 15 {
                self.away_bench.push(Player { x, y, number: (i + 1), ball_pos: false, size, pound, speed, foot, p_foot, p_tackle, is_tackle:false, is_tackler: false });
            } else {
                self.away_players.push(Player { x, y, number: (i + 1), ball_pos: false, size, pound, speed, foot, p_foot, p_tackle, is_tackle:false, is_tackler: false });
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
                        if let Some(p) = self.home_players.iter_mut().find(|p| p.number == number_tackle as usize) {
                            p.is_tackle = true;
                        }
                    } else if team_tackle == 'A' {
                        if let Some(p) = self.away_players.iter_mut().find(|p| p.number == number_tackle as usize) {
                            p.is_tackle = true;
                        }
                    }

                    let tackler = part.get(6)
                    .and_then(|s| Some(s.to_string()))
                    .unwrap_or("".to_string());
                    let team_tackler = tackler.chars().next().unwrap();
                    let number_tackler = tackler[1..].parse().unwrap_or(0);
                    if team_tackler == 'H' {
                        if let Some(p) = self.home_players.iter_mut().find(|p| p.number == number_tackler as usize) {
                            p.is_tackler = true;
                        }
                    } else if team_tackler == 'A' {
                        if let Some(p) = self.away_players.iter_mut().find(|p| p.number == number_tackler as usize) {
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

    //EXTRACT

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
            drawable.set_state(self.state.x, self.state.y, self.state.name.clone(), self.state.size);
        }
        if self.state.name == "ruck" {
            drawable.set_state(self.state.x, self.state.y, self.state.name.clone(), self.state.size);
        }
        drawable.set_time(self.time);
        return drawable;
    }

    //PLAY

    pub fn play(&mut self, input: String) {
        self.time += 25;
        if self.state.name == "start" {
            self.state.name = "play".to_string();
        }

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
                    'T' => {
                        let team = player.chars().next().unwrap();
                        let number = player[1..].parse().unwrap_or(0);
                        self.tackle(team, number);
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
        if !self.ball.is_carried && self.ball.z <= 3.5 {
            for (team, player) in
            self.home_players.iter_mut().map(|p| ('H', p))
            .chain(self.away_players.iter_mut().map(|p| ('A', p))) {
                let distance = ((player.x - self.ball.x).powi(2) + (player.y - self.ball.y).powi(2)).sqrt();
                if distance < 1.0 && self.ball.z <= player.size + 50.0 { // 50 cm player arm
                    let is_successful = rand::random::<f32>() * 100.0 > (self.field.weather / 2) as f32;
                    if !is_successful {
                        print!("Player {} failed to pick up the ball due to weather\n", player.number);
                        continue;
                    }
                    player.ball_pos = true;
                    self.ball.is_carried = true;
                    self.ball_throw.active = false;

                    match team {
                        'H' => {
                            print!("Home player {} picked up the ball\n", player.number);
                            self.ball.x = player.x + if self.field.home_direction_try == 'N' { 0.5 } else { -0.5 };
                            self.state.x = self.ball.x;
                        },
                        'A' => {
                            print!("Away player {} picked up the ball\n", player.number);
                            self.ball.x = player.x + if self.field.home_direction_try == 'N' { -0.5 } else { 0.5 };
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

    // ACTIONS

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

    fn tackle(&mut self, team: char, number: i32) {
        if !self.ball.is_carried {
            print!("No player has the ball to tackle\n");
            return;
        }
        let (players, opponents) = if team == 'H' {
            (&mut self.home_players, &mut self.away_players)
        } else {
            (&mut self.away_players, &mut self.home_players)
        };

        if let Some(p) = players.iter_mut().find(|p| p.ball_pos) {
            print!("No player in the other team have the ball because {} have the ball\n", p.number);
            return;
        }
        if let Some(p) = players.iter_mut().find(|p| p.number == number as usize) {
            if let Some(o) = opponents.iter_mut().find(|p| p.ball_pos) {
                let distance = ((p.x - o.x).powi(2) + (p.y - o.y).powi(2)).sqrt();
                if distance < 1.2 {
                    let is_successful = rand::random::<f32>() * 100.0 < p.p_tackle;
                    if is_successful {
                        p.is_tackle = true;
                        print!("Tackle successful by player {} {}\n", team, number);
                        let ruck_team = if team == 'H' { 'A' } else { 'H' };
                        let ruck_x = o.x;
                        let ruck_y = o.y;
                        o.ball_pos = false;
                        self.set_ruck(ruck_x, ruck_y, ruck_team);
                    } else {
                        print!("Tackle failed by player {} {}\n", team, number);
                    }
                } else {
                    print!("Player {} {} is too far to tackle\n", team, number);
                }
            }
        }
    }

    // SCRUM

    pub fn scrum(&mut self, input: String) {
        self.time += 25;
        let mut scrum_h_pound = 0.0;
        let mut scrum_a_pound = 0.0;

        if self.state.size != SCRUM_SIZE {
            self.state.size = SCRUM_SIZE;
        }

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
                        let players = if team == 'H' {
                            &mut self.home_players
                        } else {
                            &mut self.away_players
                        };
                        let player_index = players.iter()
                        .position(|p| p.number == number as usize)
                        .unwrap_or(15);
                        if player_index >= 7 && player_index < 15 {
                            self.run(team, number, direction, true);
                        } else  {
                            print!("Player {} {} cannot run during scrum\n", team, number);
                        }
                    },
                    'W' => {
                        let team = player.chars().next().unwrap();
                        let number = player[1..].parse().unwrap_or(0);
                        let direction = action[1..].parse().unwrap_or(0.0);
                        // println!("Player {} {} walk {}", team, number, direction);
                        let players = if team == 'H' {
                            &mut self.home_players
                        } else {
                            &mut self.away_players
                        };
                        let player_index = players.iter()
                        .position(|p| p.number == number as usize)
                        .unwrap_or(15);
                        if player_index >= 7 && player_index < 15 {
                            self.run(team, number, direction, false);
                        } else  {
                            print!("Player {} {} cannot walk during scrum\n", team, number);
                        }
                    },
                    'T' => {
                        let team = player.chars().next().unwrap();
                        let number = player[1..].parse().unwrap_or(0);
                        let players = if team == 'H' {
                            &mut self.home_players
                        } else {
                            &mut self.away_players
                        };
                        let player_index = players.iter()
                        .position(|p| p.number == number as usize)
                        .unwrap_or(15);
                        if player_index >= 9 && player_index < 15 {
                            print!("Player {} {} is not in scrum\n", team, number);
                        } else if player_index <= 6 {
                            print!("Player {} {} add to push in scrum\n", team, number);
                        } else if player_index == 7 || player_index == 8 {
                            self.try_catch_ball_in_scrum(team, number);
                        } else {
                            continue;
                        }
                    },
                    'S' => {
                        let team = player.chars().next().unwrap();
                        let number = player[1..].parse().unwrap_or(0);
                        // println!("Player {} {} stop or scrum", team, number);
                        let players = if team == 'H' {
                            &mut self.home_players
                        } else {
                            &mut self.away_players
                        };
                        let player_index = players.iter()
                        .position(|p| p.number == number as usize)
                        .unwrap_or(15);
                        if player_index <= 7 {
                            if team == 'H' {
                                scrum_h_pound += players[player_index].pound;
                            } else {
                                scrum_a_pound += players[player_index].pound;
                            }
                        } else  {
                            continue;
                        }
                    },
                    _ => {
                        print!("Unknown action in scrum: {}\n", action);
                    },
                }
            }
        }
        self.update_ball_position_scrum(scrum_h_pound, scrum_a_pound);
        self.check_ball_out_of_scrum();
    }

    fn update_ball_position_scrum(&mut self, scrum_h_pound: f32, scrum_a_pound: f32) {
        let scrum_in = if self.state.team == 'H' { scrum_h_pound } else { scrum_a_pound };
        let scrum_front = if self.state.team == 'H' { scrum_a_pound } else { scrum_h_pound };
        let mut direction = if self.state.team == 'H' {
            if self.field.home_direction_try == 'N' { -0.5 } else { 0.5 }
        } else {
            if self.field.home_direction_try == 'N' { 0.5 } else { -0.5 }
        };
        if scrum_front > scrum_in {
            let diff_ratio = (scrum_front - scrum_in) / (scrum_front + scrum_in) * 20.0;
            let reverse_probability = diff_ratio.clamp(0.0, 1.0);
            if rand::random::<f32>() < reverse_probability {
                direction = -direction;
                print!("Scrum contest\n");
            }
        }
        self.ball.x += direction;
    }

    fn check_ball_out_of_scrum(&mut self) {
        let distance = ((self.ball.x - self.state.x).powi(2) + (self.ball.y - self.state.y).powi(2)).sqrt();
        if !self.ball.is_carried && distance >= SCRUM_SIZE {
            self.state.name = "play".to_string();
            self.state.x = self.ball.x;
            self.state.y = self.ball.y;
            print!("Ball out of scrum, resuming play\n");
        }
    }

    fn try_catch_ball_in_scrum(&mut self, team: char, number: i32) {

        let players = if team == 'H' {
            &mut self.home_players
        } else {
            &mut self.away_players
        };

        if let Some(p) = players.iter_mut().find(|p| p.number == number as usize) {
            let distance = ((p.x - self.ball.x).powi(2) + (p.y - self.ball.y).powi(2)).sqrt();

            if distance < 1.0 {
                p.ball_pos = true;
                self.ball.is_carried = true;
                self.ball_throw.active = false;

                match team {
                    'H' => {
                        print!("Home player {} picked up the ball in scrum\n", p.number);
                        self.ball.x = p.x + if self.field.home_direction_try == 'N' { 0.5 } else { -0.5 };
                        self.state.x = self.ball.x;
                    },
                    'A' => {
                        print!("Away player {} picked up the ball in scrum\n", p.number);
                        self.ball.x = p.x + if self.field.home_direction_try == 'N' { -0.5 } else { 0.5 };
                        self.state.x = self.ball.x;
                    },
                    _ => {}
                }
                self.state.name = "play".to_string();
                self.state.team = team;
                self.ball.y = p.y;
                self.state.y = self.ball.y;
                self.ball.z = 1.0;
            }
        }
    }

    // RUCK

    fn set_ruck(&mut self, x: f32, y: f32, team: char) {
        self.ball.is_carried = false;
        self.ball.x = x - 0.5;
        self.ball.y = y;
        self.ball.z = 0.0;
        self.state.name = "ruck".to_string();
        self.state.x = x;
        self.state.y = y;
        self.state.team = team;
        print!("Ruck formed at position {} {}\n", x, y);
    }

    pub fn ruck(&mut self, input: String) {
        self.time += 25;

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
                        self.run_ruck(team, number, direction, true);
                    },
                    'W' => {
                        let team = player.chars().next().unwrap();
                        let number = player[1..].parse().unwrap_or(0);
                        let direction = action[1..].parse().unwrap_or(0.0);
                        // println!("Player {} {} walk {}", team, number, direction);
                        self.run_ruck(team, number, direction, false);
                    },
                    'T' => {
                        let team = player.chars().next().unwrap();
                        let number = player[1..].parse().unwrap_or(0);
                        self.try_catch_ball_in_ruck(team, number);
                    },
                    'S' => continue,
                    _ => {
                        print!("Unknown action in ruck: {}\n", action);
                    },
                }
            }
        }

        self.check_tackler();
        self.check_scrap();
    }

    //ACTIONS IN RUCK

    fn run_ruck(&mut self, team: char, number: i32, direction: f32, is_running: bool) {
        let players = if team == 'H' {
            &mut self.home_players
        } else {
            &mut self.away_players
        };

        if let Some(p) = players.iter_mut().find(|p| p.number == number as usize) {
            if p.is_tackle {
                print!("Player {} {} is tackled and cannot move in ruck\n", team, number);
                return;
            }
            let speed = if is_running { p.speed } else { WALK_SPEED };
            let rad = direction.to_radians();
            p.x += rad.cos() * (speed * RUNNING_SPEED_FACTOR);
            p.y += rad.sin() * (speed * RUNNING_SPEED_FACTOR);
            if p.ball_pos {
                self.ball.x += rad.cos() * (speed * RUNNING_SPEED_FACTOR);
                self.ball.y += rad.sin() * (speed * RUNNING_SPEED_FACTOR);
            }
            let distance = ((p.x - self.state.x).powi(2) + (p.y - self.state.y).powi(2)).sqrt();
            if distance < self.state.size {
                if team == 'H' {
                    if (self.field.home_direction_try == 'S' && direction >= 135.0 && direction <= 225.0) ||
                    (self.field.home_direction_try == 'N' && (direction >= 315.0 || direction <= 45.0)) {
                        print!("Home player {} go in ruck\n", number);
                    } else {
                        print!("Penalty for {}\n", "A");
                    }
                }
                if team == 'A' {
                    if (self.field.home_direction_try == 'N' && direction >= 135.0 && direction <= 225.0) ||
                    (self.field.home_direction_try == 'S' && (direction >= 315.0 || direction <= 45.0)) {
                        print!("Away player {} go in ruck\n", number);
                    } else {
                        print!("Penalty for {}\n", "H");
                    }
                }
            }
        }
    }

    fn check_tackler(&mut self) {
        for (team, player) in
        self.home_players.iter_mut().map(|p| ('H', p))
        .chain(self.away_players.iter_mut().map(|p| ('A', p))) {
            if player.is_tackler {
                let distance = ((player.x - self.state.x).powi(2) + (player.y - self.state.y).powi(2)).sqrt();
                if distance < 1.0 {
                    print!("Tackler penalty: ball for {}\n", if team == 'H' { 'A' } else { 'H' });
                } else {
                    player.is_tackler = false;
                }
            }
        }
    }

    fn check_scrap(&mut self) {
        let mut scrap_h_pound = 0.0;
        let mut scrap_a_pound = 0.0;
        let mut contest_player: Option<&mut Player> = None;
        let mut contest_dist: f32 = f32::MAX;

        for (team, player) in
        self.home_players.iter_mut().map(|p| ('H', p))
        .chain(self.away_players.iter_mut().map(|p| ('A', p))) {
            let distance = ((player.x - self.state.x).powi(2) + (player.y - self.state.y).powi(2)).sqrt();
            if distance < self.state.size && !player.is_tackle {
                if team == 'H' {
                    scrap_h_pound += player.pound;
                    if self.state.team == 'A' && distance < contest_dist {
                        contest_dist = distance;
                        contest_player = Some(player);
                    }
                } else {
                    scrap_a_pound += player.pound;
                    if self.state.team == 'H' && distance < contest_dist {
                        contest_dist = distance;
                        contest_player = Some(player);
                    }
                }
            }
        }

        let scrap_in = if self.state.team == 'H' { scrap_h_pound } else { scrap_a_pound };
        let scrap_front = if self.state.team == 'H' { scrap_a_pound } else { scrap_h_pound };

        if scrap_front > scrap_in {
            if let Some(player) = contest_player {
                //update ball
                player.ball_pos = true;
                self.ball.is_carried = true;
                self.ball.z = 1.0;
                if self.state.team == 'H' {
                    print!("Away player {} picked up the ball from ruck\n", player.number);
                    self.ball.x = player.x + if self.field.home_direction_try == 'N' { -0.5 } else { 0.5 };
                } else {
                    print!("Home player {} picked up the ball from ruck\n", player.number);
                    self.ball.x = player.x + if self.field.home_direction_try == 'N' { 0.5 } else { -0.5 };
                }
                self.ball.y = player.y;

                //update state
                self.state.name = "play".to_string();
                self.state.team = if self.state.team == 'H' { 'A' } else { 'H' };
                self.state.x = self.ball.x;
                self.state.y = self.ball.y;
            }
        }
    }

    fn try_catch_ball_in_ruck(&mut self, team: char, number: i32) {
        let is_offside = self.check_offside();
        let players = if team == 'H' {
            &mut self.home_players
        } else {
            &mut self.away_players
        };

        if let Some(p) = players.iter_mut().find(|p| p.number == number as usize) {
            let distance_ball = ((p.x - self.ball.x).powi(2) + (p.y - self.ball.y).powi(2)).sqrt();
            let distance_ruck = ((p.x - self.state.x).powi(2) + (p.y - self.state.y).powi(2)).sqrt();

            if distance_ball < 1.0 && distance_ruck >= 1.0 {
                if is_offside {
                    print!("Offside penalty for {}\n", self.state.team);
                    return;
                }
                p.ball_pos = true;
                self.ball.is_carried = true;
                self.ball_throw.active = false;

                match team {
                    'H' => {
                        print!("Home player {} picked up the ball in scrum\n", p.number);
                        self.ball.x = p.x + if self.field.home_direction_try == 'N' { 0.5 } else { -0.5 };
                        self.state.x = self.ball.x;
                    },
                    'A' => {
                        print!("Away player {} picked up the ball in scrum\n", p.number);
                        self.ball.x = p.x + if self.field.home_direction_try == 'N' { -0.5 } else { 0.5 };
                        self.state.x = self.ball.x;
                    },
                    _ => {}
                }
                self.state.name = "play".to_string();
                self.state.team = team;
                self.ball.y = p.y;
                self.state.y = self.ball.y;
                self.ball.z = 1.0;
            }
        }
    }

    fn check_offside(&self) -> bool {
        let players = if self.state.team == 'H' {
            &self.away_players
        } else {
            &self.home_players
        };
        let diff = if (self.state.team == 'H' && self.field.home_direction_try == 'N') ||
                            (self.state.team == 'A' && self.field.home_direction_try == 'S') {
            1.0
        } else {
            -1.0
        };

        for player in players.iter() {
            let distance = ((player.x - self.state.x).powi(2) + (player.y - self.state.y).powi(2)).sqrt();
            if distance >= self.state.size &&
            ((diff == 1.0 && player.x < self.state.x + diff) ||
            (diff == -1.0 && player.x > self.state.x + diff)) {
                print!("Player {} is offside\n", player.number);
                return true;
            }
        }
        return false;
    }
}
