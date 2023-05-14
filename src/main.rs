use std::thread;
//  use std::sync::mpsc;

use druid::widget::{Button, Checkbox, Flex, Label, TextBox};
use druid::{AppLauncher, Data, Lens, Widget, WidgetExt, WindowDesc};
use rustube::{self};

const _SCREEN_SMALL: (f64, f64) = (480.0, 854.0);
const SCREEN_MEDIUM: (f64, f64) = (960.0, 540.0);
const _SCREEN_LARGE: (f64, f64) = (1366.0, 768.0);

#[derive(Clone, Data, Lens, Default, PartialEq, Eq)]
struct AppState {
    url: String,
    audio_only: bool,
    notification: String,
}

fn main() {
    let main_window = WindowDesc::new(ui_builder())
        .title("Manus YT Downloader")
        .window_size(SCREEN_MEDIUM);

    let default_state = AppState {
        url: String::new(),
        audio_only: false,
        notification: String::new(),
    };

    // let (tx, rx) = mpsc::channel();

    AppLauncher::with_window(main_window)
        .launch(default_state)
        .expect("Failed to launch application")
}

fn ui_builder() -> impl Widget<AppState> {
    Flex::column()
        .with_flex_spacer(0.1)
        .with_child(
            Label::new("Insert Video URL here:")
                .with_text_size(21.0)
                .padding(15.0),
        )
        .with_child(
            TextBox::<String>::new()
                .with_placeholder("Enter link here")
                .fix_width(500.0)
                .lens(AppState::url)
                .padding(15.0),
        )
        .with_child(
            Flex::row()
                .with_child(
                    Button::new("Download")
                        .on_click(|_ctx, data: &mut AppState, _env| {
                            yt_download(data.url.clone(), data.audio_only.clone())
                        })
                        .fix_width(150.0)
                        .fix_height(40.0)
                        .padding(15.0),
                )
                .with_child(
                    Checkbox::new("audio only")
                        .lens(AppState::audio_only)
                        .padding(15.0),
                ),
        )
        .with_child(
            Label::new(|data: &AppState, _env: &_| data.notification.clone())
                .with_text_size(21.0)
                .padding(15.0),
        )
        .with_flex_spacer(0.1)
}

// use best_audio to choose stream

fn yt_download(url: String, audio_only: bool) {
    thread::spawn(move || {
        println!("download_thread started");
        let id = rustube::Id::from_string(url).unwrap();
        println!("id: {:?}", id);
        let video = rustube::blocking::Video::from_id(id).unwrap();
        println!("got video info");
        match audio_only {
            // download only best quali audio
            true => {
                println!("downloading audio");
                let path_to_audio = video.best_audio().unwrap().blocking_download().unwrap();
                println!("Downloaded audio to: {:?}", path_to_audio)
            }
            // download best quali video + audio
            false => {
                println!("downloading video");
                let path_to_video = video.best_quality().unwrap().blocking_download().unwrap();
                println!("Downloaded video/audio to: {:?}", path_to_video)
            }
        }
    });
}
