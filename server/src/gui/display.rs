use minifb::{Window, WindowOptions};
use crate::game::game_state::GameState;

const GROUND_COLOR: u32 = 0xFF66D575;
const GROUND_LINE_COLOR: u32 = 0xFFCDF4D3;
const GROUND_OUT_COLOR: u32 = 0xFFD9D9D9;
const TEAM1: u32 = 0xFFFF0000;
const TEAM2: u32 = 0xFF0000FF;
const WHITE: u32 = 0xFFFFFFFF;

pub struct Display {
    window: Window,
    buffer: Vec<u32>,
    width: usize,
    height: usize,
    size: usize,
    try_size: usize,
}

impl Display {
    pub fn new(width: usize, height: usize, try_size: usize) -> Self {
        let size: usize = width + 2 * try_size;
        let window = Window::new(
            "Rugby Simulator",
            size,
            height,
            WindowOptions::default(),
        ).unwrap_or_else(|e| {
            panic!("Unable to open window: {}", e);
        });
        let buffer: Vec<u32> = vec![GROUND_COLOR; size * height];

        Self { window, buffer, width, height, size, try_size}
    }

    pub fn render(&mut self, state: &GameState, pixel_per_cell: usize) {
        // Clear field
        // self.draw_field2(pixel_per_cell); for a funny display like a blackberry
        self.draw_field(pixel_per_cell);

        // Draw players
        for player in &state.players {
            self.draw_square(
                player.x * pixel_per_cell,
                player.y * pixel_per_cell,
                pixel_per_cell - 2,
                if player.team == 1 { TEAM1 } else { TEAM2 }
            );
        }

        // Draw ball
        self.draw_square(
            state.ball.x * pixel_per_cell,
            state.ball.y * pixel_per_cell,
            pixel_per_cell / 2,
            WHITE
        );

        // Update the window
        self.window
            .update_with_buffer(&self.buffer, self.size, self.height)
            .unwrap();
    }

    fn draw_square(&mut self, x: usize, y: usize, size: usize, color: u32) {
        for dy in 0..size {
            for dx in 0..size {
                let px: usize = x + dx;
                let py = y + dy;
                if px < self.size && py < self.height {
                    self.buffer[py * self.size + px] = color;
                }
            }
        }
    }

    fn draw_field(&mut self, pixel_per_cell: usize) {
        // Clear screen to green
        self.buffer.fill(GROUND_COLOR);
        // Draw the field lines
        // Top & Bottom row
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

    pub fn close(&mut self) {
        // No close method.
    }

    pub fn is_open(&self) -> bool {
        self.window.is_open()
    }
}
