mod line;

use std::borrow::Borrow;
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::iter;

use itertools::Itertools;

use crate::iter::skip_last::SkipLastIterator;

use super::point::{Direction, Point};
pub use line::{Line, LineIntersection};

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
        insertion_point(self.line_iter(), point)
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

/// A rectilinear polygon.
#[derive(Debug, Eq)]
pub struct Polygon {
    path_: Path,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PolygonError {
    NotEnoughVertices,
    NonRectilinear,
    SelfIntersecting,
}

impl Polygon {
    /// An implicit edge is assumed between the last and the first point.
    pub fn with_path(mut path: Path) -> Result<Polygon, PolygonError> {
        match &path.points() {
            &[] => Ok(Polygon { path_: path }),
            &[_] => Err(PolygonError::NotEnoughVertices),
            &[_, _] => Err(PolygonError::NotEnoughVertices),
            &[_, _, _] => Err(PolygonError::NotEnoughVertices),
            &[first, _, .., second_last, last] => {
                let last_line = {
                    let (first, second_last, last) = (*first, *second_last, *last);
                    let line =
                        Line::from_points(last, first).ok_or(PolygonError::NonRectilinear)?;

                    if line.collinear(second_last) {
                        path.remove();
                        Line::from_points(second_last, first).unwrap()
                    } else {
                        line
                    }
                };

                // The first and the last line segments in the path always intersect last_line at
                // the end vertices, therefore they are filtered out. The other line segments
                // should not, otherwise the polygon is self-intersecting.
                let intersecting_lines = path
                    .line_iter()
                    .skip(1) // Filter out the first line segment.
                    .skip_last() // Filter out the last line segment.
                    .any(|line| line.intersects(&last_line));

                if intersecting_lines {
                    return Err(PolygonError::SelfIntersecting);
                }

                Ok(Polygon { path_: path })
            }
        }
    }

    pub fn path(&self) -> &Path {
        &self.path_
    }

    pub fn vertex_iter_from_ind(
        &self,
        start_idx: usize,
    ) -> impl DoubleEndedIterator<Item = &Point> {
        let first_part = self.path().points().iter().skip(start_idx);
        let second_part = self.path().points().iter().take(start_idx);
        first_part.chain(second_part)
    }

    pub fn vertex_iter_from_ind_backwards(
        &self,
        start_idx: usize,
    ) -> impl DoubleEndedIterator<Item = &Point> {
        self.vertex_iter_from_ind(start_idx + 1).rev()
    }

    pub fn line_iter(&self) -> impl Iterator<Item = Line<&Point>> + Clone {
        let last = self.path().last().unwrap();
        let first = self.path().first().unwrap();
        let last_line = Line::<&Point>::from_points(last, first).unwrap();
        self.path().line_iter().chain(iter::once(last_line))
    }

    pub fn area(&self) -> i32 {
        let vertices = self.path_.points();
        let x_s = || vertices.iter().map(|point| point.x);
        let y_s = || vertices.iter().map(|point| point.y);

        let positives: i32 = x_s().zip(y_s().cycle().skip(1)).map(|(x, y)| x * y).sum();
        let negatives: i32 = y_s().zip(x_s().cycle().skip(1)).map(|(x, y)| x * y).sum();

        (positives - negatives).abs() / 2
    }

    pub fn insertion_point(&self, point: &Point) -> Option<usize> {
        let path_len = self.path().points().len();
        insertion_point(self.line_iter(), point).map(|ind| ind % path_len)
    }

    pub fn is_on_edge(&self, point: &Point) -> bool {
        self.insertion_point(point).is_some()
    }

