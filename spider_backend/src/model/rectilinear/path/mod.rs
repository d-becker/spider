use std::borrow::Borrow;

use super::{Line, LineIntersection};
use crate::iter::skip_last::SkipLastIterator;
use crate::model::point::Point;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Path {
    points_: Vec<Point>,
}

impl Path {
    pub fn new() -> Path {
        Path {
            points_: Vec::new(),
        }
    }

    pub fn with_start(start: Point) -> Path {
        Path {
            points_: vec![start],
        }
    }

    pub fn with_points<PointT, Iter>(points: Iter) -> Option<Path>
    where
        PointT: Borrow<Point>,
        Iter: Iterator<Item = PointT>,
    {
        let mut path = Path::new();

        for point in points {
            path.add(*point.borrow()).ok()?;
        }

        Some(path)
    }

    pub fn first(&self) -> Option<&Point> {
        self.points_.first()
    }

    pub fn last(&self) -> Option<&Point> {
        self.points_.last()
    }

    pub fn points(&self) -> &[Point] {
        &self.points_
    }

    pub fn line_iter(
        &self,
    ) -> impl DoubleEndedIterator<Item = Line<&Point>> + Clone + ExactSizeIterator {
        self.points()
            .iter()
            .zip(self.points().iter().skip(1))
            .map(|(p1, p2)| Line::from_points(p1, p2).unwrap())
    }

    pub fn add(&mut self, point: Point) -> Result<(), &'static str> {
        if let Some(last_point) = self.points_.last() {
            if *last_point == point {
                return Ok(());
            }

            let new_line = Line::from_points(last_point, &point).ok_or("Not rectilinear.")?;
            let points_to_add = self.compute_real_points_to_add(&new_line, point);

            for pt in points_to_add {
                self.handle_loop(&pt);
                self.handle_collinearity(&pt);

                self.points_.push(pt);
            }
        } else {
            self.points_.push(point);
        }

        Result::Ok(())
    }

    pub fn remove(&mut self) -> Option<Point> {
        self.points_.pop()
    }

    pub fn insertion_point(&self, point: &Point) -> Option<usize> {
        super::insertion_point(self.line_iter(), point)
    }

    pub fn contains(&self, point: &Point) -> bool {
        self.insertion_point(point).is_some()
    }

    fn compute_real_points_to_add(
        &self,
        new_line: &Line<&Point>,
        point: Point,
    ) -> impl Iterator<Item = Point> {
        let mut points_to_add = vec![point];

        // We do not need the last line segment, that is always connected at the end and if
        // there are further connections, it is handled in 'handle_loop'.
        for line in self.line_iter().skip_last() {
            let intersection = line.intersection(&new_line);
            if let Some(int) = intersection {
                match int {
                    LineIntersection::Point(point) => {
                        if point != points_to_add[0] {
                            points_to_add.push(point);
                        }
                    }
                    LineIntersection::Line(line) => {
                        let pt = line.start(); // Does it matter which end?
                        if pt != &points_to_add[0] {
                            points_to_add.push(*pt);
                        }
                    }
                }

                break;
            }
        }

        points_to_add.into_iter().rev()
    }

    fn handle_loop(&mut self, new_pos: &Point) {
        if let Some(i) = self.insertion_point(new_pos) {
            self.points_.truncate(i);
        }
    }

    fn handle_collinearity(&mut self, new_pos: &Point) {
        let last_line_opt = self.line_iter().rev().next();
        if let Some(line) = last_line_opt {
            if line.collinear(new_pos) {
                self.points_.pop();
            }
        }
    }
}

#[cfg(test)]
mod tests;
