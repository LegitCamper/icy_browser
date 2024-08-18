// Simple browser with familiar browser widget and the ultralight(webkit) webengine as a backend

use iced::{Sandbox, Settings, Theme};
use iced_aw::BOOTSTRAP_FONT_BYTES;

use icy_browser::{browser_widgets, BrowserWidget, Ultralight};

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
    BrowserWidget(browser_widgets::Message),
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
        self.widgets.view().map(Message::BrowserWidget).into()
    }
}