    pub fn is_inside(&self, point: &Point) -> bool {
        // Non-zero rule.
        // First we need to return false if the point is on an edge. We use
        // `itertools::process_results` to short-circuit.
        let not_on_edge = self.line_iter().map(|line| {
            if !line.contains(point) {
                Ok(line)
            } else {
                Err(())
            }
        });

        let sum_result: Result<i32, ()> = itertools::process_results(not_on_edge, |iter| {
            // We select the lines that intersect the half line cast from the point.
            let intersecting_line_sides = iter
                .filter(|line| line.intersects_half_line(point, Direction::RIGHT))
                .map(|line| line.point_on_side(point));

            let reduced = Self::reduce_sequences(intersecting_line_sides);

            // TODO: Use iterators.
            let values = reduced.collect::<Vec<_>>();
            let sum: i32 = values.iter().sum();

            // Check wrapping. At most two elements of a sequence wrap around to the start, and at
            // most two are at the end. Of course, it is either 1+2 or 2+1 (or none). We put the
            // first two elements after the last two an check if there is a sequence. It shouldn't
            // matter if we duplicate some elements (for example if there are three elements).
            let end_idx = 2.min(values.len());
            let first_two = &values[0..end_idx];

            let start_idx = if values.len() > 2 {
                values.len() - 1
            } else {
                0
            };
            let last_two = &values[start_idx..];

            let together: Vec<i32> = last_two.iter().chain(first_two).copied().collect();

            let to_subtract = match &together[..] {
                &[v, 0, w, ..] | &[.., v, 0, w] if v == w => v,
                _ => 0,
            };

            sum - to_subtract
        });

        if let Ok(sum) = sum_result {
            sum != 0
        } else {
            false
        }
    }

    pub fn intersections_with_line<'a, 'b, 'c, PointT, LineT>(
        &'a self,
        line: LineT,
    ) -> impl Iterator<Item = LineIntersection> + 'c
    where
        PointT: Borrow<Point> + 'b,
        LineT: Borrow<Line<PointT>> + 'b,
        'a: 'c,
        'b: 'c,
    {
        line::intersections_line_iters(self.line_iter(), std::iter::once(line))
    }

