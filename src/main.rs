use std::rc::Rc;
use std::cell::RefCell;

use gio::prelude::*;
use glib::clone;
use gtk::prelude::*;

use spider_core;
use spider_core::gui;

use spider_core::ImmutableRcWrapper;

use spider_core::model::point::{Direction, Point};
use spider_core::model::field::Field;
use spider_core::model::snake::Snake;
use spider_core::model::spider::Spider;
use spider_core::model::game::Game;

fn main() {
    let field = Field::new(50, 20);
    let spider = Spider::new(Point::new(0, 0), Direction::RIGHT, Point::new(0, 0), Point::new(50, 20));
    let snake = Snake::new(Point::new(10, 10));

    let game = Game::new(field, spider, snake);
    let game_rc = Rc::new(RefCell::new(game));

    gtk::init().unwrap();
    gtk::timeout_add_seconds(1, clone!(@strong game_rc => move || {
        game_rc.borrow_mut().update_state();
        Continue(true)
    }));

    gui::start_gui(ImmutableRcWrapper::from_rc(game_rc.clone()));
}
