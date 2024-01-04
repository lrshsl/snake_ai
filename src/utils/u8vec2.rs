use crate::{
    Vec2,
};
use core::ops::*;

#[derive(Clone)]
pub struct U8Vec2 {
    pub x: u8,
    pub y: u8
}

impl U8Vec2 {
    pub fn to_vec2(&self) -> Vec2 {
        Vec2::new(
            self.x as f32, self.y as f32 )
    }
}

pub fn u8vec2(x: u8, y: u8) -> U8Vec2 {
    U8Vec2{x, y}
}

impl Into<Vec2> for U8Vec2 {
    fn into(self) -> Vec2 {
        Vec2::new(self.x as f32, self.y as f32)
    }
}

impl Into<(u8, u8)> for U8Vec2 {
    fn into(self) -> (u8, u8) {
        (self.x, self.y)
    }
}

impl AddAssign<U8Vec2> for U8Vec2 {
    #[inline]
    fn add_assign(&mut self, other: Self) {
        self.x.add_assign(other.x);
        self.y.add_assign(other.y);
    }
}
