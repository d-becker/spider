use std::borrow::Borrow;
use std::iter;

use super::point::{Direction, Point};

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

#[derive(Debug, Eq, PartialEq)]
pub struct Line<PointT>
where
    PointT: Borrow<Point>,
{
    start: PointT,
    end: PointT,
}

impl<PointT: Borrow<Point>> Line<PointT> {
    pub fn from_points(start: PointT, end: PointT) -> Option<Line<PointT>> {
        if rectilinear(start.borrow(), end.borrow()) {
            Some(Line { start, end })
        } else {
            None
        }
    }

    pub fn start(&self) -> &PointT {
        &self.start
    }

    pub fn end(&self) -> &PointT {
        &self.end
    }

    pub fn vertical(&self) -> bool {
        self.start.borrow().x == self.end.borrow().x
    }

    pub fn horizontal(&self) -> bool {
        self.start.borrow().y == self.end.borrow().y
    }

    pub fn direction(&self) -> Direction {
        let diff = self.end.borrow().subtract(self.start.borrow());

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
        let p1 = self.start().borrow();
        let p2 = self.start().borrow();
        let p3 = p.borrow();

        vertical_collinear(p1, p2, p3) || horizontal_collinear(p1, p2, p3)
    }

    pub fn contains<PointT2>(&self, p: PointT2) -> bool
    where
        PointT2: Borrow<Point>,
    {
        let start = self.start().borrow();
        let end = self.end().borrow();
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
        let l1_h = [self.start().borrow().x, self.end().borrow().x];
        let l2_h = [other.start().borrow().x, other.end().borrow().x];

        let l1_v = [self.start().borrow().y, self.end().borrow().y];
        let l2_v = [other.start().borrow().y, other.end().borrow().y];

        intervals_overlap(l1_h, l2_h) && intervals_overlap(l1_v, l2_v)
    }

    pub fn intersects_half_line(
        &self,
        half_line_point: &Point,
        half_line_dir: Direction,
    ) -> bool
    {
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
        self.intersects(&half_line)
    }
}

fn intervals_overlap(mut int1: [i32; 2], mut int2: [i32; 2]) -> bool {
    int1.sort();
    int2.sort();

    (int1[0] <= int2[0] && int2[0] <= int1[1]) || (int1[0] <= int2[1] && int2[0] <= int1[1])
}

fn vertical_collinear(p1: &Point, p2: &Point, p3: &Point) -> bool {
    p1.x == p2.x && p2.x == p3.x
}

fn horizontal_collinear(p1: &Point, p2: &Point, p3: &Point) -> bool {
    p1.y == p2.y && p2.y == p3.y
}

fn get_range(
    start: i32,
    start_incl: bool,
    end: i32,
    end_incl: bool,
) -> Box<dyn Iterator<Item = i32>> {
    if start <= end {
        Box::new(start + !start_incl as i32..=end - !end_incl as i32)
    } else {
        Box::new((end + !end_incl as i32..=start - !start_incl as i32).rev())
    }
}

