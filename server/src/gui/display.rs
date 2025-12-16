use minifb::{Window, WindowOptions};
use crate::{game::constants::SCRUM_SIZE, gui::drawable::Drawable};

const GROUND_COLOR: u32 = 0xFF66D575;
const GROUND_LINE_COLOR: u32 = 0xFFCDF4D3;
const GROUND_OUT_COLOR: u32 = 0xFFD9D9D9;
const TEAM1: u32 = 0xFFA12222;
const TEAM2: u32 = 0xFF000000;
const WHITE: u32 = 0xFFFFFFFF;
const BLACK: u32 = 0xFF000000;
const SCRUM: u32 = 0xFFB195EE;
const SCRUM_LINE: u32 = 0xFF9267EE;
const RUCK: u32 = 0xFF00DFFF;
const RUCK_LINE: u32 = 0xFF20C2BD;
const OFFSIDE_LINE: u32 = 0xFFE63319;

const FONT_3X5: [[&str; 5]; 10] = [
    [ "111", "101", "101", "101", "111" ], // 0
    [ "010", "110", "010", "010", "111" ], // 1
    [ "111", "001", "111", "100", "111" ], // 2
    [ "111", "001", "111", "001", "111" ], // 3
    [ "101", "101", "111", "001", "001" ], // 4
    [ "111", "100", "111", "001", "111" ], // 5
    [ "111", "100", "111", "101", "111" ], // 6
    [ "111", "001", "001", "001", "001" ], // 7
    [ "111", "101", "111", "101", "111" ], // 8
    [ "111", "101", "111", "001", "111" ], // 9
];

pub struct Display {
    window: Window,
    buffer: Vec<u32>,
    width: usize,
    height: usize,
    size: usize,
    try_size: usize,
    time: u64,
    is_initialized: bool,
}

impl Display {
    pub fn new(width: usize, height: usize, try_size: usize) -> Self {
        let size: usize = width + 2 * try_size;
        let window = Window::new(
            "starting...",
            size,
            height,
            WindowOptions::default(),
        ).unwrap_or_else(|e| {
            panic!("Unable to open window: {}", e);
        });
        let buffer: Vec<u32> = vec![GROUND_COLOR; size * height];
        let time = 0;
        let is_initialized = false;

        Self { window, buffer, width, height, size, try_size, time, is_initialized }
    }

    pub fn initialize(&mut self, field: String, pixel_per_cell: usize) {
        // Parse the field dimensions from the field string
        let field_parts: Vec<&str> = field.split('_').collect();
        self.width = field_parts.get(0)
            .and_then(|s| s.split('=').nth(1))
            .and_then(|v| v.parse().ok())
            .unwrap_or(100) * pixel_per_cell;
        self.height = field_parts.get(1)
        .and_then(|s| s.split('=').nth(1))
        .and_then(|v| v.parse().ok())
        .unwrap_or(70) * pixel_per_cell;
        self.try_size = field_parts.get(2)
            .and_then(|s| s.split('=').nth(1))
            .and_then(|v| v.parse().ok())
            .unwrap_or(10) * pixel_per_cell;

        self.size = self.width + 2 * self.try_size;
        self.window = Window::new(
            "Rugby Simulator",
            self.size,
            self.height,
            WindowOptions::default(),
        ).unwrap_or_else(|e| {
            panic!("Unable to open window: {}", e);
        });
        self.is_initialized = true;
        self.buffer = vec![GROUND_COLOR; self.size * self.height];
    }

