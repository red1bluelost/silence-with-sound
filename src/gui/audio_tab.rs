use crate::source::{AudioConfigBuilder, BoxSource};

use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{gio, glib};
use rand::distributions::Uniform;
use rand::Rng;
use rodio::source::Buffered;
use rodio::{OutputStream, OutputStreamHandle, Sink, Source};

use std::cell::Cell;
use std::rc::Rc;
use std::time::Duration;

const VOLUME_DEFAULT: f64 = 1.0;
const REVERB_DEFAULT: f64 = 0.0;

pub struct AudioTab {
    pub(super) curr_src_id: Rc<Cell<Option<glib::SourceId>>>,
    pub(super) container: gtk::Widget,
    pub(super) volume_scale: gtk::Scale,
    pub(super) reverb_scale: gtk::Scale,
    pub(super) apply_button: gtk::Button,
}

impl Default for AudioTab {
    fn default() -> Self {
        let grid = gtk::Grid::builder()
            .halign(gtk::Align::Center)
            .column_homogeneous(true)
            .column_spacing(2)
            .row_homogeneous(false)
            .row_spacing(1)
            .build();

        let volume_scale = build_scale_box(&grid, 0, "Volume", 0.0, 10.0);
        let reverb_scale = build_scale_box(&grid, 1, "Reverb", 0.0, 10.0);

        let apply_button = gtk::Button::builder()
            .label("Apply")
            .hexpand(true)
            .vexpand(true)
            .build();
        grid.attach(&apply_button, 2, 3, 1, 1);

        let audio_tab = Self {
            curr_src_id: Default::default(),
            container: grid.into(),
            volume_scale,
            reverb_scale,
            apply_button,
        };
        audio_tab.reset_to_defaults();
        audio_tab
    }
}

#[glib::object_subclass]
impl ObjectSubclass for AudioTab {
    const NAME: &'static str = "R1BLAudioTab";
    type Type = super::AudioTab;
    type ParentType = gtk::Widget;
}

impl ObjectImpl for AudioTab {
    fn constructed(&self) {
        let obj_self = self.obj().clone();
        self.container.set_parent(&obj_self);
    }
    fn dispose(&self) {
        self.kill_audio();
        self.container.unparent();
    }
}

impl WidgetImpl for AudioTab {
    fn measure(
        &self,
        orientation: gtk::Orientation,
        for_size: i32,
    ) -> (i32, i32, i32, i32) {
        self.container.measure(orientation, for_size)
    }

    fn size_allocate(&self, width: i32, height: i32, baseline: i32) {
        let allocation = gtk::Allocation::new(0, 0, width, height);
        self.container.size_allocate(&allocation, baseline)
    }
}

impl AudioTab {
    pub(super) fn reset_to_defaults(&self) {
        self.volume_scale.set_value(VOLUME_DEFAULT);
        self.reverb_scale.set_value(REVERB_DEFAULT);
    }

    pub(super) fn register_file(&self, file: gio::File) {
        let sws_gui = self.obj().clone();
        self.apply_button.connect_clicked(glib::clone! {
            @weak sws_gui, @strong file => @default-panic,
            move |_| sws_gui.imp().start_audio(file.clone())
        });
    }

    pub(super) fn kill_audio(&self) {
        if let Some(src_id) = self.curr_src_id.take() {
            src_id.remove();
        }
    }

    fn start_audio(&self, file: gio::File) {
        self.kill_audio();

        let path = file.path().unwrap();
        let duration = Duration::from_micros(
            ffmpeg::format::input(&path)
                .unwrap()
                .duration()
                .try_into()
                .unwrap(),
        );
        debug_assert_ne!(duration, Duration::ZERO);

        let source = AudioConfigBuilder::default()
            .file_name(path.into_boxed_path())
            .volume(self.volume_scale.value() as f32)
            .build()
            .unwrap()
            .generate()
            .unwrap();

        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();

        let audio_playing = Box::new(AudioPlaying {
            _stream,
            _stream_handle: stream_handle,
            sink,
            duration,
        });

        Self::run_audio(
            audio_playing,
            Rc::clone(&self.curr_src_id),
            source.buffered(),
        );
    }

    fn run_audio(
        audio_playing: Box<AudioPlaying>,
        id_ref: Rc<Cell<Option<glib::SourceId>>>,
        source: Buffered<BoxSource<i16>>,
    ) {
        let range =
            Uniform::new(Duration::from_secs(0), Duration::from_secs(5));
        let wait = rand::thread_rng().sample(range);

        let id_clone = Rc::clone(&id_ref);
        let src_id = glib::timeout_add_local_once(
            wait + audio_playing.duration,
            move || {
                audio_playing.sink.append(source.clone());
                audio_playing.sink.play();

                Self::run_audio(audio_playing, id_clone, source);
            },
        );
        id_ref.set(Some(src_id));
    }
}

struct AudioPlaying {
    _stream: OutputStream,
    _stream_handle: OutputStreamHandle,
    sink: Sink,
    duration: Duration,
}

fn build_scale_box(
    grid: &gtk::Grid,
    column: i32,
    name: &str,
    lower: f64,
    upper: f64,
) -> gtk::Scale {
    let adjustment =
        gtk::Adjustment::builder().lower(lower).upper(upper).build();

    let scale = gtk::Scale::builder()
        .name(name)
        .orientation(gtk::Orientation::Vertical)
        .adjustment(&adjustment)
        .draw_value(true)
        .value_pos(gtk::PositionType::Bottom)
        .inverted(true)
        .has_tooltip(true)
        .height_request(100)
        .hexpand(true)
        .vexpand(true)
        .build();

    let label = gtk::Label::builder()
        .label(name)
        .hexpand(true)
        .vexpand(true)
        .build();

    grid.attach(&frame_it(&label), column, 0, 1, 1);
    grid.attach(&frame_it(&scale), column, 1, 1, 2);

    scale
}

fn frame_it(widget: &impl IsA<gtk::Widget>) -> impl IsA<gtk::Widget> {
    gtk::Frame::builder().child(widget).build()
}
