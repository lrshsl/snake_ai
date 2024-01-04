use crate::Color;

pub mod snake_colors {
    use crate::Color;

    pub use macroquad::color::colors::{
        GRAY, WHITE, GREEN,
    };
    pub const DARK_GREEN: Color = Color::new(0.0, 0.7, 0.1, 1.);
}

use snake_colors::*;

// Game
pub const GAME_TICK: f64 = 0.15; // seconds

// Grid
pub const GRID_COLOR: Color = GRAY;
pub const BORDER_COLOR: Color = WHITE;
pub const MARGIN: f32 = 10.0; // per cent

// Snake
pub const SNAKE_HEAD_COLOR: Color = DARK_GREEN;
pub const SNAKE_TAIL_COLOR: Color = GREEN;

// Effective
pub const NB_COLUMNS: u8 = 30;
pub const NB_ROWS: u8 = NB_COLUMNS;
