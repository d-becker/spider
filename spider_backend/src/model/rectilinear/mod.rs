pub mod line;
pub mod path;
pub mod polygon;

use std::borrow::Borrow;

pub use line::{Line, LineIntersection};
pub use path::Path;
pub use polygon::Polygon;

use super::point::Point;

pub fn horizontal(p1: &Point, p2: &Point) -> bool {
    p1.y == p2.y
}

pub fn vertical(p1: &Point, p2: &Point) -> bool {
    p1.x == p2.x
}

pub fn rectilinear<PointT1, PointT2>(p1: PointT1, p2: PointT2) -> bool
where
    PointT1: Borrow<Point>,
    PointT2: Borrow<Point>,
{
    let p1 = p1.borrow();
    let p2 = p2.borrow();

    horizontal(p1, p2) || vertical(p1, p2)
}

fn insertion_point<PointT, Iter>(iter: Iter, point: &Point) -> Option<usize>
where
    Iter: Iterator<Item = Line<PointT>>,
    PointT: Borrow<Point>,
{
    for (i, line) in iter.enumerate() {
        if line.contains(point) {
            return Some(i + 1);
        }
    }

    None
}

#[cfg(test)]
mod tests;
