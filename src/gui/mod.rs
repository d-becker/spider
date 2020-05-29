pub mod draw;
pub mod router;

use std::borrow::Borrow;

use cairo::Context;
use gio::prelude::*;
use gtk::prelude::*;
use gtk::DrawingArea;

use super::model;
use super::ImmutableRcWrapper;

use draw::Drawable;

pub fn start_gui(game: ImmutableRcWrapper<model::game::Game>) {
    let app = gtk::Application::new(None, Default::default()).expect("Initialization failed...");

    app.connect_activate(move |app| on_activate(app, game.clone()));
    app.run(&std::env::args().collect::<Vec<_>>());
}

fn draw_game(
    drawing_area: &DrawingArea,
    cx: &Context,
    game: std::cell::Ref<model::game::Game>,
) -> Inhibit {
    game.borrow().draw_restore(drawing_area, cx);
    Inhibit(false)
}

fn handle_key_press(key_event: &gdk::EventKey) {
    let str_ = match key_event.get_keyval() {
        gdk::enums::key::Down => "down",
        gdk::enums::key::Up => "up",
        gdk::enums::key::Left => "left",
        gdk::enums::key::Right => "right",
        gdk::enums::key::space => "space",
        _ => "unknown",
    };

    println!("Key {} pressed.", str_);
}

fn on_activate(app: &gtk::Application, game: ImmutableRcWrapper<model::game::Game>) {
    let window = gtk::ApplicationWindow::new(app);
    let drawing_area = DrawingArea::new();

    window.add_events(gdk::EventMask::KEY_PRESS_MASK);
    window.connect_key_press_event(move |_, ev| {
        handle_key_press(ev);
        Inhibit(false)
    });
    drawing_area.connect_draw(move |dr, cx| draw_game(dr, cx, game.borrow()));
    window.add(&drawing_area);

    gtk::timeout_add_seconds(1, move || {
        let width = drawing_area.get_allocated_width();
        let height = drawing_area.get_allocated_height();
        drawing_area.clone().queue_draw_area(0, 0, width, height);
        Continue(true)
    });
    window.show_all();
}
