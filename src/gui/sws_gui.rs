use super::AudioTab;

use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{gio, glib};

use std::cell::RefCell;

#[derive(Debug)]
pub(super) struct NewAudioTab {
    pub(super) container: gtk::FlowBox,
    pub(super) file_find_button: gtk::Button,
}

#[derive(Debug)]
pub struct SWSGui {
    pub(super) notebook: gtk::Notebook,
    pub(super) new_audio_tab: NewAudioTab,
    pub(super) audio_tabs: RefCell<Vec<AudioTab>>,
}

impl Default for SWSGui {
    fn default() -> Self {
        let container = gtk::FlowBox::builder().build();

        let file_find_button =
            gtk::Button::builder().label("Choose Audio File").build();
        container.append(&file_find_button);

        let new_audio_tab = NewAudioTab {
            container,
            file_find_button,
        };

        let notebook = gtk::Notebook::builder()
            .tab_pos(gtk::PositionType::Top)
            .scrollable(true)
            .show_tabs(true)
            .show_border(true)
            .group_name("audio")
            .build();

        Self {
            notebook,
            new_audio_tab,
            audio_tabs: Default::default(),
        }
    }
}

#[glib::object_subclass]
impl ObjectSubclass for SWSGui {
    const NAME: &'static str = "R1BLSWSGui";
    type Type = super::SWSGui;
    type ParentType = gtk::Widget;
}

impl ObjectImpl for SWSGui {
    fn constructed(&self) {
        self.notebook.set_parent(&*self.obj());
    }
    fn dispose(&self) {
        self.notebook.unparent();
    }
}

impl WidgetImpl for SWSGui {
    fn measure(
        &self,
        orientation: gtk::Orientation,
        for_size: i32,
    ) -> (i32, i32, i32, i32) {
        self.notebook.measure(orientation, for_size)
    }

    fn size_allocate(&self, width: i32, height: i32, baseline: i32) {
        let allocation = gtk::Allocation::new(0, 0, width, height);
        self.notebook.size_allocate(&allocation, baseline)
    }
}

impl SWSGui {
    pub(super) fn append_new_audio_tab(&self) {
        self.notebook.append_page(
            &self.new_audio_tab.container,
            Some(&gtk::Label::builder().label("New Tab").visible(true).build()),
        );
    }

    pub(super) fn remove_new_audio_tab(&self) {
        self.notebook.detach_tab(&self.new_audio_tab.container);
    }

    fn new_tab_button_press(
        sws_gui: super::SWSGui,
        parent: impl IsA<gtk::Window>,
    ) {
        let audio_file_filter = gtk::FileFilter::new();
        audio_file_filter.add_mime_type("audio/*");

        let filters = gio::ListStore::new::<gtk::FileFilter>();
        filters.append(&audio_file_filter);

        let fd = gtk::FileDialog::builder()
            .title("Choose an Audio File")
            .filters(&filters)
            .build();
        fd.open(Some(&parent), gio::Cancellable::NONE, move |file_res| {
            match file_res {
                Ok(file) => {
                    let imp = sws_gui.imp();
                    imp.remove_new_audio_tab();
                    imp.append_audio_tab(file);
                }
                Err(err) => {
                    err.kind::<gtk::DialogError>().unwrap();
                }
            }
        })
    }

    pub(super) fn register_parent(&self, parent: gtk::ApplicationWindow) {
        let sws_gui = self.obj().clone();

        self.new_audio_tab
            .file_find_button
            .connect_clicked(glib::clone! {
                @weak parent, @weak sws_gui => @default-panic,
                move |_| Self::new_tab_button_press(sws_gui, parent)
            });

        parent.connect_close_request(glib::clone! {
            @weak sws_gui => @default-return glib::Propagation::Proceed, move |_| {
                sws_gui.imp().audio_tabs.borrow().iter().for_each(AudioTab::kill_audio);
                glib::Propagation::Proceed
            }
        });

        self.append_new_audio_tab();
    }

    fn append_audio_tab(&self, file: gio::File) {
        let basename = file.basename().unwrap();
        let label = gtk::Label::builder()
            .label(glib::GString::try_from(basename).unwrap())
            .build();
        let audio_tab = AudioTab::new(file);
        self.notebook.append_page(&audio_tab, Some(&label));
        self.audio_tabs.borrow_mut().push(audio_tab);
    }
}
