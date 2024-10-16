## Iced library to create custom browsers
<img src="https://raw.githubusercontent.com/gist/hecrj/ad7ecd38f6e47ff3688a38c79fd108f0/raw/74384875ecbad02ae2a926425e9bcafd0695bade/color.svg" width=20%>

[![Build](https://github.com/LegitCamper/icy_browser/actions/workflows/ci.yml/badge.svg)](https://github.com/LegitCamper/icy_browser/actions/workflows/ci.yml)

### Browser Widgets
- [iced_webview](https://github.com/LegitCamper/iced_webview) 
> Currently only supports [Ultralight which has its own licence](https://ultralig.ht/pricing/) you should review 
- Navigation Bar
- Tab Bar
- Bookmark Bar

### Examples
#### basic_browser.rs
<img src="https://github.com/LegitCamper/icy_browser/blob/main/assets/basic_browser.png?raw=true" width=50%>

`cargo run --example basic_browser --features ultralight-resources`
``` Rust
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
```
