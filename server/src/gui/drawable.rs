pub struct Position {
    pub x: usize,
    pub y: usize,
}
pub struct DrawPlayer {
    pub pos: Position,
    pub number: usize,
}
pub struct Drawable {
    pub ball: Position,
    pub home_players: Vec<DrawPlayer>,
    pub away_players: Vec<DrawPlayer>,
}

impl Drawable {
    pub fn new(x: usize, y: usize) -> Self {
        Drawable {
            ball: Position { x, y },
            home_players: Vec::new(),
            away_players: Vec::new(),
        }
    }

    pub fn add_home_player(&mut self, x: usize, y: usize, number: usize) {
        self.home_players.push(DrawPlayer {
            pos: Position { x, y },
            number,
        });
    }

    pub fn add_away_player(&mut self, x: usize, y: usize, number: usize) {
        self.away_players.push(DrawPlayer {
            pos: Position { x, y },
            number,
        });
    }
}