    pub fn render(&mut self, drawable: &Drawable, pixel_per_cell: usize) {
        // Check if the display is initialized
        if !self.is_initialized {
            return;
        }

        // Clear field
        self.draw_field(pixel_per_cell);

        // Draw state
        match drawable.state.name.as_str() {
            "scrum" => {
                self.draw_diamond(
                    (drawable.state.pos.x * pixel_per_cell as f32) as usize,
                    (drawable.state.pos.y * pixel_per_cell as f32) as usize,
                    (drawable.state.size * pixel_per_cell as f32) as usize,
                    SCRUM,
                    SCRUM_LINE
                );
                self.draw_line(
                    ((drawable.state.pos.x) * pixel_per_cell as f32) as usize,
                    OFFSIDE_LINE
                );
                self.draw_line(
                    ((drawable.state.pos.x - SCRUM_SIZE) * pixel_per_cell as f32) as usize,
                    OFFSIDE_LINE
                );
                self.draw_line(
                    ((drawable.state.pos.x + SCRUM_SIZE) * pixel_per_cell as f32) as usize,
                    OFFSIDE_LINE
                );
            },
            "ruck" => {
                self.draw_circle(
                    (drawable.state.pos.x * pixel_per_cell as f32) as usize,
                    (drawable.state.pos.y * pixel_per_cell as f32) as usize,
                    (drawable.state.size * pixel_per_cell as f32) as usize,
                    RUCK,
                    RUCK_LINE
                );
                self.draw_line(
                    ((drawable.state.pos.x + drawable.state.size) * pixel_per_cell as f32) as usize,
                    OFFSIDE_LINE
                );
                self.draw_line(
                    ((drawable.state.pos.x - drawable.state.size) * pixel_per_cell as f32) as usize,
                    OFFSIDE_LINE
                );
            },
            _ => {}
        }

        // Draw ball
        self.draw_square(
            (drawable.ball.x * pixel_per_cell as f32) as usize,
            (drawable.ball.y * pixel_per_cell as f32) as usize,
            pixel_per_cell / 2,
            WHITE
        );
        // Draw ball line
        self.draw_line(
            (drawable.ball.x * pixel_per_cell as f32) as usize,
            BLACK
        );

        // Draw home players
        for player in &drawable.home_players {
            self.draw_square(
                (player.pos.x * pixel_per_cell as f32) as usize,
                (player.pos.y * pixel_per_cell as f32) as usize,
                pixel_per_cell - 2,
                TEAM1
            );
            if player.number > 9 {
                self.draw_digit(
                    (player.pos.x * pixel_per_cell as f32) as usize - (pixel_per_cell - 2) / 2 + 1,
                    (player.pos.y * pixel_per_cell as f32) as usize - (pixel_per_cell - 2) / 3,
                    (pixel_per_cell - 2) / 5,
                    (player.number / 10) as u8,
                    WHITE
                );
                self.draw_digit(
                    (player.pos.x * pixel_per_cell as f32) as usize + 1,
                    (player.pos.y * pixel_per_cell as f32) as usize - (pixel_per_cell - 2) / 3,
                    (pixel_per_cell - 2) / 5,
                    (player.number % 10) as u8,
                    WHITE
                );
            } else {
                self.draw_digit(
                    (player.pos.x * pixel_per_cell as f32) as usize - (pixel_per_cell - 2) / 5,
                    (player.pos.y * pixel_per_cell as f32) as usize - (pixel_per_cell - 2) / 3,
                    (pixel_per_cell - 2) / 5,
                    player.number as u8,
                    WHITE
                );
            }
        }

        // Draw away players
        for player in &drawable.away_players {
            self.draw_square(
                (player.pos.x * pixel_per_cell as f32) as usize,
                (player.pos.y * pixel_per_cell as f32) as usize,
                pixel_per_cell - 2,
                TEAM2
            );
            if player.number > 9 {
                self.draw_digit(
                    (player.pos.x * pixel_per_cell as f32) as usize - (pixel_per_cell - 2) / 2 + 1,
                    (player.pos.y * pixel_per_cell as f32) as usize - (pixel_per_cell - 2) / 3,
                    (pixel_per_cell - 2) / 5,
                    (player.number / 10) as u8,
                    WHITE
                );
                self.draw_digit(
                    (player.pos.x * pixel_per_cell as f32) as usize + 1,
                    (player.pos.y * pixel_per_cell as f32) as usize - (pixel_per_cell - 2) / 3,
                    (pixel_per_cell - 2) / 5,
                    (player.number % 10) as u8,
                    WHITE
                );
            } else {
                self.draw_digit(
                    (player.pos.x * pixel_per_cell as f32) as usize - (pixel_per_cell - 2) / 5,
                    (player.pos.y * pixel_per_cell as f32) as usize - (pixel_per_cell - 2) / 3,
                    (pixel_per_cell - 2) / 5,
                    player.number as u8,
                    WHITE
                );
            }
        }

        // Draw time
        self.draw_time(drawable.time, pixel_per_cell);

        // Update the window
        self.window
            .update_with_buffer(&self.buffer, self.size, self.height)
            .unwrap();
    }

