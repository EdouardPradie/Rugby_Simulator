#[derive(Clone, Copy)]
pub struct Player {
    pub x: f32,
    pub y: f32,
    pub number: usize,
    pub ball_pos: bool,
    pub size: f32,
    pub pound: f32,
    pub speed: f32,
    pub foot: f32,
    pub p_foot: f32,
    pub p_tackle: f32,
    pub is_tackle: bool,
    pub is_tackler: bool,
}