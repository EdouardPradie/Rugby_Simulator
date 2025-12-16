use crate::game::game_state::GameState;
use crate::game::constants::*;

impl GameState {
    pub fn setup_scrum(&mut self, team: char, x: f32, y: f32) {
        self.state.name = "scrum".to_string();
        self.state.team = team;
        self.state.x = x;
        self.state.y = y;
        self.state.size = SCRUM_SIZE;

        self.ball.is_carried = false;
        if (self.field.home_direction_try == 'N' && team == 'H') ||
           (self.field.home_direction_try == 'S' && team == 'A') {
            self.ball.x = x - 0.5;
        } else {
            self.ball.x = x + 0.5;
        }
        self.ball.y = y;
        self.ball.z = 0.0;

        let (north_team, south_team) = if self.field.home_direction_try == 'N' {
            (&mut self.home_players, &mut self.away_players)
        } else {
            (&mut self.away_players, &mut self.home_players)
        };

        let scrum_offsets = |dir: f32| -> Vec<(usize, f32, f32)> {
            vec![
                // Front row
                (1, dir * 0.5, -1.0),
                (2, dir * 0.5,  0.0),
                (3, dir * 0.5,  1.0),

                // Second row
                (4, dir * 1.5, -0.5),
                (5, dir * 1.5,  0.5),

                // Back row
                (6, dir * 1.5, -1.5),
                (7, dir * 1.5,  1.5),
                (8, dir * 2.5,  0.0),

                // Scrum-half
                (9, dir * 0.5, 2.5),

                // Wingers
                (11, dir * 17.0, -1.0),
                (14, dir * 17.0,  1.0),
            ]
        };

        for (num, dx, dy) in scrum_offsets(-1.0) {
            if let Some(p) = north_team.iter_mut().find(|p| p.number == num) {
                p.x = self.state.x + dx;
                if num == 9 && self.state.y > (self.field.height / 2) as f32 {
                    p.y = self.state.y - dy;
                } else {
                    p.y = self.state.y + dy;
                }
                if num == 11 || num == 14 {
                    if dy == -1.0 {
                        p.y = 10.0;
                    } else if dy == 1.0 {
                        p.y = self.field.height as f32 - 10.0;
                    }
                }
            }
        }

        for (num, dx, dy) in scrum_offsets(1.0) {
            if let Some(p) = south_team.iter_mut().find(|p| p.number == num) {
                p.x = self.state.x + dx;
                if num == 9 && self.state.y <= (self.field.height / 2) as f32 {
                    p.y = self.state.y + dy;
                } else {
                    p.y = self.state.y - dy;
                }
                if num == 11 || num == 14 {
                    if dy == 1.0 {
                        p.y = 10.0;
                    } else if dy == -1.0 {
                        p.y = self.field.height as f32 - 10.0;
                    }
                }
            }
        }

        let (x1, x2, attack_team, defense_team, defense_line) = if (self.field.home_direction_try == 'N' && team == 'H') ||
            (self.field.home_direction_try == 'S' && team == 'A') {
            (self.state.x - 5.0, self.state.x - 15.0, north_team, south_team, 5.0)
        } else {
            (self.state.x + 5.0, self.state.x + 15.0, south_team, north_team, -5.0)
        };

        let y1: f32 = self.state.y;
        let y2: f32 = if self.state.y < (self.field.height / 2) as f32 {
            self.field.height as f32 - 15.0
        } else {
            15.0
        };

        let mut pts = Vec::with_capacity(4);
        for i in 1..5 {
            let t = i as f32 / (5 as f32 - 1.0);
            let x = x1 + t * (x2 - x1);
            let y = y1 + t * (y2 - y1);
            pts.push((x, y));
        }

        let mut i = 0;
        for p in attack_team.iter_mut().filter(|p| p.number >= 10) {
            if p.number == 11 || p.number == 14 {
                continue;
            }
            p.x = pts.get(i).unwrap().0;
            p.y =  pts.get(i).unwrap().1;
            i += 1;
        }

        i = 0;
        for p in defense_team.iter_mut().filter(|p| p.number >= 10) {
            if p.number == 11 || p.number == 14 {
                continue;
            }
            p.x = self.state.x + defense_line;
            p.y =  pts.get(i).unwrap().1;
            i += 1;
        }

        print!("Setting up scrum for team {} at ({}, {})\n", self.state.team, self.state.x, self.state.y);
    }
}