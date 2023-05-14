use std::thread;
use std::sync::mpsc;

use druid::{Widget, WindowDesc, AppLauncher, Data, Lens, WidgetExt};
use druid::widget::{Label, Flex, Button, TextBox};
use rustube::{ self };

const _SCREEN_SMALL: (f64, f64) = (480.0, 854.0);
const SCREEN_MEDIUM: (f64, f64) = (960.0, 540.0);
const _SCREEN_LARGE: (f64, f64) = (1366.0, 768.0);

#[derive(Clone, Data, Lens, Default, PartialEq, Eq)]
struct AppState {
    url: String,
    notification: String,
}

fn main() {
    let main_window = WindowDesc::new(ui_builder())
        .title("Manus YT Downloader")
        .window_size(SCREEN_MEDIUM);

    let default_state = AppState {
        url: String::new(),
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
            .padding(15.0)
        )
        .with_child(
            TextBox::<String>::new()
                .with_placeholder("Enter link here")
                .fix_width(500.0)
                .lens(AppState::url)
                .padding(15.0)
            )
        .with_child(
            Button::new("Download")
                .on_click(|_ctx, data: &mut AppState, _env| {
                    yt_download(data.url.clone())
                })
                .fix_width(150.0)
                .fix_height(40.0)
                .padding(15.0)
        )
        .with_child(
            Label::new(|data: &AppState, _env: &_| data.notification.clone())
                .with_text_size(21.0)
                .padding(15.0)
        )
        .with_flex_spacer(0.1)
}

fn yt_download(url: String) {
    thread::spawn(move || {
        match rustube::blocking::download_best_quality(url.as_str()) {
            Ok(_) => {
                println!("Downloaded video.")
            },
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    });
}