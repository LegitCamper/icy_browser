// Simple keybaord driven browser using the ultralight(webkit) webengine as a backend

use iced::event::{self, Event};
use iced::Theme;
use iced::{Element, Settings, Subscription, Task};
use std::time::Duration;

use icy_browser::{
    get_fonts, widgets, Bookmark, IcyBrowser, KeyType, ShortcutBuilder, ShortcutModifier,
    Ultralight,
};

fn main() -> iced::Result {
    let settings = Settings {
        fonts: get_fonts(),
        ..Default::default()
    };

    println!("Press 'Crtl + E' to open to Command palatte");

    iced::application("Keyboard Driven Browser", Browser::update, Browser::view)
        .subscription(Browser::subscription)
        .settings(settings)
        .theme(|_| Theme::Dark)
        .run()
}

#[derive(Debug, Clone)]
pub enum Message {
    IcyBrowser(icy_browser::Message), // Passes messages to icy_browser
    Update,
    Event(Event),
}

struct Browser {
    icy_browser: IcyBrowser<Ultralight>,
}

impl Default for Browser {
    fn default() -> Self {
        let shortcuts = ShortcutBuilder::new()
            .add_shortcut(
                icy_browser::Message::ToggleOverlay,
                vec![
                    KeyType::Modifier(ShortcutModifier::Ctrl),
                    KeyType::Key(iced::keyboard::Key::Character("e".into())),
                ],
            )
            .build();
        let widgets = IcyBrowser::new()
            .with_custom_shortcuts(shortcuts)
            .with_tab_bar()
            .with_bookmark_bar(&[
                Bookmark::new("https://www.rust-lang.org", "rust-lang.org"),
                Bookmark::new(
                    "https://github.com/LegitCamper/icy_browser",
                    "icy_browser github",
                ),
                Bookmark::new("https://docs.rs/iced/latest/iced/", "iced docs"),
            ])
            .build();

        Self {
            icy_browser: widgets,
        }
    }
}

impl Browser {
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::IcyBrowser(msg) => self.icy_browser.update(msg).map(Message::IcyBrowser),
            Message::Update => self.icy_browser.force_update().map(Message::IcyBrowser),
            Message::Event(event) => self
                .icy_browser
                .update(widgets::Message::IcedEvent(Some(event)))
                .map(Message::IcyBrowser),
        }
    }

    fn view(&self) -> Element<Message> {
        self.icy_browser.view().map(Message::IcyBrowser)
    }

    fn subscription(&self) -> Subscription<Message> {
        Subscription::batch([
            iced::time::every(Duration::from_millis(10)).map(move |_| Message::Update),
            // This is needed for child widgets such as overlay to detect Key events
            event::listen().map(Message::Event),
        ])
    }
}
