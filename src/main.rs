use macroquad::prelude::*;

use ai::{
    very_naive_snake_ai::AI,
    r#trait::SnakeAi,
};
use structs::{ board::Board, snake::Snake };
use draw_functions::{ draw_snake, draw_grid };
use constants::*;

mod ai;
mod structs;
mod utils;
mod draw_functions;
mod constants;


#[macroquad::main("Snake 1.0")]
async fn main() {
    // Make a window
    next_frame().await;

    // Initialize variables
    let mut board = Board::new(
        NB_COLUMNS, NB_ROWS, MARGIN,
        screen_width(), screen_height()
    );
    let mut snake = Snake::new(&board);
    let mut ai = AI::new(&mut snake, &board);

    // Game loop
    let mut last_step = get_time();
    loop {
        // Update state only if necessary (game-tick = minimum GAME_TICK seconds)
        if get_time() - last_step > GAME_TICK {
            last_step = get_time();
            ai.next_step(&board);
        }

        // Drawing
        board.update_screen_size(screen_width(), screen_height());
        draw_grid(&board);
        draw_snake(&ai.snake, &board);

        next_frame().await
    }
}
