use gtk::prelude::*;
use gtk::{
    glib::ExitCode, Application, ApplicationWindow, Button, Orientation,
};

use std::cell::RefCell;

pub fn main() -> ExitCode {
    let app = Application::builder()
        .application_id("org.r1bl.SoundWithSilence")
        .build();

    app.connect_activate(|app| {
        let button = Button::builder()
            .label("Press Me!")
            .margin_top(12)
            .margin_bottom(12)
            .margin_start(12)
            .margin_end(12)
            .build();
        let values = RefCell::new(
            [
                "Lamo!",
                "Your Mom!",
                "Hi :3",
                "Rehab was supposed to be a fresh start",
            ]
            .into_iter()
            .cycle(),
        );
        button.connect_clicked(move |button| {
            let value =
                values.borrow_mut().next().unwrap_or_else(|| unreachable!());
            button.set_label(value)
        });

        let elm_box = gtk::Box::builder()
            .orientation(Orientation::Vertical)
            .build();
        elm_box.append(&button);

        let window = ApplicationWindow::builder()
            .application(app)
            .resizable(true)
            .default_height(500)
            .default_width(500)
            .title("Hello, World!")
            .child(&elm_box)
            .build();

        window.present();
    });

    app.run()
}
