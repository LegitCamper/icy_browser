## Iced library to create custom browsers

[![Build](https://github.com/LegitCamper/icy_browser/actions/workflows/ci.yml/badge.svg)](https://github.com/LegitCamper/icy_browser/actions/workflows/ci.yml)
<img src="https://raw.githubusercontent.com/gist/hecrj/ad7ecd38f6e47ff3688a38c79fd108f0/raw/74384875ecbad02ae2a926425e9bcafd0695bade/color.svg" width=10%>

### Supported Platforms
| Platform | Support               |
| Windows  | <span>&#10003;</span> |
| Linux    | <span>&#10003;</span> |


### Supported Browser Engines
| Browser Engine | Support      |
| ----------------- | --------- |
| WebKit/Ultralight | <span>&#10003;</span> |
| Chromium/CEF      | X Planned |


### Browser Widgets
- Navigation Bar
- Tab Bar
- Browser View

### Examples
#### basic_browser.rs
<img src="https://github.com/LegitCamper/rust-browser/blob/main/assets/basic_browser.png" width=50%>

``` Rust
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
```
