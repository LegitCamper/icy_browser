// Simple browser with familiar browser widgets and the ultralight(webkit) webengine as a backend

use iced::{Settings, Task, Theme};
use icy_browser::{get_fonts, Browser, Message};

fn run() -> (Browser, Task<Message>) {
    (
        Browser::new_with_ultralight()
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

    iced::application("Basic Browser", Browser::update, Browser::view)
        .subscription(Browser::subscription)
        .settings(settings)
        .theme(|_| Theme::Dark)
        .run_with(run)
}
