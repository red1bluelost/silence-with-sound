mod audio_tab;
mod sws_gui;

use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{gio, glib};

glib::wrapper! {
    pub struct SWSGui(ObjectSubclass<sws_gui::SWSGui>)
    @extends gtk::Widget,
    @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl SWSGui {
    fn new() -> Self {
        glib::Object::builder().build()
    }

    delegate::delegate! {
        to self.imp() {
            fn register_parent(&self, parent: gtk::ApplicationWindow);
        }
    }
}

glib::wrapper! {
    pub struct AudioTab(ObjectSubclass<audio_tab::AudioTab>)
    @extends gtk::Widget,
    @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl AudioTab {
    fn new(file: gio::File) -> Self {
        let audio_tab: Self = glib::Object::builder().build();
        audio_tab.imp().register_file(file);
        audio_tab
    }

    delegate::delegate! {
        to self.imp() {
            fn kill_audio(&self);
        }
    }
}

pub fn main() -> glib::ExitCode {
    let app = gtk::Application::builder()
        .application_id("org.r1bl.SoundWithSilence")
        .build();

    app.connect_activate(|app| {
        // let m: gdk::Monitor = gdk::Display::default()
        //     .unwrap()
        //     .monitors()
        //     .into_iter()
        //     .next()
        //     .unwrap()
        //     .unwrap()
        //     .downcast()
        //     .unwrap();
        // let g = m.geometry();

        let sws_gui = SWSGui::new();

        let window = gtk::ApplicationWindow::builder()
            .application(app)
            .resizable(true)
            .title("Sound with Silence")
            .child(&sws_gui)
            .build();

        sws_gui.register_parent(window.clone());

        window.present();
    });

    app.run()
}
