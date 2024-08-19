## Iced library to create custom browsers

[![Build](https://github.com/LegitCamper/rust-browser/actions/workflows/ci.yml/badge.svg)](https://github.com/LegitCamper/rust-browser/actions/workflows/ci.yml)
<img src="https://raw.githubusercontent.com/gist/hecrj/ad7ecd38f6e47ff3688a38c79fd108f0/raw/74384875ecbad02ae2a926425e9bcafd0695bade/color.svg" width=8%>

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
use iced::{Sandbox, Settings, Theme};
use iced_aw::BOOTSTRAP_FONT_BYTES;

use icy_browser::{widgets, BrowserWidget, Ultralight};

fn main() -> Result<(), iced::Error> {
    // This imports `icons` for widgets
    let bootstrap_font = BOOTSTRAP_FONT_BYTES.into();
    let settings = Settings {
        fonts: vec![bootstrap_font],
        ..Default::default()
    };
    Browser::run(settings)
}

struct Browser {
    widgets: BrowserWidget<Ultralight>,
}

#[derive(Debug, Clone)]
pub enum Message {
    BrowserWidget(widgets::Message),
}

impl Sandbox for Browser {
    type Message = Message;

    fn new() -> Self {
        let widgets = BrowserWidget::new_with_ultralight()
            .with_tab_bar()
            .with_nav_bar()
            .with_browsesr_view()
            .build();

        Self { widgets }
    }

    fn title(&self) -> String {
        String::from("Basic Browser")
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::BrowserWidget(msg) => {
                self.widgets.update(msg);
            }
        }
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {
        self.widgets.view().map(Message::BrowserWidget)
    }
}
```
