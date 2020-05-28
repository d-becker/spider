use super::point::{Direction, Point};
use super::rectilinear;

#[derive(Debug)]
pub struct Spider {
    dir_: Direction,
    pos_: Point,
    path_: Option<rectilinear::Path>,

    upper_left_: Point,
    lower_right_: Point,
}

impl Spider {
    pub fn new(pos: Point, dir: Direction, upper_left: Point, lower_right: Point) -> Spider {
        Spider {
            dir_: dir,
            pos_: pos,
            path_: None,
            upper_left_: upper_left,
            lower_right_: lower_right,
        }
    }

    pub fn pos(&self) -> &Point {
        &self.pos_
    }

    pub fn start_path(&mut self) {
        self.path_ = Some(rectilinear::Path::with_start(*self.pos()));
    }

    pub fn stop_path(&mut self) {
        self.path_ = None;
    }

    pub fn has_path(&self) -> bool {
        self.path().is_some()
    }

    pub fn path(&self) -> Option<&rectilinear::Path> {
        self.path_.as_ref()
    }

    pub fn update(&mut self) {
        let pos = self.pos();
        let dir_point = self.dir_.to_point();
        let new_pos = pos.add(dir_point);

        if !self.pos_in_bounds(&new_pos) {
            return;
        }

        self.pos_ = new_pos;

        if let Some(ref mut path) = self.path_ {
            path.add(new_pos).expect(
                "Should not happen because the available directions guarantee rectilinearity.",
            );
        }
    }

    pub fn get_dir(&self) -> Direction {
        self.dir_
    }

    pub fn set_dir(&mut self, dir: Direction) {
        self.dir_ = dir;
    }

    fn pos_in_bounds(&self, new_pos: &Point) -> bool {
        self.upper_left_.x <= new_pos.x
            && new_pos.x <= self.lower_right_.x
            && self.upper_left_.y <= new_pos.y
            && new_pos.y <= self.lower_right_.y
    }
}

#[cfg(test)]
mod tests;
