use cairo;
use gtk::prelude::*;
use gtk::DrawingArea;

use super::model::field::Field;
use super::model::game::Game;
use super::model::snake::Snake;
use super::model::spider::Spider;
use super::model::rectilinear::Path;
use super::model::rectilinear::Polygon;

const FIELD_FRAME_WIDTH: f64 = 0.4;
const SPIDER_DOT_SIZE: f64 = 0.2;
const SPIDER_PATH_WIDTH: f64 = 0.1;

pub trait Drawable {
    fn draw(&self, drawing_area: &DrawingArea, cx: &cairo::Context);
    fn draw_restore(&self, drawing_area: &DrawingArea, cx: &cairo::Context) {
        cx.save();
        self.draw(drawing_area, cx);
        cx.restore();
    }
}

impl Drawable for Field {
    fn draw(&self, drawing_area: &DrawingArea, cx: &cairo::Context) {
        cx.set_source_rgb(1.0, 1.0, 1.0);
        cx.paint();

        for polygon in self.cut_polygons() {
            polygon.draw_restore(drawing_area, cx);
        }

        cx.set_source_rgb(0.0, 0.0, 0.0);
        cx.set_line_width(FIELD_FRAME_WIDTH);
        cx.rectangle(0.0, 0.0, self.width() as f64, self.height() as f64);
        cx.stroke();
    }
}

impl Drawable for Spider {
    fn draw(&self, drawing_area: &DrawingArea, cx: &cairo::Context) {
        cx.set_source_rgb(0.0, 0.0, 1.0);
        let spider_pos = self.pos();
        draw_dot(cx, (spider_pos.x as f64, spider_pos.y as f64), SPIDER_DOT_SIZE);

        if let Some(path) = self.path() {
            path.draw_restore(drawing_area, cx);
        }
    }
}

impl Drawable for Path {
    fn draw(&self, _drawing_area: &DrawingArea, cx: &cairo::Context) {
        cx.set_source_rgb(0.0, 0.0, 0.0);
        cx.set_line_width(SPIDER_PATH_WIDTH);

        let points = self.points();
        if let Some(first_point) = points.first() {
            cx.move_to(first_point.x as f64, first_point.y as f64);
        }

        for point in &points[1..] {
            cx.line_to(point.x as f64, point.y as f64);
        }

        cx.stroke();
    }
}

impl Drawable for Polygon {
    fn draw(&self, _drawing_area: &DrawingArea, cx: &cairo::Context) {
        let points = self.path().points();
        if let Some(first_point) = points.first() {
            cx.move_to(first_point.x as f64, first_point.y as f64);
        }

        for point in &points[1..] {
            cx.line_to(point.x as f64, point.y as f64);
        }

        cx.set_source_rgb(135.0 / 255.0, 206.0 / 255.0, 250.0 / 255.0);
        cx.fill_preserve();

        cx.set_source_rgb(0.0, 0.0, 0.0);
        cx.set_line_width(SPIDER_PATH_WIDTH);
        cx.close_path();
        cx.stroke();
    }
}

impl Drawable for Snake {
    fn draw(&self, _drawing_area: &DrawingArea, cx: &cairo::Context) {
        cx.set_source_rgb(1.0, 0.0, 0.0);
        let spider_pos = self.pos();
        draw_dot(cx, (spider_pos.x as f64, spider_pos.y as f64), SPIDER_DOT_SIZE);
        cx.fill();
    }
}

impl Drawable for Game {
    fn draw(&self, drawing_area: &DrawingArea, cx: &cairo::Context) {
        let width = drawing_area.get_allocated_width() as f64;
        let height = drawing_area.get_allocated_height() as f64;
        let field_width = self.field().width() as f64;
        let field_height = self.field().height() as f64;
        cx.scale(width / field_width, height / field_height);

        self.field().draw_restore(drawing_area, cx);
        self.spider().draw_restore(drawing_area, cx);
        self.snake().draw_restore(drawing_area, cx);
    }
}

fn draw_dot(cx: &cairo::Context, (x, y): (f64, f64), radius: f64) {
    cx.new_sub_path();
    cx.arc(x, y, radius, 0.0, 2.0 * std::f64::consts::PI);
    cx.fill();
}
