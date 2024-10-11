// Simple browser with familiar browser widgets and the ultralight(webkit) webengine as a backend

use iced::{Settings, Task, Theme};
use icy_browser::{get_fonts, Bookmark, IcyBrowser, Message, Ultralight};

fn run() -> (IcyBrowser<Ultralight>, Task<Message>) {
    (
        IcyBrowser::new()
            .with_tab_bar()
            .with_nav_bar()
            .bookmarks(&[Bookmark::new("https://www.rust-lang.org", "rust-lang.org")])
            .with_bookmark_bar()
            .build(),
        Task::none(),
    )
}

fn main() -> iced::Result {
    let settings = Settings {
        fonts: get_fonts(),
        ..Default::default()
    };

    iced::application("Basic Browser", IcyBrowser::update, IcyBrowser::view)
        .subscription(IcyBrowser::subscription)
        .settings(settings)
        .theme(|_| Theme::Dark)
        .run_with(run)
}
