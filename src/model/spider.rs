use super::point::{Direction, Point};
use super::rectilinear;

#[derive(Debug)]
pub struct Spider {
    dir: Direction,
    path: rectilinear::Path,
}

impl Spider {
    pub fn new(pos: Point, dir: Direction) -> Spider {
        Spider {
            dir,
            path: rectilinear::Path::with_start(pos),
        }
    }

    pub fn get_pos(&self) -> Point {
        self.path.last().unwrap()
    }

    // TODO: Use Path in public interface.
    pub fn get_points(&self) -> &[Point] {
        self.path.points()
    }

    pub fn update(&mut self) {
        let pos = self.get_pos();

        let dir_point = self.dir.to_point();
        let new_pos = pos.add(dir_point);

        self.path.add(new_pos).expect(
            "Should not happen because the available directions guarantee rectilinearity..",
        );
    }

    pub fn get_dir(&self) -> Direction {
        self.dir
    }

    pub fn set_dir(&mut self, dir: Direction) {
        self.dir = dir;
    }
}

#[cfg(test)]
mod tests;
