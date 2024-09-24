// Simple keybaord driven browser using the ultralight(webkit) webengine as a backend

use iced::event::{self, Event};
use iced::Theme;
use iced::{Element, Settings, Subscription, Task};
use std::time::Duration;

use icy_browser::{
    get_fonts, widgets, BasicBrowser, BrowserWidget, KeyType, Message as WidgetMessage,
    ShortcutBuilder, ShortcutModifier,
};

fn main() -> iced::Result {
    let settings = Settings {
        fonts: get_fonts(),
        ..Default::default()
    };

    println!("Press Crtl - E to open to Command palatte");

    iced::application("Keyboard Driven Browser", Browser::update, Browser::view)
        .subscription(Browser::subscription)
        .settings(settings)
        .theme(|_| Theme::Dark)
        .run()
}

#[derive(Debug, Clone)]
pub enum Message {
    BrowserWidget(widgets::Message), // Passes messagees to Browser widgets
    Update,
    Event(Event),
}

struct Browser {
    widgets: BasicBrowser,
}

impl Default for Browser {
    fn default() -> Self {
        let shortcuts = ShortcutBuilder::new()
            .add_shortcut(
                WidgetMessage::ToggleOverlay,
                vec![
                    KeyType::Modifier(ShortcutModifier::Ctrl),
                    KeyType::Key(iced::keyboard::Key::Character("e".into())),
                ],
            )
            .build();
        let widgets = BrowserWidget::new_basic()
            .with_custom_shortcuts(shortcuts)
            .with_tab_bar()
            .build();

        Self { widgets }
    }
}

impl Browser {
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::BrowserWidget(msg) => self.widgets.update(msg).map(Message::BrowserWidget),
            Message::Update => self.widgets.force_update().map(Message::BrowserWidget),
            Message::Event(event) => self
                .widgets
                .update(widgets::Message::Event(Some(event)))
                .map(Message::BrowserWidget),
        }
    }

    fn view(&self) -> Element<Message> {
        self.widgets.view().map(Message::BrowserWidget)
    }

    fn subscription(&self) -> Subscription<Message> {
        Subscription::batch([
            iced::time::every(Duration::from_millis(10)).map(move |_| Message::Update),
            // This is needed for child widgets such as overlay to detect Key events
            event::listen().map(Message::Event),
        ])
    }
}
