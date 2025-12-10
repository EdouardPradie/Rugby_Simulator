pub struct Position {
    pub x: f32,
    pub y: f32,
}
pub struct DrawPlayer {
    pub pos: Position,
    pub number: usize,
}

pub struct DrawState {
    pub pos: Position,
    pub name: String,
    pub size: f32,
}
pub struct Drawable {
    pub ball: Position,
    pub state: DrawState,
    pub home_players: Vec<DrawPlayer>,
    pub away_players: Vec<DrawPlayer>,
    pub time: u64,
}

impl Drawable {
    pub fn new(x: f32, y: f32) -> Self {
        Drawable {
            ball: Position { x, y },
            state: DrawState {
                pos: Position { x: 0.0, y: 0.0 },
                name: String::new(),
                size: 0.0,
            },
            home_players: Vec::new(),
            away_players: Vec::new(),
            time: 0,
        }
    }

    pub fn add_home_player(&mut self, x: f32, y: f32, number: usize) {
        self.home_players.push(DrawPlayer {
            pos: Position { x, y },
            number,
        });
    }

    pub fn add_away_player(&mut self, x: f32, y: f32, number: usize) {
        self.away_players.push(DrawPlayer {
            pos: Position { x, y },
            number,
        });
    }

    pub fn set_state(&mut self, x: f32, y: f32, name: String, size: f32) {
        self.state = DrawState {
            pos: Position { x, y },
            name,
            size,
        };
    }

    pub fn set_time(&mut self, time: u64) {
        self.time = time;
    }
}