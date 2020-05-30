use std::rc::Rc;
use std::cell::RefCell;

use gio::prelude::*;
use glib::clone;
use gtk::prelude::*;

use spider_core;
use spider_core::gui::{self, SpiderGui};

use spider_core::ImmutableRcWrapper;

use spider_core::model::point::{Direction, Point};
use spider_core::model::field::Field;
use spider_core::model::snake::Snake;
use spider_core::model::spider::Spider;
use spider_core::model::game::Game;

use spider_core::gui::draw::Drawable;
use spider_core::gui::router::{Router, RouterCommand};

fn default_router() -> Router<gdk::enums::key::Key> {
    let mut router = Router::new();

    use gdk::enums::key;
    router.bind(key::Down, RouterCommand::DOWN);
    router.bind(key::Up, RouterCommand::UP);
    router.bind(key::Left, RouterCommand::LEFT);
    router.bind(key::Right, RouterCommand::RIGHT);
    router.bind(key::space, RouterCommand::STOP);
    router.bind(key::p, RouterCommand::PAUSE);

    router
}

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

    let draw_game: gui::DrawCallback = Rc::new(clone!(@strong game_rc => move |drawing_area, cx| {
        game_rc.borrow().draw_restore(drawing_area, cx);
        Inhibit(false)
    }));

    let router = default_router();
    let key_press_handler: gui::KeyEventCallback = Rc::new(clone!(@strong game_rc => move |key_ev| {
        router.route(key_ev.get_keyval(), &mut game_rc.borrow_mut());
        Inhibit(false)
    }));

    let spider_gui = SpiderGui::new(draw_game, key_press_handler);
    spider_gui.run();
}
