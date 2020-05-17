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

#[derive(Debug)]
pub struct Spider {
    dir: Direction,
    points: Vec<Point>,
}

impl Spider {
    pub fn new(pos: Point, dir: Direction) -> Spider {
        Spider {
            dir,
            points: vec![pos],
        }
    }

    pub fn get_pos(&self) -> Point {
        *self.points.last().unwrap()
    }

    pub fn get_points(&self) -> &[Point] {
        &self.points
    }

    pub fn update(&mut self) {
        if self.dir == Direction::NONE {
            return;
        }

        let pos = self.get_pos();

        let dir_point = self.dir.to_point();
        let new_pos = pos.add(dir_point);

        self.handle_loop(&new_pos);
        self.handle_collinearity(&new_pos);

        self.points.push(new_pos);
    }

    pub fn get_dir(&self) -> Direction {
        self.dir
    }

    pub fn set_dir(&mut self, dir: Direction) {
        self.dir = dir;
    }

    fn handle_loop(&mut self, new_pos: &Point) {
        let mut ind = None;
        for (i, pair) in self.points.windows(2).enumerate() {
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
            self.points.truncate(i + 1);
        }
    }

    fn handle_collinearity(&mut self, new_pos: &Point) {
        let mut should_delete = false;
        if let [.., ref second_last, ref last] = self.points.as_mut_slice() {
            if collinear(second_last, last, new_pos) {
                should_delete = true;
            }
        }

        if should_delete {
            self.points.pop();
        }
    }
}

#[cfg(test)]
mod tests;
