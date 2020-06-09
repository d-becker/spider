pub mod draw;
pub mod router;

use std::rc::Rc;

use gio::prelude::*;
use gtk::prelude::*;
use gtk::DrawingArea;

use super::model;

pub type DrawCallback = Rc<dyn Fn(&DrawingArea, &cairo::Context) -> Inhibit>;
pub type KeyEventCallback = Rc<dyn Fn(&gdk::EventKey) -> Inhibit>;
pub struct SpiderGui {
    app: gtk::Application,
    on_draw: DrawCallback,
    key_event_callback: KeyEventCallback,
}

impl SpiderGui {
    pub fn new(on_draw: DrawCallback, key_event_callback: KeyEventCallback) -> SpiderGui {
        let app =
            gtk::Application::new(None, Default::default()).expect("Initialization failed...");

        let spider_gui = SpiderGui {
            app,
            on_draw,
            key_event_callback,
        };
        spider_gui
    }

    pub fn run(self) {
        let app = self.app;
        let on_draw = self.on_draw;
        let key_event_callback = self.key_event_callback;
        app.connect_activate(move |application| {
            Self::on_activate(application, on_draw.clone(), key_event_callback.clone())
        });
        app.run(&std::env::args().collect::<Vec<_>>());
    }

    fn on_activate(
        app: &gtk::Application,
        draw_callback: DrawCallback,
        key_event_callback: KeyEventCallback,
    ) {
        let window = gtk::ApplicationWindow::new(app);
        let drawing_area = DrawingArea::new();

        window.add_events(gdk::EventMask::KEY_PRESS_MASK);

        window.connect_key_press_event(move |_, ev| key_event_callback(ev));

        drawing_area.connect_draw(move |dr, cx| draw_callback(dr, cx));
        window.add(&drawing_area);

        // TODO: Make the refresh rate a parameter.
        gtk::timeout_add(500, move || {
            let width = drawing_area.get_allocated_width();
            let height = drawing_area.get_allocated_height();
            drawing_area.clone().queue_draw_area(0, 0, width, height);
            Continue(true)
        });
        window.show_all();
    }
}
