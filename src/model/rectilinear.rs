use std::borrow::Borrow;

use super::point::Point;

pub fn collinear<PointT1, PointT2, PointT3>(p1: PointT1, p2: PointT2, p3: PointT3) -> bool
where
    PointT1: Borrow<Point>,
    PointT2: Borrow<Point>,
    PointT3: Borrow<Point>,
{
    let p1 = p1.borrow();
    let p2 = p2.borrow();
    let p3 = p3.borrow();

    vertical_collinear(p1, p2, p3) || horizontal_collinear(p1, p2, p3)
}

pub fn line_contains<PointT1, PointT2, PointT3>(start: PointT1, end: PointT2, p: PointT3) -> bool
where
    PointT1: Borrow<Point>,
    PointT2: Borrow<Point>,
    PointT3: Borrow<Point>,
{
    let start = start.borrow();
    let end = end.borrow();
    let p = p.borrow();

    if vertical_collinear(start, end, p) {
        (start.y <= p.y && p.y <= end.y) || (end.y <= p.y && p.y <= start.y)
    } else if horizontal_collinear(start, end, p) {
        (start.x <= p.x && p.x <= end.x) || (end.x <= p.x && p.x <= start.x)
    } else {
        false
    }
}

fn vertical_collinear(p1: &Point, p2: &Point, p3: &Point) -> bool {
    p1.x == p2.x && p2.x == p3.x
}

fn horizontal_collinear(p1: &Point, p2: &Point, p3: &Point) -> bool {
    p1.y == p2.y && p2.y == p3.y
}

#[derive(Debug)]
pub struct Path {
    points_: Vec<Point>,
}

impl Path {
    pub fn new() -> Path {
        Path { points_: Vec::new() }
    }

    pub fn with_start(start: Point) -> Path {
        Path {
            points_: vec![start],
        }
    }

    pub fn last(&self) -> Option<Point> {
        self.points_.last().map(|&point| point)
    }

    pub fn points(&self) -> &[Point] {
        &self.points_
    }

    pub fn add(&mut self, point: Point) -> Result<(), &'static str> {
        if let Some(last_point) = self.points_.last() {
            if last_point.x != point.x && last_point.y != point.y {
                return Result::Err("Not rectilinear.");
            }

            if *last_point == point {
                return Ok(());
            }
        }

        self.handle_loop(&point);
        self.handle_collinearity(&point);

        self.points_.push(point);
        Result::Ok(())
    }

    fn handle_loop(&mut self, new_pos: &Point) {
        let mut ind = None;
        for (i, pair) in self.points_.windows(2).enumerate() {
            if let &[start, end] = pair {
                if line_contains(start, end, new_pos) {
                    ind = Some(i);
                    break;
                }
            } else {
                unreachable!();
            }
        }

        if let Some(i) = ind {
            self.points_.truncate(i + 1);
        }
    }

    fn handle_collinearity(&mut self, new_pos: &Point) {
        let mut should_delete = false;
        if let [.., ref second_last, ref last] = self.points_.as_slice() {
            if collinear(second_last, last, new_pos) {
                should_delete = true;
            }
        }

        if should_delete {
            self.points_.pop();
        }
    }
}

pub struct Polygon {
}

#[cfg(test)]
mod tests;
