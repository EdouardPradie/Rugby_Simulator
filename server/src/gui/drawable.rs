pub struct Position {
    pub x: f32,
    pub y: f32,
}
pub struct DrawPlayer {
    pub pos: Position,
    pub number: usize,
}
pub struct Drawable {
    pub ball: Position,
    pub home_players: Vec<DrawPlayer>,
    pub away_players: Vec<DrawPlayer>,
    pub time: u64,
}

impl Drawable {
    pub fn new(x: f32, y: f32) -> Self {
        Drawable {
            ball: Position { x, y },
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

    pub fn set_time(&mut self, time: u64) {
        self.time = time;
    }
}