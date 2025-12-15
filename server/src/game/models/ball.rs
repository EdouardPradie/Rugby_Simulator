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