    fn draw_square(&mut self, x: usize, y: usize, size: usize, color: u32) {
        for dy in 0..size {
            for dx in 0..size {
                if x + dx < size / 2 || y + dy < size / 2 {
                    continue;
                }
                let px: usize = x + dx - size / 2;
                let py: usize = y + dy - size / 2;
                if py * self.size + px < self.size * self.height {
                    self.buffer[py * self.size + px] = color;
                }
            }
        }
    }

    fn draw_line(&mut self, x: usize, color: u32) {
        for i in 0..self.height {
            if  i * self.size + x >= self.size * self.height {
                continue;
            }
            self.buffer[i * self.size + x] = color;
        }
    }

    fn draw_diamond(
        &mut self,
        x: usize,
        y: usize,
        radius: usize,
        fill_color: u32,
        border_color: u32
    ) {
        let cx = x as isize;
        let cy = y as isize;
        let r = radius as isize;

        for dy in -r..=r {
            for dx in -r..=r {
                let manhattan = dx.abs() + dy.abs();

                if manhattan <= r {
                    let px = cx + dx;
                    let py = cy + dy;

                    if px >= 0 && py >= 0 &&
                       (px as usize) < self.size &&
                       (py as usize) < self.height
                    {
                        let idx = (py as usize) * self.size + (px as usize);

                        let color = if manhattan == r {
                            border_color
                        } else {
                            fill_color
                        };
                        self.buffer[idx] = color;
                    }
                }
            }
        }
    }

    fn draw_circle(
        &mut self,
        x: usize,
        y: usize,
        radius: usize,
        fill_color: u32,
        border_color: u32
    ) {
        let cx = x as isize;
        let cy = y as isize;
        let r = radius as isize;

        for dy in -r..=r {
            for dx in -r..=r {
                let px = cx + dx;
                let py = cy + dy;

                if px < 0 || py < 0 ||
                   (px as usize) >= self.size ||
                   (py as usize) >= self.height {
                    continue;
                }

                let dist_sq = (dx * dx + dy * dy) as f32;
                let r_sq = (r * r) as f32;

                if dist_sq <= r_sq {
                    let idx = (py as usize) * self.size + (px as usize);
                    let dist = dist_sq.sqrt();
                    let color = if (r as f32 - dist).abs() < 1.0 {
                        border_color
                    } else {
                        fill_color
                    };

                    self.buffer[idx] = color;
                }
            }
        }
    }

    fn draw_digit(
        &mut self,
        x: usize,
        y: usize,
        size: usize,
        digit: u8,
        color: u32
    ) {
        if digit > 9 { return; }

        let pattern = &FONT_3X5[digit as usize];

        for (row_idx, row) in pattern.iter().enumerate() {
            for (col_idx, c) in row.chars().enumerate() {
                if c == '1' {
                    // Scale the pixel block
                    for dy in 0..size {
                        for dx in 0..size {
                            let px = x + col_idx * size + dx;
                            let py = y + row_idx * size + dy;

                            if px < self.size && py * self.size + px < self.size * self.height {
                                self.buffer[py * self.size + px] = color;
                            }
                        }
                    }
                }
            }
        }
    }

