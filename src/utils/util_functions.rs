use crate::{
    structs::board::Board,
    utils::{
        u8vec2::U8Vec2, 
        helper_types::Step,
    }
};

pub fn cell_to_xy(cell: &U8Vec2, board: &Board) -> (f32, f32) {
    let cell_x = cell.x as f32 * board.cell_width + board.cell_width * 0.5;
    let x = cell_x + board.min.x;

    let cell_y = cell.y as f32 * board.cell_width + board.cell_width * 0.5;
    let y = cell_y + board.min.y;
    (x, y)
}

pub fn move_by_step(pos: &mut U8Vec2, step: &Step) {
    pos.x += step[1] as u8;
    pos.x -= step[3] as u8;
    pos.y += step[2] as u8;
    pos.y -= step[0] as u8;
}

