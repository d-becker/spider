use super::point::{Direction, Point};
use super::rectilinear;

#[derive(Debug)]
pub struct Spider {
    dir_: Direction,
    path_: rectilinear::Path,
}

impl Spider {
    pub fn new(pos: Point, dir: Direction) -> Spider {
        Spider {
            dir_: dir,
            path_: rectilinear::Path::with_start(pos),
        }
    }

    pub fn pos(&self) -> Point {
        self.path_.last().unwrap()
    }

    pub fn path(&self) -> &rectilinear::Path {
        &self.path_
    }

    pub fn update(&mut self) {
        let pos = self.pos();

        let dir_point = self.dir_.to_point();
        let new_pos = pos.add(dir_point);

        self.path_.add(new_pos).expect(
            "Should not happen because the available dir_ections guarantee rectilinearity..",
        );
    }

    pub fn get_dir(&self) -> Direction {
        self.dir_
    }

    pub fn set_dir(&mut self, dir: Direction) {
        self.dir_ = dir;
    }
}

#[cfg(test)]
mod tests;