    fn draw_field(&mut self, pixel_per_cell: usize) {
        // Clear screen to green
        self.buffer.fill(GROUND_COLOR);
        // Draw the field lines
        // Top & Bottom row
        if  self.width < 94 * pixel_per_cell ||
            self.width > 100 * pixel_per_cell ||
            self.height < 68 * pixel_per_cell ||
            self.height > 70 * pixel_per_cell ||
            self.try_size < 10 * pixel_per_cell ||
            self.try_size > 22 * pixel_per_cell ||
            pixel_per_cell < 1 {
            println!("Invalid field dimensions or pixel_per_cell. Please check your configuration.");
            println!("Width: {}, Height: {}, Try Size: {}, Pixel per Cell: {}", self.width, self.height, self.try_size, pixel_per_cell);
            return;
        }
        for j in 0..self.size {
            // Draw out lines
            for i in 0..pixel_per_cell {
                self.buffer[(i * self.size) + j] = GROUND_OUT_COLOR;
                self.buffer[(self.size * self.height) - ((i * self.size) + j) - 1] = GROUND_OUT_COLOR;
            }
            // Draw hashed lines
            let center = (pixel_per_cell * 5) / 2;
            if (j >= self.try_size + pixel_per_cell * 5 + 1) &&
            (j < self.try_size + pixel_per_cell * 10 + 1) ||
            (j >= self.try_size + (pixel_per_cell * 45) / 2 - center - (pixel_per_cell / 2 - 1)) &&
            (j < self.try_size + (pixel_per_cell * 45) / 2 + center - (pixel_per_cell / 2 - 1)) ||
            (j >= self.try_size + (pixel_per_cell * 81) / 2 - center - (pixel_per_cell / 2 - 1)) &&
            (j < self.try_size + (pixel_per_cell * 81) / 2 + center - (pixel_per_cell / 2 - 1)) ||
            (j >= self.try_size + self.width / 2 - center) &&
            (j < self.try_size + self.width / 2 + center) ||
            (j >= self.size - self.try_size - (pixel_per_cell * 81) / 2 - center + (pixel_per_cell / 2 - 1)) &&
            (j < self.size - self.try_size - (pixel_per_cell * 81) / 2 + center + (pixel_per_cell / 2 - 1)) ||
            (j >= self.size - self.try_size - (pixel_per_cell * 45) / 2 - center + (pixel_per_cell / 2 - 1)) &&
            (j < self.size - self.try_size - (pixel_per_cell * 45) / 2 + center + (pixel_per_cell / 2 - 1)) ||
            (j >= self.size - self.try_size - pixel_per_cell * 10 - 1) &&
            (j < self.size - self.try_size - pixel_per_cell * 5 - 1) {
                // Draw lines of 5 + 1 for out
                self.buffer[((1 + 6 * pixel_per_cell) * self.size) + j] = GROUND_LINE_COLOR;
                self.buffer[(self.size * self.height) - (((1 + 6 * pixel_per_cell) * self.size) + j) - 1] = GROUND_LINE_COLOR;
                // Draw lines of 15 + 1 for out
                self.buffer[((1 + 16 * pixel_per_cell) * self.size) + j] = GROUND_LINE_COLOR;
                self.buffer[(self.size * self.height) - (((1 + 16 * pixel_per_cell) * self.size) + j) - 1] = GROUND_LINE_COLOR;
            }
        }
        // Left & Right row
        for j in 1..(self.height-1) {
            // Draw out lines
            for i in 0..pixel_per_cell {
                self.buffer[j * self.size + i] = GROUND_OUT_COLOR;
                self.buffer[j * self.size + (self.size - i - 1)] = GROUND_OUT_COLOR;
            }
            // Draw the center line
            self.buffer[j * self.size + (self.size / 2)] = GROUND_LINE_COLOR;
            // Draw try lines
            self.buffer[j * self.size + 1 + self.try_size] = GROUND_LINE_COLOR;
            self.buffer[j * self.size + (self.size - self.try_size - 2)] = GROUND_LINE_COLOR;
            // Draw lines of 22
            self.buffer[j * self.size + 1 + self.try_size + (22 * pixel_per_cell)] = GROUND_LINE_COLOR;
            self.buffer[j * self.size + (self.size - self.try_size - (22 * pixel_per_cell) - 2)] = GROUND_LINE_COLOR;
            // Draw hashed lines of 5 + 1 for out
            let center = (pixel_per_cell * 5) / 2;
            if (j >= pixel_per_cell * 6 - center + 1) &&
            (j < pixel_per_cell * 6 + center + 1) ||
            (j >= pixel_per_cell * 16 - center + 1) &&
            (j < pixel_per_cell * 16 + center + 1) ||
            (j >= self.height / 2 - center - pixel_per_cell * 5 - 1) &&
            (j < self.height / 2 - center - 1) ||
            (j >= self.height / 2 + center + 1) &&
            (j < self.height / 2 + center + pixel_per_cell * 5 + 1) ||
            (j >= self.height - pixel_per_cell * 16 - center - 1) &&
            (j < self.height - pixel_per_cell * 16 + center - 1) ||
            (j >= self.height - pixel_per_cell * 6 - center - 1) &&
            (j < self.height - pixel_per_cell * 6 + center - 1) {
                self.buffer[j * self.size + 1 + self.try_size + (5 * pixel_per_cell)] = GROUND_LINE_COLOR;
                self.buffer[j * self.size + (self.size - self.try_size - (5 * pixel_per_cell) - 2)] = GROUND_LINE_COLOR;
            }
            // Draw hashed lines of 40 + 1 for out
            if (j >= pixel_per_cell * 6 - center + 1) &&
            (j < pixel_per_cell * 6 + center + 1) ||
            (j >= pixel_per_cell * 16 - center + 1) &&
            (j < pixel_per_cell * 16 + center + 1) ||
            (j >= self.height / 2 - center - pixel_per_cell * 10 + 1) &&
            (j < self.height / 2 - center - pixel_per_cell * 5 + 1) ||
            (j >= self.height / 2 - center) &&
            (j < self.height / 2 + center) ||
            (j >= self.height / 2 + center + pixel_per_cell * 5 - 1) &&
            (j < self.height / 2 + center + pixel_per_cell * 10 - 1) ||
            (j >= self.height - pixel_per_cell * 16 - center - 1) &&
            (j < self.height - pixel_per_cell * 16 + center - 1) ||
            (j >= self.height - pixel_per_cell * 6 - center - 1) &&
            (j < self.height - pixel_per_cell * 6 + center - 1) {
                self.buffer[j * self.size + 1 + self.try_size + (40 * pixel_per_cell)] = GROUND_LINE_COLOR;
                self.buffer[j * self.size + (self.size - self.try_size - (40 * pixel_per_cell) - 2)] = GROUND_LINE_COLOR;
            }
        }
    }

