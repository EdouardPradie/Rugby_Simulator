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