fn generate_rectilinear_path(
    p1: &Point,
    p1_incl: bool,
    p2: &Point,
    p2_incl: bool,
) -> Option<Box<dyn Iterator<Item = Point>>> {
    if p1.x == p2.x {
        let common_x = p1.x;
        Some(Box::new(
            get_range(p1.y, p1_incl, p2.y, p2_incl).map(move |y| Point::new(common_x, y)),
        ))
    } else if p1.y == p2.y {
        let common_y = p1.y;
        Some(Box::new(
            get_range(p1.x, p1_incl, p2.x, p2_incl).map(move |x| Point::new(x, common_y)),
        ))
    } else {
        None
    }
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

    pub fn with_points(points: &[Point]) -> Option<Path> {
        let mut path = Path::new();

        for point in points {
            path.add(*point).ok()?;
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

    pub fn line_iter(&self) -> impl Iterator<Item = Line<&Point>> {
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

            // TODO: Now we generate every point between the last and the new point to handle path
            // intersections. We could use a more efficient approach.
            if let Some(it) = generate_rectilinear_path(last_point, false, &point, true) {
                for intermediary_point in it {
                    self.handle_loop(&intermediary_point);
                    self.handle_collinearity(&intermediary_point);

                    self.points_.push(intermediary_point);
                }
            } else {
                return Result::Err("Not rectilinear.");
            }
        } else {
            self.points_.push(point);
        }
        Result::Ok(())
    }

    pub fn insertion_point(&self, point: &Point) -> Option<usize> {
        for (i, line) in self.line_iter().enumerate() {
            if line.contains(point) {
                return Some(i + 1);
            }
        }

        None
    }

    pub fn contains(&self, point: &Point) -> bool {
        self.insertion_point(point).is_some()
    }

    fn handle_loop(&mut self, new_pos: &Point) {
        if let Some(i) = self.insertion_point(new_pos) {
            self.points_.truncate(i);
        }
    }

    fn handle_collinearity(&mut self, new_pos: &Point) {
        // Is .last() inefficient.
        if let Some(line) = self.line_iter().last() {
            if line.collinear(new_pos) {
                self.points_.pop();
            }
        }
    }
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
    pub fn with_path(path: Path) -> Result<Polygon, PolygonError> {
        match &path.points() {
            &[] => Ok(Polygon { path_: path }),
            &[_] => Err(PolygonError::NotEnoughVertices),
            &[_, _] => Err(PolygonError::NotEnoughVertices),
            &[_, _, _] => Err(PolygonError::NotEnoughVertices),
            &[first, _, _, .., last] => {
                // TODO: Find more efficient way than checking all points on the path.
                if let Some(mut iter) = generate_rectilinear_path(last, false, first, false) {
                    if iter.any(|point| path.contains(&point)) {
                        Err(PolygonError::SelfIntersecting)
                    } else {
                        Ok(Polygon { path_: path })
                    }
                } else {
                    Err(PolygonError::NonRectilinear)
                }
            }
        }
    }

    pub fn path(&self) -> &Path {
        &self.path_
    }

    pub fn vertex_iter_from_ind<'a>(
        &'a self,
        start_idx: usize,
    ) -> impl DoubleEndedIterator<Item = Point> + 'a {
        RotatedIterator::new(self.path().points(), start_idx)
    }

    pub fn vertex_iter_from_ind_backwards<'a>(
        &'a self,
        start_idx: usize,
    ) -> impl DoubleEndedIterator<Item = Point> + 'a {
        self.vertex_iter_from_ind(start_idx + 1).rev()
    }

    pub fn line_iter(&self) -> impl Iterator<Item = Line<&Point>> {
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
        let in_path = self.path_.insertion_point(point);

        if in_path.is_some() {
            return in_path;
        }

        if let &[first, .., last] = self.path_.points() {
            // TODO: Use line segment iterator?
            let line = Line::from_points(last, first).unwrap();
            if line.contains(point) {
                return Some(0);
            }
        } else {
            // We always have at least four vertices.
            unreachable!();
        }

        None
    }

    pub fn is_on_edge(&self, point: &Point) -> bool {
        self.insertion_point(point).is_some()
    }

    pub fn is_inside(&self, point: &Point) -> bool {
        let mut count = 0;
        for line in self.line_iter() {
            if line.contains(point) {
                return false;
            }

            if line.intersects_half_line(point, Direction::RIGHT) {
                count += 1;
            }
        }

        count % 2 == 1
    }

    pub fn cut(&self, path: &Path) -> Option<(Polygon, Polygon)> {
        if !self.check_path_inside(path) {
            return None;
        }

        let (insertion_start, insertion_end, path_points): (usize, usize, Vec<Point>) =
            self.cut_path_insertion_and_direction(path)?;

        let orig_points = self.path().points();

        let points1: Vec<Point> = orig_points[..insertion_start]
            .iter()
            .chain(&path_points)
            .chain(&orig_points[insertion_end..])
            .map(|&p| p)
            .collect();
        let path1 = Path::with_points(&points1).unwrap();
        let poly1 = Polygon::with_path(path1).unwrap();

        let points2: Vec<Point> = path_points
            .iter()
            .rev()
            .chain(&orig_points[insertion_start..insertion_end])
            .map(|&p| p)
            .collect();
        let path2 = Path::with_points(&points2).unwrap();
        let poly2 = Polygon::with_path(path2).unwrap();

        Some((poly1, poly2))
    }

    fn cut_path_insertion_and_direction(&self, path: &Path) -> Option<(usize, usize, Vec<Point>)> {
        let path_start = &path.first()?;
        let path_end = &path.last()?;
        let start_insertion_idx = self.insertion_point(path_start)?;
        let end_insertion_idx = self.insertion_point(path_end)?;
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
                path_points.iter().rev().map(|&p| p).collect(),
            ))
        }
    }

    fn should_reverse_cut_path_same_insertion_points(
        vertex_start: &Point,
        vertex_end: &Point,
        cut_start: &Point,
        cut_end: &Point,
    ) -> bool {
        // TODO: Line as an argument?
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
        let points = path.points();
        let non_end_points = &points[1..points.len() - 1];
        non_end_points.iter().all(|p| self.is_inside(p))
    }
}