    fn draw_time(&mut self, time: u64, pixel_per_cell: usize) {
        self.time = time / 100;
        let minutes = self.time / 60;
        let seconds = self.time % 60;

        // Draw minutes
        self.draw_digit(
            15,
            15,
            pixel_per_cell / 2,
            (minutes / 10) as u8,
            WHITE
        );
        self.draw_digit(
            15 + (pixel_per_cell * 2),
            15,
            pixel_per_cell / 2,
            (minutes % 10) as u8,
            WHITE
        );

        // Draw colon
        for i in 0..(pixel_per_cell / 2) {
            for j in 0..(pixel_per_cell / 2) {
                self.buffer[(18 + i) * self.size + (15 + pixel_per_cell * 4 + j)] = WHITE;
                self.buffer[(22 + i + pixel_per_cell) * self.size + (15 + pixel_per_cell * 4 + j)] = WHITE;
            }
        }

        // Draw seconds
        self.draw_digit(
            15 + (pixel_per_cell * 5),
            15,
            pixel_per_cell / 2,
            (seconds / 10) as u8,
            WHITE
        );
        self.draw_digit(
            15 + (pixel_per_cell * 7),
            15,
            pixel_per_cell / 2,
            (seconds % 10) as u8,
            WHITE
        );
    }

    pub fn close(&mut self) {
        // No close method.
    }

    pub fn is_open(&self) -> bool {
        self.window.is_open()
    }
}
