use super::point::Point;
use super::rectilinear;

#[derive(Debug)]
pub struct Field {
    width_: i32,
    height_: i32,
    free_polygon_: rectilinear::Polygon,
    cut_polygons_: Vec<rectilinear::Polygon>,
}

impl Field {
    pub fn new(width: i32, height: i32) -> Field {
        let points = [
            Point::new(0, 0),
            Point::new(width, 0),
            Point::new(width, height),
            Point::new(0, height),
        ];
        let path = rectilinear::Path::with_points(points.iter())
            .expect("Should never happen, the points are vertices of a square.");
        let poly = rectilinear::Polygon::with_path(path)
            .expect("Should never happen, the points are vertices of a square.");

        Field {
            width_: width,
            height_: height,
            free_polygon_: poly,
            cut_polygons_: Vec::new(),
        }
    }

    pub fn width(&self) -> i32 {
        self.width_
    }

    pub fn height(&self) -> i32 {
        self.height_
    }

    pub fn free_polygon(&self) -> &rectilinear::Polygon {
        &self.free_polygon_
    }

    pub fn cut_polygons(&self) -> &[rectilinear::Polygon] {
        &self.cut_polygons_
    }

    pub fn cut(
        &mut self,
        new_free_polygon: rectilinear::Polygon,
        cut_polygon: rectilinear::Polygon,
    ) {
        self.free_polygon_ = new_free_polygon;
        self.cut_polygons_.push(cut_polygon);
    }

    pub fn free_area(&self) -> i32 {
        self.free_polygon_.area()
    }
}

#[cfg(test)]
mod tests;