impl PartialEq for Polygon {
    // TODO: Handle possible collinear vertices between the last and the first.
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
    let mut vertex_pairs = vertices1.iter().zip(other_iter);

    vertex_pairs.all(|(v1, v2)| v1 == v2)
}

fn is_eq_backward(vertices1: &[Point], vertices2: &[Point], offset: usize) -> bool {
    debug_assert_eq!(vertices1.len(), vertices2.len());

    let other_iter = vertices2
        .iter()
        .rev()
        .cycle()
        .skip(vertices2.len() - offset - 1)
        .take(vertices2.len());
    let mut vertex_pairs = vertices1.iter().zip(other_iter);

    vertex_pairs.all(|(v1, v2)| v1 == v2)
}

struct RotatedIterator<'a> {
    slice: &'a [Point],
    idx_fwd: usize,
    idx_back: usize,
    started_iteration: bool,
}

impl<'a> RotatedIterator<'a> {
    pub fn new(slice: &'a [Point], start_idx: usize) -> RotatedIterator {
        let start_idx = start_idx % slice.len();
        RotatedIterator {
            slice: slice,
            idx_fwd: start_idx,
            idx_back: start_idx,
            started_iteration: false,
        }
    }

    fn increase_idx(&self, ind: usize) -> usize {
        (ind + 1) % self.slice.len()
    }

    fn decrease_idx(&self, ind: usize) -> usize {
        let mut res = ind;
        if ind == 0 {
            res += self.slice.len();
        }

        res - 1
    }
}

impl<'a> Iterator for RotatedIterator<'a> {
    type Item = Point;

    fn next(&mut self) -> Option<Point> {
        if self.idx_fwd == self.idx_back && self.started_iteration {
            None
        } else {
            let res = self.slice.get(self.idx_fwd);
            self.idx_fwd = self.increase_idx(self.idx_fwd);
            self.started_iteration = true;

            res.map(|&p| p)
        }
    }
}

impl<'a> DoubleEndedIterator for RotatedIterator<'a> {
    fn next_back(&mut self) -> Option<Point> {
        if self.idx_fwd == self.idx_back && self.started_iteration {
            None
        } else {
            self.idx_back = self.decrease_idx(self.idx_back);
            let res = self.slice.get(self.idx_back);
            self.started_iteration = true;

            res.map(|&p| p)
        }
    }
}

#[cfg(test)]
mod tests;
