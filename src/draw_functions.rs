use crate::{
    draw_circle, draw_line, draw_rectangle_lines, utils::util_functions::cell_to_xy, Board, Snake,
    BORDER_COLOR, GRID_COLOR, SNAKE_HEAD_COLOR, SNAKE_TAIL_COLOR,
};

pub fn draw_snake(snake: &Snake, board: &Board) {
    // Head
    let (head_x, head_y) = cell_to_xy(&snake.head, board);
    draw_circle(head_x, head_y, board.cell_width * 0.5, SNAKE_HEAD_COLOR);
    
    // Body
    for tail_part in snake.tail.iter() {
        let (x, y) = cell_to_xy(tail_part, board);
        draw_circle(x, y, board.cell_width * 0.5, SNAKE_TAIL_COLOR);
    }
}

pub fn draw_grid(board: &Board) {
    // Borders
    draw_rectangle_lines(board.min.x, board.min.y, board.size.x, board.size.y, 3., BORDER_COLOR);

    // Vertical lines
    for col in 0..board.core.size_in_units.x {
        let x = (col as f32) * board.cell_width + board.min.x;
        draw_line(x, board.min.y, x, board.max.y, 1., GRID_COLOR)
    }

    // Horizontal lines
    for row in 0..board.core.size_in_units.y {
        let y = (row as f32) * board.cell_width + board.min.y;
        draw_line(board.min.x, y, board.max.x, y, 1., GRID_COLOR)
    }
    // TODO: DRY..
}

pub fn draw_path(board: &Board) {
    // TODO
}
