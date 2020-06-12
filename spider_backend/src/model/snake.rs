use rand::seq::SliceRandom;

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
        let mut rng = rand::thread_rng();
        let directions = [Direction::UP, Direction::LEFT, Direction::DOWN, Direction::RIGHT, Direction::NONE];
        *directions.choose(&mut rng).unwrap()
    }

    pub fn pos(&self) -> &Point {
        &self.pos_
    }

    pub fn set_pos(&mut self, point: Point) {
        self.pos_ = point;
    }
}