    pub fn intersections_with_path<'a, 'b, 'c>(
        &'a self,
        path: &'b Path,
    ) -> impl Iterator<Item = LineIntersection> + 'c
    where
        'a: 'c,
        'b: 'c,
    {
        line::intersections_line_iters(self.line_iter(), path.line_iter())
    }

    pub fn intersects_line<PointT>(&self, line: &Line<PointT>) -> bool
    where
        PointT: Borrow<Point>,
    {
        // Iterator has an element.
        self.intersections_with_line(line).any(|_| true)
    }

    pub fn intersects_path(&self, path: &Path) -> bool {
        // Iterator has an element.
        self.intersections_with_path(path).any(|_| true)
    }

    pub fn cut(&self, path: &Path) -> Option<(Polygon, Polygon)> {
        if path.points().len() < 2 || !self.check_path_inside(path) {
            return None;
        }

        let path_start = &path.first()?;
        let path_end = &path.last()?;
        let start_insertion_idx = self.insertion_point(path_start)?;
        let end_insertion_idx = self.insertion_point(path_end)?;

        if path.points().len() == 2
            && !self.check_two_point_path_line_outside(path, start_insertion_idx)
        {
            return None;
        }

        let (insertion_start, insertion_end, path_points): (usize, usize, Vec<Point>) = self
            .cut_path_insertion_and_direction(
                path,
                path_start,
                path_end,
                start_insertion_idx,
                end_insertion_idx,
            )?;

        let orig_points = self.path().points();

        let points1 = orig_points[..insertion_start]
            .iter()
            .chain(&path_points)
            .chain(&orig_points[insertion_end..])
            .copied();
        let path1 = Path::with_points(points1).unwrap();
        let poly1 = Polygon::with_path(path1).ok()?; // Error if too few vertices.

        let points2 = path_points
            .iter()
            .rev()
            .chain(&orig_points[insertion_start..insertion_end])
            .copied();
        let path2 = Path::with_points(points2).unwrap();
        let poly2 = Polygon::with_path(path2).ok()?; // Error if too few vertices.

        Some((poly1, poly2))
    }

    fn reduce_sequences<I: Iterator<Item = i32>>(iter: I) -> impl Iterator<Item = i32> {
        // We reduce sequences of [1, 0, 1] to 1 and sequences of [-1, 0, -1] to -1. These
        // occur when the half line intersects two lines in the same direction connected by a
        // line that is perpendicular to the half line. In this case counting the lines
        // separately would give incorrect results.
        // Note that this does not reduce sequences recursively: [1, 0, 1, 0, 1] becomes [1, 0, 1]
        // and not [1]. Sequences like this should not occur with the polygon's edges.
        let mut lookahead = VecDeque::with_capacity(3);
        let mut iterator_exhausted = false;
        let filtered = iter.batching(move |it| {
            // Get the next group of three if possible.
            while lookahead.len() < 3 && !iterator_exhausted {
                if let Some(value) = it.next() {
                    lookahead.push_back(value);
                } else {
                    iterator_exhausted = true;
                }
            }

            // If we do not have three elements, we are at the end and return everything.
            if lookahead.len() < 3 {
                return lookahead.pop_front();
            }

            let first = lookahead[0];
            if lookahead[1] == 0 && lookahead[0] == lookahead[2] {
                lookahead.clear();
            } else {
                lookahead.pop_front();
            }

            Some(first)
        });

        filtered
    }

    fn cut_path_insertion_and_direction(
        &self,
        path: &Path,
        path_start: &Point,
        path_end: &Point,
        start_insertion_idx: usize,
        end_insertion_idx: usize,
    ) -> Option<(usize, usize, Vec<Point>)> {
        let path_points = path.points().to_vec();

        let reverse = {
            if start_insertion_idx < end_insertion_idx {
                false
            } else if start_insertion_idx > end_insertion_idx {
                true
            } else {
                debug_assert!(start_insertion_idx == end_insertion_idx);

                let vertices = self.path().points();
                let edge_start_idx = if start_insertion_idx == 0 {
                    vertices.len() - 1
                } else {
                    start_insertion_idx - 1
                };
                let edge_end_idx = start_insertion_idx;
                Polygon::should_reverse_cut_path_same_insertion_points(
                    &vertices[edge_start_idx],
                    &vertices[edge_end_idx],
                    path_start,
                    path_end,
                )
            }
        };

        if !reverse {
            Some((start_insertion_idx, end_insertion_idx, path_points))
        } else {
            Some((
                end_insertion_idx,
                start_insertion_idx,
                path_points.iter().rev().copied().collect(),
            ))
        }
    }

    fn should_reverse_cut_path_same_insertion_points(
        vertex_start: &Point,
        vertex_end: &Point,
        cut_start: &Point,
        cut_end: &Point,
    ) -> bool {
        let line = Line::from_points(vertex_start, vertex_end).unwrap();
        debug_assert!(line.collinear(cut_start));
        debug_assert!(line.collinear(cut_end));

        if vertex_start.x == vertex_end.x {
            (vertex_start.y - cut_start.y).abs() > (vertex_start.y - cut_end.y).abs()
        } else {
            debug_assert!(vertex_start.y == vertex_end.y);
            (vertex_start.x - cut_start.x).abs() > (vertex_start.x - cut_end.x).abs()
        }
    }

    fn check_path_inside(&self, path: &Path) -> bool {
        self.check_points_inside(path) && self.check_path_does_not_intersect_polygon(path)
    }

    fn check_points_inside(&self, path: &Path) -> bool {
        let points = path.points();
        let non_end_points = &points[1..points.len() - 1];
        non_end_points.iter().all(|p| self.is_inside(p))
    }

    fn check_path_does_not_intersect_polygon(&self, path: &Path) -> bool {
        // We expect two intersections, at the beginning and at the end.
        let inner_path = path.line_iter().skip(1).skip_last();
        let intersection_found =
            line::intersections_line_iters(self.line_iter(), inner_path).any(|_| true);
        !intersection_found
    }

    fn check_two_point_path_line_outside(
        &self,
        path: &Path,
        path_start_insertion_point: usize,
    ) -> bool {
        // Check for the case when the path only has a starting point and an endpoint, both on the
        // polygon's edges, and no other points but the line is outside.
        // We don't need to check for cases where there are more than two points as
        // `check_points_inside' and `check_path_does_not_intersect_polygon' cover those cases.
        debug_assert!(path.points().len() == 2);

        let path_end = path.points().last().unwrap();
        let mut left_or_right = self
            .line_iter()
            .nth(path_start_insertion_point - 1)
            .unwrap()
            .point_on_side(path_end);

        // We're at a vertex, we need the next edge that is not collinear with the path.
        if left_or_right == 0 {
            left_or_right = self
                .line_iter()
                .nth(path_start_insertion_point)
                .unwrap()
                .point_on_side(path_end);
        }

        let right = -1;
        let left = 1;
        let ok: bool = if self.is_clockwise() {
            left_or_right == right
        } else {
            left_or_right == left
        };

        ok
    }

    // TODO: This could be cached.
    fn is_clockwise(&self) -> bool {
        // Find the top left vertex, it is part of the convex hull.
        // See https://en.wikipedia.org/wiki/Curve_orientation#Orientation_of_a_simple_polygon.
        let cmp = |p1: &&Point, p2: &&Point| -> Ordering { p1.x.cmp(&p2.x).then(p1.y.cmp(&p2.y)) };

        let min_ind_pt_opt: Option<(usize, &Point)> = self
            .vertex_iter_from_ind(0)
            .enumerate()
            .min_by(|(_ind1, pt1), (_ind2, pt2)| cmp(pt1, pt2));

        if let Some((ind, pt)) = min_ind_pt_opt {
            // We can unwrap them because we have always at least four vertices.
            let pt_before: &Point = self.vertex_iter_from_ind_backwards(ind).nth(1).unwrap();
            let pt_after: &Point = self.vertex_iter_from_ind(ind).nth(1).unwrap();

            let line = Line::from_points(pt_before, pt).unwrap();

            // Negative determinant means clockwise orientation.
            line.point_on_side(pt_after) < 0
        } else {
            // We have no vertices, that can be regarded as clockwise or anti-clockwise.
            true
        }
    }
}

