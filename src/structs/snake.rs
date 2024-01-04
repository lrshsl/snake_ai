use crate::{
    structs::board::Board,
    utils::{
        u8vec2::{U8Vec2, u8vec2},
        helper_types::Step,
        util_functions::move_by_step,
    },
};

pub struct Snake {
    pub head: U8Vec2,
    pub tail: Vec<U8Vec2>,
    moving_tail_part: usize,
    heading: Step,
}

impl Snake {
    pub fn new(board: &Board) -> Self {
        let col = board.core.size_in_units.x / 2;
        let row = board.core.size_in_units.y / 2;

        // Start position
        let head = u8vec2(col + 2, row);
        let mut tail = vec![
            u8vec2(col + 1, row + 1),
            u8vec2(col, row + 1),
            u8vec2(col, row),
            u8vec2(col + 1, row),
        ];
        tail.reserve_exact(
            board.core.size_in_units.x as usize * board.core.size_in_units.x as usize);
        Self {
            head,
            tail,
            moving_tail_part: 0,
            heading: [false, true, false, false],
        }
    }

    pub fn update(&mut self) {
        self.step_forward();
        self.eat_if_possible();
        self.die_if_necessairy();
    }

    fn step_forward(&mut self) {
        let old_head_pos = self.head.clone(); // TODO: HELP!!
        move_by_step(&mut self.head, &self.heading);
        self.tail[self.moving_tail_part] = old_head_pos;
        self.moving_tail_part += 1;
        self.moving_tail_part %= 4;
    }

    fn eat_if_possible(&mut self) { }
    fn die_if_necessairy(&mut self) { }

    pub fn set_heading(&mut self, heading: Option<Step>) {
        match heading {
            Some( val ) => self.heading = val,
            None => {}
        }
    }
}
