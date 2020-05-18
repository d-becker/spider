use std::borrow::Borrow;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }

    pub fn origin() -> Point {
        Point::new(0, 0)
    }

    pub fn add<T: Borrow<Point>>(&self, other: T) -> Point {
        let borrowed = other.borrow();
        let x = self.x + borrowed.x;
        let y = self.y + borrowed.y;
        Point::new(x, y)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
    NONE,
}

impl Direction {
    pub fn to_point(&self) -> Point {
        match self {
            Direction::UP => Point::new(0, -1),
            Direction::DOWN => Point::new(0, 1),
            Direction::LEFT => Point::new(-1, 0),
            Direction::RIGHT => Point::new(1, 0),
            Direction::NONE => Point::new(0, 0),
        }
    }
}

#[cfg(test)]
mod tests;
