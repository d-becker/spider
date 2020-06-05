use std::borrow::Borrow;

use itertools::Itertools;

use super::rectilinear;
use crate::model::point::{Direction, Point};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Line<PointT>
where
    PointT: Borrow<Point>,
{
    start: PointT,
    end: PointT,
}

#[derive(Debug, Eq, PartialEq)]
pub enum LineIntersection {
    Line(Line<Point>),
    Point(Point),
}

impl<PointT: Borrow<Point>> Line<PointT> {
    pub fn from_points(start: PointT, end: PointT) -> Option<Line<PointT>> {
        if rectilinear(start.borrow(), end.borrow()) {
            Some(Line { start, end })
        } else {
            None
        }
    }

    pub fn start(&self) -> &Point {
        self.start.borrow()
    }

    pub fn end(&self) -> &Point {
        self.end.borrow()
    }

    pub fn vertical(&self) -> bool {
        self.start().x == self.end().x
    }

    pub fn horizontal(&self) -> bool {
        self.start().y == self.end().y
    }

    pub fn direction(&self) -> Direction {
        let diff = self.end().subtract(self.start());

        if diff.x == 0 {
            if diff.y > 0 {
                Direction::DOWN
            } else if diff.y < 0 {
                Direction::UP
            } else {
                Direction::NONE
            }
        } else {
            debug_assert!(diff.y == 0);
            if diff.x > 0 {
                Direction::RIGHT
            } else {
                debug_assert!(diff.x < 0);
                Direction::LEFT
            }
        }
    }

    pub fn collinear<PointT2>(&self, p: PointT2) -> bool
    where
        PointT2: Borrow<Point>,
    {
        let p1 = self.start();
        let p2 = self.end();
        let p3 = p.borrow();

        vertical_collinear(p1, p2, p3) || horizontal_collinear(p1, p2, p3)
    }

    // Negative => right.
    // Positive => left.
    pub fn point_on_side<PointT2>(&self, point: PointT2) -> i32
    where
        PointT2: Borrow<Point>,
    {
        // From https://math.stackexchange.com/a/274728 and
        // https://en.wikipedia.org/wiki/Curve_orientation#Orientation_of_a_simple_polygon.
        let point = point.borrow();
        let start = self.start();
        let end = self.end();
        let d = (point.x - start.x) * (end.y - start.y) - (point.y - start.y) * (end.x - start.x);
        d.signum()
    }

    pub fn contains<PointT2>(&self, p: PointT2) -> bool
    where
        PointT2: Borrow<Point>,
    {
        let start = self.start();
        let end = self.end();
        let p = p.borrow();

        if vertical_collinear(start, end, p) {
            (start.y <= p.y && p.y <= end.y) || (end.y <= p.y && p.y <= start.y)
        } else if horizontal_collinear(start, end, p) {
            (start.x <= p.x && p.x <= end.x) || (end.x <= p.x && p.x <= start.x)
        } else {
            false
        }
    }

    pub fn intersects<PointT2>(&self, other: &Line<PointT2>) -> bool
    where
        PointT2: Borrow<Point>,
    {
        self.intersection(other).is_some()
    }

    pub fn intersection<PointT2>(&self, other: &Line<PointT2>) -> Option<LineIntersection>
    where
        PointT2: Borrow<Point>,
    {
        let l1_h = [self.start().borrow().x, self.end().borrow().x];
        let l2_h = [other.start().borrow().x, other.end().borrow().x];

        let l1_v = [self.start().borrow().y, self.end().borrow().y];
        let l2_v = [other.start().borrow().y, other.end().borrow().y];

        let horiz_overlap = intervals_overlap(l1_h, l2_h)?;
        let vertical_overlap = intervals_overlap(l1_v, l2_v)?;

        let horiz_is_point = horiz_overlap.0 == horiz_overlap.1;
        let vert_is_point = vertical_overlap.0 == vertical_overlap.1;

        if horiz_is_point && vert_is_point {
            Some(LineIntersection::Point(Point::new(
                horiz_overlap.0,
                vertical_overlap.0,
            )))
        } else if horiz_is_point {
            debug_assert!(!vert_is_point);
            let x = horiz_overlap.0;
            let p1 = Point::new(x, vertical_overlap.0);
            let p2 = Point::new(x, vertical_overlap.1);
            Some(LineIntersection::Line(Line::from_points(p1, p2).unwrap()))
        } else {
            // The intersection of lines cannot be 2 dimensional.
            debug_assert!(vert_is_point);
            let y = vertical_overlap.0;
            let p1 = Point::new(horiz_overlap.0, y);
            let p2 = Point::new(horiz_overlap.1, y);
            Some(LineIntersection::Line(Line::from_points(p1, p2).unwrap()))
        }
    }

    pub fn intersects_half_line(&self, half_line_point: &Point, half_line_dir: Direction) -> bool {
        self.intersection_with_half_line(half_line_point, half_line_dir)
            .is_some()
    }

    pub fn intersection_with_half_line(
        &self,
        half_line_point: &Point,
        half_line_dir: Direction,
    ) -> Option<LineIntersection> {
        // TODO: Better solution?
        let half_line_end = {
            let mut pt = *half_line_point;
            match half_line_dir {
                Direction::UP => pt.y = i32::MIN,
                Direction::DOWN => pt.y = i32::MAX,
                Direction::LEFT => pt.x = i32::MIN,
                Direction::RIGHT => pt.x = i32::MAX,
                Direction::NONE => {}
            };
            pt
        };

        let half_line = Line::from_points(half_line_point, &half_line_end).unwrap();
        self.intersection(&half_line)
    }
}

pub fn intersections_line_iters<PointT1, PointT2, LineT1, LineT2, Iter1, Iter2>(
    iter1: Iter1,
    iter2: Iter2,
) -> impl Iterator<Item = LineIntersection>
where
    PointT1: Borrow<Point>,
    PointT2: Borrow<Point>,
    LineT1: Borrow<Line<PointT1>>,
    LineT2: Borrow<Line<PointT2>>,
    Iter1: Iterator<Item = LineT1> + Clone,
    Iter2: Iterator<Item = LineT2>,
{
    let intersections_with_line = |iter_clone: Iter1, line2: LineT2| {
        iter_clone.filter_map(move |line1| line1.borrow().intersection(line2.borrow()))
    };

    iter2
        .flat_map(move |line2| intersections_with_line(iter1.clone(), line2))
        .dedup()
}

fn intervals_overlap(mut int1: [i32; 2], mut int2: [i32; 2]) -> Option<(i32, i32)> {
    int1.sort();
    int2.sort();

    let larger_start = i32::max(int1[0], int2[0]);
    let smaller_end = i32::min(int1[1], int2[1]);

    if larger_start <= smaller_end {
        Some((larger_start, smaller_end))
    } else {
        None
    }
}

fn vertical_collinear(p1: &Point, p2: &Point, p3: &Point) -> bool {
    p1.x == p2.x && p2.x == p3.x
}

fn horizontal_collinear(p1: &Point, p2: &Point, p3: &Point) -> bool {
    p1.y == p2.y && p2.y == p3.y
}

// TODO: Move it somewhere else.
pub struct SkipLast<I>
where
    I : Iterator
{
    iter_: std::iter::Peekable<I>,
}

impl<I> Iterator for SkipLast<I>
where
    I: Iterator,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter_.peek()?;
        self.next()
    }
}
