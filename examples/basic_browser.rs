// Simple browser with familiar browser widgets and the ultralight(webkit) webengine as a backend

use iced::{Settings, Task, Theme};
use icy_browser::{get_fonts, BasicBrowser, Message};

fn run() -> (BasicBrowser, Task<Message>) {
    (
        BasicBrowser::new_basic()
            .with_tab_bar()
            .with_nav_bar()
            .build(),
        Task::none(),
    )
}

fn main() -> iced::Result {
    let settings = Settings {
        fonts: get_fonts(),
        ..Default::default()
    };

    iced::application("Basic Browser", BasicBrowser::update, BasicBrowser::view)
        .subscription(BasicBrowser::subscription)
        .settings(settings)
        .theme(|_| Theme::Dark)
        .run_with(run)
}
