use cairo;
use gtk::prelude::*;
use gtk::DrawingArea;

use super::model::field::Field;
use super::model::game::Game;
use super::model::snake::Snake;
use super::model::spider::Spider;

pub trait Drawable {
    fn draw(&self, drawing_area: &DrawingArea, cx: &cairo::Context);
    fn draw_restore(&self, drawing_area: &DrawingArea, cx: &cairo::Context) {
        cx.save();
        self.draw(drawing_area, cx);
        cx.restore();
    }
}

impl Drawable for Field {
    fn draw(&self, _drawing_area: &DrawingArea, cx: &cairo::Context) {
        cx.set_source_rgb(1.0, 1.0, 1.0);
        cx.paint();
        cx.set_source_rgb(0.0, 0.0, 0.0);
        cx.set_line_width(0.4);
        cx.rectangle(0.0, 0.0, self.width() as f64, self.height() as f64);
        cx.stroke();
    }
}

impl Drawable for Spider {
    fn draw(&self, _drawing_area: &DrawingArea, cx: &cairo::Context) {
        cx.set_source_rgb(0.0, 0.0, 1.0);
        let spider_pos = self.pos();
        draw_dot(cx, (spider_pos.x as f64, spider_pos.y as f64), 0.2);
        cx.fill();
    }
}

impl Drawable for Snake {
    fn draw(&self, _drawing_area: &DrawingArea, cx: &cairo::Context) {
        cx.set_source_rgb(1.0, 0.0, 0.0);
        let spider_pos = self.pos();
        draw_dot(cx, (spider_pos.x as f64, spider_pos.y as f64), 0.2);
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
