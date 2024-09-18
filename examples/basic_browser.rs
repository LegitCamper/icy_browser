// Simple browser with familiar browser widget and the ultralight(webkit) webengine as a backend

use iced::{Element, Settings, Theme};
use iced_aw::BOOTSTRAP_FONT_BYTES;

use icy_browser::{widgets, BrowserWidget, Ultralight};

fn main() -> iced::Result {
    // This imports `icons` for widgets
    let bootstrap_font = BOOTSTRAP_FONT_BYTES.into();
    let settings = Settings {
        fonts: vec![bootstrap_font],
        ..Default::default()
    };

    iced::application("Basic Browser Example", Browser::update, Browser::view)
        .settings(settings)
        .theme(|_| Theme::Dark)
        .run()
}

#[derive(Debug, Clone)]
pub enum Message {
    BrowserWidget(widgets::Message),
}

struct Browser {
    widgets: BrowserWidget<Ultralight>,
}

impl Browser {
    fn update(&mut self, message: Message) {
        match message {
            Message::BrowserWidget(msg) => {
                self.widgets.update(msg);
            }
        }
    }

    fn view(&self) -> Element<Message> {
        self.widgets.view().map(Message::BrowserWidget)
    }
}

impl Default for Browser {
    fn default() -> Self {
        let widgets = BrowserWidget::new_with_ultralight()
            .with_tab_bar()
            .with_nav_bar()
            .with_browsesr_view()
            .build();

        Self { widgets }
    }
}
