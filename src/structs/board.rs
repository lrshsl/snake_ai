use crate::{utils::u8vec2::U8Vec2, vec2, Vec2};

#[derive(Clone)]
pub struct BoardCore {
    screen_size: Vec2,
    pub size_in_units: U8Vec2,
    min_margin_in_per_cent: f32,
}

pub struct Board {
    pub core: BoardCore,
    pub cell_width: f32,
    pub size: Vec2,
    pub min: Vec2,
    pub max: Vec2,
}

impl Board {
    pub fn new(
        nb_cols: u8,
        nb_rows: u8,
        min_margin_in_per_cent: f32,
        screen_width: f32,
        screen_height: f32,
    ) -> Self {
        // Core
        let screen_size = vec2(screen_width, screen_height);
        let size_in_units = U8Vec2 {
            x: nb_cols,
            y: nb_rows,
        };

        let mut core = BoardCore {
            screen_size,
            size_in_units,
            min_margin_in_per_cent,
        };

        // Shell
        Self::from_core(core)
    }

    pub fn update_screen_size(&mut self, width: f32, height: f32) {
        self.core.screen_size = vec2(width, height);
        *self = Self::from_core(self.core.clone()); // TODO: Optimize?
    }
}

impl Board {
    fn from_core(core: BoardCore) -> Self {
        // Aliases for readability
        fn pcent2dec(x: f32) -> f32 {
            x / 100.
        }
        let fsize_in_units: Vec2 = core.size_in_units.to_vec2();

        // Calculate proto size
        let proto_margin = core.screen_size * pcent2dec(core.min_margin_in_per_cent);
        let proto_board_size: Vec2 = core.screen_size - 2. * proto_margin;
        let proto_cell_size: Vec2 = proto_board_size / fsize_in_units;

        // Make cells quadratic
        let cell_width = proto_cell_size.min_element();
        let board_size = cell_width * fsize_in_units;
        let margin_in_px = (core.screen_size - board_size) * 0.5;

        // Aliases for readability
        let size = board_size;
        let min = margin_in_px;

        Self {
            core,
            cell_width,
            size,
            min,
            max: size + min,
        }
    }
}
