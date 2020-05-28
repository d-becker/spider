use super::field::Field;
use super::point::{Direction, Point};
use super::spider::Spider;

#[derive(Debug)]
pub struct Snake {
    pos_: Point,
}

impl Snake {
    pub fn new(pos: Point) -> Snake {
        Snake { pos_: pos }
    }

    pub fn next_step(&self, field: &Field, spider: &Spider) -> Direction {
        // TODO: Implement it.
        Direction::NONE
    }

    pub fn pos(&self) -> &Point {
        &self.pos_
    }

    pub fn set_pos(&mut self, point: Point) {
        self.pos_ = point;
    }
}