impl PartialEq for Polygon {
    fn eq(&self, other: &Polygon) -> bool {
        let self_points = self.path().points();
        let other_points = other.path().points();

        if self_points.len() != other_points.len() {
            return false;
        }

        if self_points.is_empty() {
            // We are empty and the sizes match so `other' must be empty too.
            return true;
        }

        let self_first = self_points.get(0).unwrap();

        let offset_opt = other_points.iter().position(|p| p == self_first);
        if offset_opt.is_none() {
            return false;
        }

        let offset = offset_opt.unwrap();

        is_eq_forward(self_points, other_points, offset)
            || is_eq_backward(self_points, other_points, offset)
    }
}

fn is_eq_forward(vertices1: &[Point], vertices2: &[Point], offset: usize) -> bool {
    debug_assert_eq!(vertices1.len(), vertices2.len());

    let other_iter = vertices2.iter().cycle().skip(offset).take(vertices2.len());
    vertices1.iter().eq(other_iter)
}

fn is_eq_backward(vertices1: &[Point], vertices2: &[Point], offset: usize) -> bool {
    debug_assert_eq!(vertices1.len(), vertices2.len());

    let other_iter = vertices2
        .iter()
        .rev()
        .cycle()
        .skip(vertices2.len() - offset - 1)
        .take(vertices2.len());

    vertices1.iter().eq(other_iter)
}

#[cfg(test)]
mod tests;

#[cfg(test)]
mod tests_fractal_polygon_inside;
