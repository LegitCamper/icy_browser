// Simple browser with familiar browser widget and the ultralight(webkit) webengine as a backend

use iced::Theme;
use iced::{Element, Settings, Subscription, Task};
use iced_aw::BOOTSTRAP_FONT_BYTES;
use smol_str::SmolStr;
use std::time::Duration;

use icy_browser::{widgets, BrowserWidget, KeyType, Shortcut, Ultralight};

fn main() -> iced::Result {
    // This imports `icons` for widgets
    let bootstrap_font = BOOTSTRAP_FONT_BYTES.into();
    let settings = Settings {
        fonts: vec![bootstrap_font],
        ..Default::default()
    };

    iced::application("Basic Browser Example", Browser::update, Browser::view)
        .subscription(Browser::subscription)
        .settings(settings)
        .theme(|_| Theme::Dark)
        .run()
}

#[derive(Debug, Clone)]
pub enum Message {
    BrowserWidget(widgets::Message),
    Update,
}

struct Browser {
    widgets: BrowserWidget<Ultralight>,
}

impl Default for Browser {
    fn default() -> Self {
        let shortcuts = vec![Shortcut::ShowOverlay(vec![KeyType::Key(
            iced::keyboard::Key::Character(SmolStr::from("e")),
        )])];
        let widgets = BrowserWidget::new_with_ultralight()
            .with_custom_shortcuts(shortcuts)
            .with_tab_bar()
            .with_nav_bar()
            .build();

        Self { widgets }
    }
}

impl Browser {
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::BrowserWidget(msg) => self.widgets.update(msg).map(Message::BrowserWidget),
            Message::Update => self.widgets.force_update().map(Message::BrowserWidget),
        }
    }

    fn view(&self) -> Element<Message> {
        self.widgets.view().map(Message::BrowserWidget)
    }

    fn subscription(&self) -> Subscription<Message> {
        iced::time::every(Duration::from_millis(10)).map(move |_| Message::Update)
    }
}
