use super::point::{Direction, Point};

#[derive(Debug)]
pub struct Field {
    width_: i32,
    height_: i32,
    free_polygon_: Vec<Point>,
}

impl Field {
    pub fn new(width: i32, height: i32) -> Field {
        let points = vec![
            Point::new(0, 0),
            Point::new(width, 0),
            Point::new(width, height),
            Point::new(0, height),
        ];

        Field {
            width_: width,
            height_: height,
            free_polygon_: points,
        }
    }

    pub fn width(&self) -> i32 {
        self.width_
    }

    pub fn height(&self) -> i32 {
        self.height_
    }

    pub fn free_polygon(&self) -> &[Point] {
        &self.free_polygon_
    }

    pub fn free_area(&self) -> i32 {
        let total = self.width_ * self.height_;
        let free = shoelace_poly_area(&self.free_polygon_);
        total - free
    }
}

fn shoelace_poly_area(vertices: &[Point]) -> i32 {
    let x_s = || vertices.iter().map(|point| point.x);
    let y_s = || vertices.iter().map(|point| point.y);

    let positives: i32 = x_s().zip(y_s().cycle().skip(1)).map(|(x, y)| x * y).sum();
    let negatives: i32 = y_s().zip(x_s().cycle().skip(1)).map(|(x, y)| x * y).sum();

    (positives - negatives).abs() / 2
}

#[cfg(test)]
mod tests;
