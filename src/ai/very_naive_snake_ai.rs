use crate::{
    structs::{
        snake::Snake,
        board::Board,
    },
    utils::{
        u8vec2::U8Vec2,
        helper_types::{
            Step, Path
        }
    },
    draw_functions::draw_path,
};
use super::r#trait::SnakeAi;

const UP: Step = [false, false, true, false];
const DOWN: Step = [true, false, false, false];
const RIGHT: Step = [false, true, false, false];
const LEFT: Step = [false, false, false, true];

pub struct AI<'a> {
    pub snake: &'a mut Snake,
    path: Path<'a>,
}

impl SnakeAi for AI<'_> {
    fn next_step(&mut self, board: &Board) {
        let heading = U8Vec2{ x: 1, y: 1 };
        self.snake.set_heading(self.path.next());
        self.snake.update();
    }
}

impl<'a> AI<'a> {
    pub fn new(snake: &'a mut Snake, board: &Board) -> Self {
        let path = Self::find_hamiltonion_cycle(&snake, board);
        Self {
            snake,
            path,
        }
    }

    fn find_hamiltonion_cycle(snake: &Snake, board: &Board) -> Path<'a> {
        let cols = board.core.size_in_units.x;
        let rows = board.core.size_in_units.y;
        let mut arr: Vec<Step> = Vec::with_capacity(cols as usize * rows as usize);
        for _ in 1..snake.head.y {
            arr.push(DOWN)
        }
        arr.push(RIGHT);
        for _ in 1..((cols - snake.head.x) / 2) {
            for _ in 1..(rows - 1) {
                arr.push(UP)
            }
            arr.push(RIGHT);
            for _ in 1..(rows - 1) {
                arr.push(DOWN)
            }
            arr.push(RIGHT);
        }
        for _ in 0..cols {
            arr.push(LEFT)
        }
        for _ in 0..((snake.head.x) / 2) {
            for _ in 1..(rows - 1) {
                arr.push(UP)
            }
            arr.push(RIGHT);
            for _ in 1..(rows - 1) {
                arr.push(DOWN)
            }
            arr.push(RIGHT);
        }
        arr.into_iter()
    }
}
