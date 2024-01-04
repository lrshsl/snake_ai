use crate::structs::board::Board;

pub trait SnakeAi {
    fn next_step(&mut self, board: &Board);
}
